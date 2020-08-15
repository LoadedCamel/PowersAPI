use super::*;
use crate::structs::{BoostList, BoostSet, BoostSetBonus, Keyed, NameKey};
use std::rc::Rc;

/// Reads all of the boost sets in the current .bin file.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for boost sets
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a map containing zero or more `BoostSet` structs.
/// Otherwise, a `ParseError` with the error information.
pub fn serialized_read_boost_sets<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Keyed<BoostSet>>
where
    T: Read + Seek,
{
    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    let mut boost_sets = Keyed::<_>::new();
    let bs_size: usize = bin_read(reader)?;
    for _ in 0..bs_size {
        let boost_set = read_boost_set(reader, strings, messages)?;
        if let Some(name) = &boost_set.pch_name {
            boost_sets.insert(name.clone(), Rc::new(boost_set));
        }
    }

    verify_struct_length(boost_sets, expected_bytes, begin_pos, reader)
}

/// Reads a `BoostSet` struct from a .bin file.
/// Refer to Common/entity/boostset.h TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `BoostSet`.
/// Otherwise, a `ParseError` with the error information.
fn read_boost_set<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<BoostSet>
where
    T: Read + Seek,
{
    let mut boost_set = BoostSet::new();
    macro_rules! bs_string {
        ($($field:ident),+) => { $( boost_set.$field = read_pool_string(reader, strings, messages)?; )+ }
    }

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    boost_set.pch_name = read_name_key(reader, strings)?;
    bs_string!(pch_display_name, pch_group_name);
    read_pool_string_arr(
        &mut boost_set.ppch_conversion_groups,
        reader,
        strings,
        messages,
    )?;

    read_link_table(&mut boost_set.ppch_powers, reader)?;

    bin_read_arr_fn(
        &mut boost_set.pp_boost_lists,
        |reader| Ok(read_boost_list(reader)?),
        reader,
    )?;
    bin_read_arr_fn(
        &mut boost_set.pp_bonuses,
        |reader| Ok(read_boost_set_bonus(reader, strings, messages)?),
        reader,
    )?;

    boost_set.i_min_level = bin_read(reader)?;
    boost_set.i_max_level = bin_read(reader)?;
    bs_string!(pch_store_product);

    verify_struct_length(boost_set, expected_bytes, begin_pos, reader)
}

/// Reads a `BoostList` struct from a .bin file.
/// Refer to Common/entity/boostset.h TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `BoostList`.
/// Otherwise, a `ParseError` with the error information.
fn read_boost_list<T>(reader: &mut T) -> ParseResult<BoostList>
where
    T: Read + Seek,
{
    let mut boost_list = BoostList::new();

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    read_link_table(&mut boost_list.ppch_boosts, reader)?;

    verify_struct_length(boost_list, expected_bytes, begin_pos, reader)
}

/// Reads a `BoostSetBonus` struct from a .bin file.
/// Refer to Common/entity/boostset.h TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `BoostSetBonus`.
/// Otherwise, a `ParseError` with the error information.
fn read_boost_set_bonus<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<BoostSetBonus>
where
    T: Read + Seek,
{
    let mut boost_set_bonus = BoostSetBonus::new();

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    boost_set_bonus.pch_display_name = read_pool_string(reader, strings, messages)?;
    boost_set_bonus.i_min_boosts = bin_read(reader)?;
    boost_set_bonus.i_max_boosts = bin_read(reader)?;
    read_pool_string_arr(
        &mut boost_set_bonus.ppch_requires,
        reader,
        strings,
        messages,
    )?;
    read_link_table(&mut boost_set_bonus.ppch_auto_powers, reader)?;
    let ps = read_pascal_string_with_padding(reader)?;
    if ps.len() > 0 {
        boost_set_bonus.pch_bonus_power = Some(NameKey::new(ps));
    }
    verify_struct_length(boost_set_bonus, expected_bytes, begin_pos, reader)
}

fn read_link_table<T>(table: &mut Vec<NameKey>, reader: &mut T) -> ParseResult<()>
where
    T: Read + Seek,
{
    // link tables are stored as arrays of Pascal strings
    bin_read_arr_fn(
        table,
        |reader| {
            let ps = read_pascal_string_with_padding(reader)?;
            Ok(NameKey::new(ps))
        },
        reader,
    )?;
    Ok(())
}
