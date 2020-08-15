use super::*;
use crate::structs::{Keyed, PowerCategory};
use std::rc::Rc;

/// Reads all of the power categories in the current .bin file.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for power categories
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a map containing zero or more `PowerCategory` structs.
/// Otherwise, a `ParseError` with the error information.
pub fn serialized_read_power_categories<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Keyed<PowerCategory>>
where
    T: Read + Seek,
{
    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    let mut powercats = Keyed::<_>::new();

    // first read the length of the TOK_EARRAY ParsePowerCategory[]
    let pcat_size: usize = bin_read(reader)?;
    for _ in 0..pcat_size {
        let powercat = read_power_category(reader, strings, messages)?;
        if let Some(powercat_name) = &powercat.pch_name {
            powercats.insert(powercat_name.clone(), Rc::new(powercat));
        }
    }

    verify_struct_length(powercats, expected_bytes, begin_pos, reader)
}

/// Reads a PowerCategory struct from a .bin file.
/// Refer to Common/entity/powers_load.c TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for power categories
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `PowerCategory`.
/// Otherwise, a `ParseError` with the error information.
fn read_power_category<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<PowerCategory>
where
    T: Read + Seek,
{
    let mut powercat = PowerCategory::new();

    macro_rules! pcat_string {
        ($($field:ident), +) => {
            $( powercat.$field = read_pool_string(reader, strings, messages)?; )+
        };
    }

    // TOK_STRUCT data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    pcat_string!(pch_source_file);

    powercat.pch_name = read_name_key(reader, strings)?;
    if powercat.pch_name.is_none() {
        return Err(ParseError::new(ParseErrorKind::MissingNameKey));
    }
    pcat_string!(pch_display_name, pch_display_help, pch_display_short_help);

    // power set strings EARRAY TOK_STRING
    bin_read_arr_fn(
        &mut powercat.ppch_power_set_names,
        |re| {
            if let Ok(Some(power_name)) = read_name_key(re, strings) {
                Ok(power_name)
            } else {
                Err(ParseError::new(ParseErrorKind::MissingNameKey))
            }
        },
        reader,
    )?;

    verify_struct_length(powercat, expected_bytes, begin_pos, reader)
}
