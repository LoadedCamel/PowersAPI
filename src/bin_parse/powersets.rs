use super::*;
use crate::structs::{BasePowerSet, Keyed};
use std::rc::Rc;

/// Reads all of the power sets in the current .bin file.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for power sets
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a map containing zero or more `BasePowerSet` structs.
/// Otherwise, a `ParseError` with the error information.
pub fn serialized_read_powersets<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Keyed<BasePowerSet>>
where
    T: Read + Seek,
{
    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    // first read the length of the TOK_EARRAY ParseBasePowerSet[]
    let pbps_size: usize = bin_read(reader)?;
    let mut powersets = Keyed::<_>::new();
    for _ in 0..pbps_size {
        let powerset = read_base_powerset(reader, strings, messages)?;
        if let Some(powerset_name) = &powerset.pch_full_name {
            powersets.insert(powerset_name.clone(), Rc::new(powerset));
        }
    }

    verify_struct_length(powersets, expected_bytes, begin_pos, reader)
}

/// Reads a `BasePowerSet` struct from a .bin file.
/// Refer to Common/entity/powers_load.c TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for power sets
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `BasePowerSet`.
/// Otherwise, a `ParseError` with the error information.
fn read_base_powerset<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<BasePowerSet>
where
    T: Read + Seek,
{
    let mut powerset = BasePowerSet::new();

    macro_rules! pset_string {
        ($($field:ident),+) => {
            $( powerset.$field = read_pool_string(reader, strings, messages)?; )+
        };
    }

    macro_rules! pset_string_arr {
        ($field:ident) => {
            read_pool_string_arr(&mut powerset.$field, reader, strings, messages)?;
        };
    }

    macro_rules! pset {
        ($($field:ident),+) => {
            $( powerset.$field = bin_read(reader)?; )+
        };
    }

    macro_rules! pset_arr {
        ($($field:ident),+) => {
            $( bin_read_arr(&mut powerset.$field, reader)?; )+
        };
    }

    macro_rules! pset_enum {
        ($($field:ident),+) => {
            $( powerset.$field = bin_read_enum(reader)?; )+
        };
    }

    // TOK_STRUCT data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    pset_string!(pch_source_file);
    powerset.pch_full_name = read_name_key(reader, strings)?;
    pset_string!(pch_name);
    pset_enum!(e_system);
    pset!(b_is_shared);
    pset_string!(
        pch_display_name,
        pch_display_help,
        pch_display_short_help,
        pch_icon_name
    );
    pset_string_arr!(ppch_costume_keys);
    pset_string_arr!(ppch_costume_parts);
    pset_string!(
        pch_account_requires,
        pch_account_tooltip,
        pch_account_product
    );
    pset_string_arr!(ppch_set_buy_requires);
    pset_string!(pch_set_buy_requires_failed_text);
    pset_enum!(e_show_in_inventory);
    pset!(b_show_in_manage, b_show_in_info, i_specialize_at);
    pset_string_arr!(pp_specialize_requires);
    bin_read_arr_fn(
        &mut powerset.pp_power_names,
        |re| {
            if let Ok(Some(power_name)) = read_name_key(re, strings) {
                Ok(power_name)
            } else {
                Err(ParseError::new(ParseErrorKind::MissingNameKey))
            }
        },
        reader,
    )?;
    pset_arr!(pi_available);
    pset_arr!(
        pi_ai_max_level,
        pi_ai_min_rank_con,
        pi_ai_max_rank_con,
        pi_min_difficulty,
        pi_max_difficulty
    );
    pset!(i_force_level_bought);

    verify_struct_length(powerset, expected_bytes, begin_pos, reader)
}
