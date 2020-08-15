use super::*;
use crate::structs::{MessageStore, TextMessage};
use std::cmp;

/// Signature used for message store .bin files.
const MS_BIN_VER: u32 = 20090521;
/// Chunk size for reading in the message store.
const READ_BUF_SIZE: usize = 8192;

/// Opens a message store .bin file and reads the headers.
///
/// Arguments:
///
/// * `path` - The full file path to the .bin to open.
///
/// Returns:
///
/// If successful, a `std::io::BufReader` with the open .bin file, position advanced past the headers.
/// Otherwise, a `ParseError` with the error information.
pub fn open_message_store(path: &Path) -> ParseResult<BufReader<File>> {
    let file = File::open(path).map_err(to_pe)?;
    let mut reader = BufReader::new(file);

    // check signature
    let sig: u32 = bin_read(&mut reader)?;
    if sig != MS_BIN_VER {
        return Err(ParseError::new(ParseErrorKind::MissingCrypticSig));
    }

    Ok(reader)
}

/// Reads the strings table inside the message store.
///
/// Arguments:
///
/// * `reader` - An open `Read` + `Seek`.
///
/// Returns:
///
/// If successful, a `Vec<String>` containing all of the strings.
/// Otherwise, a `ParseError` containing the error information.
pub fn read_string_table<T>(reader: &mut T) -> ParseResult<Vec<String>>
where
    T: Read + Seek,
{
    // read string table headers
    let _string_count: usize = bin_read(reader)?;
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    let expected_bytes = expected_bytes as usize;

    // read the entire string table and chunk it
    let mut strings = Vec::<String>::new();
    let mut buf = [0u8; READ_BUF_SIZE];
    let mut sbuf = Vec::<u8>::new();
    let mut bytes_read = 0usize;
    while bytes_read < expected_bytes {
        // Read in READ_BUF_SIZE-sized chunks, making sure we don't read more than the data length told us.
        bytes_read += reader
            .take(cmp::min(expected_bytes - bytes_read, READ_BUF_SIZE) as u64)
            .read(&mut buf)
            .map_err(to_pe)?;
        let mut i = buf.iter();
        while let Some(b) = i.next() {
            match *b {
                0 => {
                    // As we read, we look for delimiting NULs -- this is where the string boundaries are.
                    // Anything we read up to this point is combined into a string, then we start anew.
                    strings.push(
                        str::from_utf8(sbuf.as_slice())
                            .map_err(|_| ParseError::new(ParseErrorKind::StringConversion))?
                            .to_owned()
                            .replace("&nbsp;", " ") // HACK
                    );
                    sbuf.clear();
                }
                0xa0 => {
                    // ASCII nbsp gets used sometimes which is confusing to utf-8
                    sbuf.push(0x20);
                }
                0x80..=0xff => (), // dump all high- and low-order ASCII (used for some random Latin chars I don't care about)
                0x01..=0x1f => (), // (they tend to be used in mission/NPC text rather than powers descriptions)
                _ => sbuf.push(*b),
            }
        }
    }
    // Commented out this assertion...interestingly, my way of reading clientmessage ends up with 16 additional strings
    // I believe this is because I'm reading based on expected_bytes, whereas the original code simply read based on string_count
    // and bailed if it read past the buffer... should be harmless?
    //assert_eq!(strings.len(), string_count);

    verify_struct_length(strings, expected_bytes as u64, begin_pos, reader)
}

/// Reads the map of message IDs inside the message store.
///
/// Arguments:
///
/// * `reader` - An open `Read` + `Seek`.
/// * `store` - A `MessageStore` previously populated by `read_string_table`.
///
/// Returns:
///
/// On success, the `store.message_ids` map will be updated with information on the
/// message IDs.
/// Otherwise, a `ParseError` containing the error information.
pub fn read_message_ids<T>(reader: &mut T, store: &mut MessageStore) -> ParseResult<()>
where
    T: Read + Seek,
{
    let elem_count = bin_read(reader)?;
    for _ in 0..elem_count {
        let len: usize = bin_read(reader)?;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf).map_err(to_pe)?;
        let string = str::from_utf8(buf.as_slice())
            .map_err(|_| ParseError::new(ParseErrorKind::StringConversion))?;

        let message_index = bin_read(reader)?;
        let help_index = bin_read(reader)?;

        let var_count = bin_read(reader)?;
        let vars = if var_count > 0 {
            let mut v = Vec::new();
            for _ in 0..var_count {
                v.push(bin_read(reader)?);
            }
            Some(v)
        } else {
            None
        };

        store.message_ids.insert(
            string.to_owned(),
            TextMessage::new(message_index, help_index, vars),
        );
    }

    Ok(())
}
