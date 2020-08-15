mod archetypes;
mod attribs;
mod boost_sets;
pub mod messagestore;
mod powercats;
mod powers;
mod powersets;
mod villains;

use crate::structs::{IntoMessage, MessageStore, NameKey, StringPool, Vec3, RGBA};
pub use archetypes::*;
pub use attribs::*;
pub use boost_sets::*;
pub use powercats::*;
pub use powers::*;
pub use powersets::*;
use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem::size_of;
use std::path::Path;
use std::str;
pub use villains::*;

const CRYPTIC_SIG: &'static [u8] = "CrypticS".as_bytes();
const PARSE_SIG: &'static str = "Parse7";
const MAX_FILETYPE_LEN: usize = 4096;

/// The kind of error reported by `ParseError`.
#[derive(Clone, Copy)]
pub enum ParseErrorKind {
    /// An I/O read error occurred, check the `ParseError.io_error` field for more info.
    ReadError,
    /// An error occurred attempting to convert a stored C string into a Rust string.
    StringConversion,
    /// The header ("Cryptic signature") is missing from the file. Probably not a .bin file.
    MissingCrypticSig,
    /// The .bin file doesn't contain the expected file type. Probably trying to load an incorrect .bin file.
    WrongFileType,
    /// The previously read segment expected `expected_bytes`, but we only read `read_bytes`.
    SizeMismatch {
        expected_bytes: u64,
        read_bytes: u64,
    },
    /// The currently read object does not have a name key, which shouldn't be possible.
    MissingNameKey,
}

/// Represents an error the occurred while parsing a .bin file.
pub struct ParseError {
    /// The kind of error that occured.
    kind: ParseErrorKind,
    /// If `kind` is `ParseErrorKind::ReadError`, this will contain the `io::Error` that caused it.
    io_error: Option<io::Error>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse .bin file.")
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl ParseError {
    /// Creates a new `ParseError` set to the specified `kind`.
    fn new(kind: ParseErrorKind) -> Self {
        ParseError {
            kind,
            io_error: None,
        }
    }

    /// Creates a new `ParseError` based on an `io::Error` that caused it.
    fn from_io_error(error: io::Error) -> Self {
        let mut e = ParseError::new(ParseErrorKind::ReadError);
        e.io_error = Some(error);
        e
    }

    /// If this `ParseError` is of type `ParseErrorKind::ReadError`, then this
    /// will return the I/O error that caused it.
    ///
    /// # Returns
    ///
    /// A reference to an `io::Error`.
    pub fn get_io_error_ref(&self) -> Option<&io::Error> {
        self.io_error.as_ref()
    }

    /// Gets the kind of error that occurred.
    ///
    /// # Returns
    ///
    /// A `ParseErrorKind` value describing the error.
    pub fn kind(&self) -> ParseErrorKind {
        self.kind
    }
}

impl std::error::Error for ParseError {}

/// Convenience wrapper for `Result<T, ParseError>`.
pub type ParseResult<T> = Result<T, ParseError>;

/// Opens a .bin file and verifies the headers.
///
/// # Arguments
///
/// * `path` - The full file path to the .bin file to open.
///
/// # Returns
///
/// If successful, a `std::io::BufReader` with the open .bin file, position advanced past the headers.
/// Otherwise, a `ParseError` with the error information.
pub fn open_serialized(path: &Path) -> ParseResult<BufReader<File>> {
    let file = File::open(path).map_err(to_pe)?;
    let mut reader = BufReader::new(file);

    // check signature
    let mut readsig = [0; CRYPTIC_SIG.len()];
    reader.read_exact(&mut readsig).map_err(to_pe)?;
    if &readsig != CRYPTIC_SIG {
        return Err(ParseError::new(ParseErrorKind::MissingCrypticSig));
    }
    // note: the "build" is actually a CRC the client/server uses to make sure the bins match the version of the client, we just ignore it here
    let _build: u32 = bin_read(&mut reader)?;
    let filetype = read_pascal_string(&mut reader)?;
    debug_assert!(filetype.len() <= MAX_FILETYPE_LEN, "File type is too long");
    if filetype != PARSE_SIG {
        return Err(ParseError::new(ParseErrorKind::WrongFileType));
    }

    Ok(reader)
}

/// Reads the string pool for the current .bin file.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
///
/// # Returns:
///
/// If successful, a `StringPool` containing all of the strings.
/// Otherwise, a `ParseError` containing the error information.
pub fn serialized_read_string_pool<T>(reader: &mut T) -> ParseResult<StringPool>
where
    T: Read + Seek,
{
    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    let padding = (4 - (expected_bytes % 4)) % 4;

    // read the entire string pool
    let mut pool = vec![0u8; expected_bytes as usize];
    reader.read_exact(pool.as_mut_slice()).map_err(to_pe)?;

    // bleed off the padding
    let _ = reader
        .seek(SeekFrom::Current(padding as i64))
        .map_err(to_pe)?;

    verify_struct_length(
        StringPool::new(pool),
        expected_bytes + padding,
        begin_pos,
        reader,
    )
}

/// Helper function to convert `io::Error` into `ParseError`.
fn to_pe(err: io::Error) -> ParseError {
    ParseError::from_io_error(err)
}

/// Reads the current file position of `reader`.
fn stream_pos<T>(reader: &mut T) -> ParseResult<u64>
where
    T: Seek,
{
    reader.seek(SeekFrom::Current(0)).map_err(to_pe)
}

/// Used by the `bin_read` family of functions to get a specific implementation
/// based on the data type of the desired return value.
/// TODO: Revisit this pattern once template specialization is stable.
/// https://github.com/rust-lang/rust/issues/31844
trait BinReadable: Sized {
    fn read_value<R>(reader: &mut R) -> ParseResult<Self>
    where
        R: Read;
}

/// Reads a value from the stream. This attempts to parse a value from the .bin file
/// and map it to the value of the field it's being read into. Check the different
/// implementations of `BinReadable` in this module for the specifics of how each
/// data type is represented.
///
/// # Arguments:
/// * reader - An open `Read`.
///
/// # Returns:
/// A value of type `T` if successful, otherwise a `ParseError`. The type of the
/// return `T` must implement `BinReadable`.
fn bin_read<T, R>(reader: &mut R) -> ParseResult<T>
where
    T: BinReadable,
    R: Read,
{
    T::read_value(reader)
}

/// Reads an array of values from the stream. This is a convenience function which calls
/// `bin_read` over a series of values and places them into `target`. See `bin_read` for
/// more info.
///
/// # Arguments:
/// * target - A vector of type `T`, which must implement `BinReadable`.
/// * reader - An open `Read`.
///
/// # Returns:
/// Nothing if successful, otherwise a `ParseError`. On return, `target` will be updated with any
/// new values found.
fn bin_read_arr<T, R>(target: &mut Vec<T>, reader: &mut R) -> ParseResult<()>
where
    T: BinReadable,
    R: Read,
{
    let size: u32 = bin_read(reader)?;
    for _ in 0..size {
        target.push(bin_read::<T, R>(reader)?);
    }
    Ok(())
}

/// Reads an array of values from the stream. This is a convenience function which mirrors
/// the functionality of `bin_read_arr` but allows for arbitrary transformation of the values
/// by calling `func` for each rather than directly calling `bin_read`.
///
/// # Arguments:
/// * target - A vector of type `T`, which must implement `BinReadable`.
/// * func - A function that is called to transform each value before being added to `target`.
/// * reader - An open `Read`.
///
/// # Returns:
/// Nothing if successful, otherwise a `ParseError`. On return, `target` will be updated with any
/// new values found.
fn bin_read_arr_fn<T, F, R>(target: &mut Vec<T>, func: F, reader: &mut R) -> ParseResult<()>
where
    F: Fn(&mut R) -> ParseResult<T>,
    R: Read,
{
    let size: u32 = bin_read(reader)?;
    for _ in 0..size {
        target.push(func(reader)?);
    }
    Ok(())
}

/// Reads a value from the stream. This attempts to parse a value from the .bin file
/// and map it to the value of the field it's being read into. This is a specialized
/// version of the `bin_read` function that tries to convert `u32` values into an
/// enumeration that supports direct conversion (via `TryFrom<u32>`).
///
/// Technically this function can work with any type that implements both `Default`
/// and `TryFrom<u32>`.
///
/// # Arguments:
/// * reader - An open `Read`.
///
/// # Returns:
/// A value of type `T` if successful, otherwise a `ParseError`. If the value cannot
/// be converted, it will silently return a default value for `T`.
fn bin_read_enum<T, R>(reader: &mut R) -> ParseResult<T>
where
    T: Default + TryFrom<u32>,
    R: Read,
{
    if let Ok(val) = T::try_from(bin_read::<u32, _>(reader)?) {
        Ok(val)
    } else {
        Ok(T::default())
    }
}

impl BinReadable for bool {
    /// Reads a Boolean value from the stream.
    fn read_value<R>(reader: &mut R) -> ParseResult<bool>
    where
        R: Read,
    {
        match bin_read(reader)? {
            0u32 => Ok(false),
            _ => Ok(true),
        }
    }
}

impl BinReadable for f32 {
    /// Reads a 32-bit floating point from the stream.
    fn read_value<R>(reader: &mut R) -> ParseResult<f32>
    where
        R: Read,
    {
        let mut buf = [0; size_of::<f32>()];
        reader.read_exact(&mut buf).map_err(to_pe)?;
        Ok(f32::from_le_bytes(buf))
    }
}

impl BinReadable for i32 {
    /// Reads a signed integer from the stream.
    fn read_value<R>(reader: &mut R) -> ParseResult<i32>
    where
        R: Read,
    {
        let mut buf = [0; size_of::<i32>()];
        reader.read_exact(&mut buf).map_err(to_pe)?;
        Ok(i32::from_le_bytes(buf))
    }
}

impl BinReadable for u16 {
    /// Reads a short unsigned integer from the stream.
    fn read_value<R>(reader: &mut R) -> ParseResult<u16>
    where
        R: Read,
    {
        let mut buf = [0; size_of::<u16>()];
        reader.read_exact(&mut buf).map_err(to_pe)?;
        Ok(u16::from_le_bytes(buf))
    }
}

impl BinReadable for u32 {
    /// Reads an unsigned integer from the stream.
    fn read_value<R>(reader: &mut R) -> ParseResult<u32>
    where
        R: Read,
    {
        let mut buf = [0; size_of::<u32>()];
        reader.read_exact(&mut buf).map_err(to_pe)?;
        Ok(u32::from_le_bytes(buf))
    }
}

impl BinReadable for usize {
    /// Reads a `usize` from the stream (converts from `u32`).
    fn read_value<R>(reader: &mut R) -> ParseResult<usize>
    where
        R: Read,
    {
        Ok(u32::read_value(reader)? as usize)
    }
}

impl BinReadable for RGBA {
    /// Reads an RGBA (fixed array of 4 u8s) value from the stream.
    fn read_value<R>(reader: &mut R) -> ParseResult<RGBA>
    where
        R: Read,
    {
        let r = bin_read::<u32, _>(reader)? as u8;
        let g = bin_read::<u32, _>(reader)? as u8;
        let b = bin_read::<u32, _>(reader)? as u8;
        let a = bin_read::<u32, _>(reader)? as u8;

        Ok(RGBA::new(r, g, b, a))
    }
}

impl BinReadable for Vec3 {
    /// Reads a Vec3 (fixed array of 3 f32s) value from the stream.
    fn read_value<R>(reader: &mut R) -> ParseResult<Vec3>
    where
        R: Read,
    {
        let x = bin_read(reader)?;
        let y = bin_read(reader)?;
        let z = bin_read(reader)?;
        Ok(Vec3 { x, y, z })
    }
}

/// Reads a Pascal-style string at the current position in `reader`. Some strings inside .bin files are actually
/// C strings (u8 char array) that are stored in "Pascal" style, that is they start with the
/// length in a u16 instead of being NUL-terminated.
///
/// The original code does some things where if a string is too long it will advance the file but not
/// return a valid value, probably trying to avoid buffer overruns. Rust is a bit safer here so I
/// omitted any of that logic.
///
/// Ref: `libs/UtilitiesLib/utils/serialize.c`
fn read_pascal_string<T>(reader: &mut T) -> ParseResult<String>
where
    T: Read,
{
    let strlen: u16 = bin_read(reader)?;
    if strlen > 0 {
        let mut buf = vec![0u8; strlen as usize];
        reader.read_exact(&mut buf).map_err(to_pe)?;
        Ok(str::from_utf8(&buf[..])
            .map_err(|_| ParseError::new(ParseErrorKind::StringConversion))?
            .to_owned())
    } else {
        Ok(String::from(""))
    }
}

/// Reads a Pascal-style string at the current position in `reader` (see `read_pascal_string` for details).
/// After reading, `reader` is advanced a number of bytes to keep to a 4-byte alignment, based on the length
/// of the returned string.
fn read_pascal_string_with_padding<T>(reader: &mut T) -> ParseResult<String>
where
    T: Read + Seek,
{
    let return_str = read_pascal_string(reader)?;
    let padding: usize = (4 - (return_str.len() + size_of::<u16>()) % 4) % 4;
    reader
        .seek(SeekFrom::Current(padding as i64))
        .map_err(to_pe)?;
    Ok(return_str)
}

/// Reads an offset into the `StringPool` from the stream and then performs a lookup to convert
/// it into the desired string. This is specifically used with `NameKey` so it does not refer to
/// the message store, as they are not UI strings.
fn read_name_key<T>(reader: &mut T, strings: &StringPool) -> ParseResult<Option<NameKey>>
where
    T: Read,
{
    let offset: usize = bin_read(reader)?;
    if offset > 0 {
        if let Some(s) = strings.get_string(offset) {
            return Ok(Some(NameKey::new(s)));
        }
    }
    Ok(None)
}

/// Reads multiple offsets into the `StringPool` from the stream and then performs lookups to
/// convert them into the desired strings. This is specifically used with `NameKey` so it does not refer to
/// the message store, as they are not UI strings.
fn read_name_key_arr<T>(
    target: &mut Vec<NameKey>,
    reader: &mut T,
    strings: &StringPool,
) -> ParseResult<()>
where
    T: Read,
{
    let size = bin_read(reader)?;
    for _ in 0..size {
        if let Some(s) = read_name_key(reader, strings)? {
            target.push(s);
        }
    }
    Ok(())
}

/// Reads an offset into the `StringPool` from the stream and then performs a lookup to convert
/// it into the desired string. Will also check `messages` to see if this a key into the message
/// store.
fn read_pool_string<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Option<String>>
where
    T: Read,
{
    let offset: usize = bin_read(reader)?;
    if offset > 0 {
        Ok(strings.get_string(offset).into_message(messages))
    } else {
        Ok(None)
    }
}

/// Reads multiple offsets into the `StringPool` from the stream and then performs lookups to
/// convert them into the desired strings. Will also check `messages` to see if they are keys
/// into the message store.
fn read_pool_string_arr<T>(
    target: &mut Vec<String>,
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<()>
where
    T: Read,
{
    let size = bin_read(reader)?;
    for _ in 0..size {
        if let Some(s) = read_pool_string(reader, strings, messages)? {
            target.push(s);
        }
    }
    Ok(())
}

/// Reads the data length of the current struct and returns it (tuple 0)
/// along with the current position in the file (tuple 1). Used in conjunction
/// with `verify_struct_length` to make sure we read the correct number of bytes.
fn read_struct_length<T>(reader: &mut T) -> ParseResult<(u64, u64)>
where
    T: Read + Seek,
{
    let expected_bytes = bin_read::<u32, _>(reader)? as u64;
    let cur_pos = stream_pos(reader)?;
    Ok((expected_bytes, cur_pos))
}

/// Verifies that we read the correct number of bytes while parsing the current
/// struct.
///
/// # Arguments
///
/// * `return_value` - This is the object to return to the caller of the current function if verification passes.
/// * `expected_bytes` - The number of bytes we expected to read (from `read_struct_length`).
/// * `begin_pos` - The offset where we began reading the current struct (from `read_struct_length`).
/// * `reader` - A `BufReader<File>` open to the current .bin file.
///
/// # Returns
///
/// A `ParseResult<T>`. If the struct is verified, this will be `return_value`.
/// Otherwise it will return a `ParseError` of kind `ParseErrorKind::SizeMismatch`.
fn verify_struct_length<T, R>(
    return_value: T,
    expected_bytes: u64,
    begin_pos: u64,
    reader: &mut R,
) -> ParseResult<T>
where
    R: Read + Seek,
{
    let read_bytes = stream_pos(reader)? - begin_pos;
    if expected_bytes != read_bytes {
        Err(ParseError::new(ParseErrorKind::SizeMismatch {
            expected_bytes,
            read_bytes,
        }))
    } else {
        Ok(return_value)
    }
}
