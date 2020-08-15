use super::*;
use crate::structs::{
    Keyed, PetCommandStrings, PowerNameRef, VillainDef, VillainDefFlags, VillainExclusion,
    VillainLevelDef,
};
use std::rc::Rc;

/// Reads all of the villain definitions in the current .bin file.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for power sets
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a map containing zero or more `VillainDef` structs.
/// Otherwise, a `ParseError` with the error information.
pub fn serialized_read_villains<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Keyed<VillainDef>>
where
    T: Read + Seek,
{
    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    // first read the length of the TOK_EARRAY ParseBasePowerSet[]
    let v_size: usize = bin_read(reader)?;
    let mut villains = Keyed::<_>::new();
    for _ in 0..v_size {
        let villain = read_villain_def(reader, strings, messages)?;
        if let Some(villain_name) = &villain.name {
            villains.insert(villain_name.clone(), Rc::new(villain));
        }
    }

    verify_struct_length(villains, expected_bytes, begin_pos, reader)
}

/// Reads a `VillainDef` struct from a .bin file.
/// Refer to Common/gameComm/VillainDef - Validate.c TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for power sets
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `VillainDef`.
/// Otherwise, a `ParseError` with the error information.
fn read_villain_def<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<VillainDef>
where
    T: Read + Seek,
{
    let mut villain = VillainDef::new();

    macro_rules! v_string {
        ($($field:ident),+) => {
            $( villain.$field = read_pool_string(reader, strings, messages)?; )+
        };
    }

    macro_rules! v_string_arr {
        ($($field:ident),+) => {
            $( read_pool_string_arr(&mut villain.$field, reader, strings, messages)?; )+
        };
    }

    macro_rules! v {
        ($($field:ident),+) => {
            $( villain.$field = bin_read(reader)?; )+
        };
    }

    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    villain.name = read_name_key(reader, strings)?;
    villain.character_class_name = read_name_key(reader, strings)?;

    villain.gender = bin_read_enum(reader)?;
    v_string!(
        description,
        group_description,
        display_class_name,
        ai_config
    );
    v!(group);
    bin_read_arr_fn(
        &mut villain.powers,
        |re| read_power_name_ref(re, strings),
        reader,
    )?;
    bin_read_arr_fn(
        &mut villain.levels,
        |re| read_villain_level_def(re, strings, messages),
        reader,
    )?;
    villain.rank = bin_read_enum(reader)?;
    v_string!(ally, gang);
    villain.exclusion = VillainExclusion::from_bits_truncate(bin_read(reader)?);
    v!(
        ignore_combat_mods,
        copy_creator_mods,
        ignore_reduction,
        can_zone,
        spawn_limit,
        spawn_limit_mission
    );
    // additional rewards TOK_REDUNDANTNAME
    v_string_arr!(additional_rewards);
    v_string!(favorite_weapon);
    v_string_arr!(skill_hp_rewards);
    // integrity failure rewards TOK_REDUNDANTNAME
    v_string_arr!(skill_status_rewards);
    v!(reward_scale);
    v_string_arr!(power_tags);
    v_string!(special_pet_power);
    v_string!(file_name);
    v!(file_age);
    bin_read_arr_fn(
        &mut villain.pet_command_strings,
        |re| read_pet_command_strings(re, strings, messages),
        reader,
    )?;
    v!(pet_visibility, pet_commandability);
    v_string!(custom_badge_stat);
    villain.flags = VillainDefFlags::from_bits_truncate(bin_read(reader)?);
    // script def TOK_NULLSTRUCT
    let _: u32 = bin_read(reader)?;

    verify_struct_length(villain, expected_bytes, begin_pos, reader)
}

/// Common/gameComm/NPC.c
fn read_power_name_ref<T>(reader: &mut T, strings: &StringPool) -> ParseResult<PowerNameRef>
where
    T: Read + Seek,
{
    let mut power_name = PowerNameRef::new();

    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    power_name.power_category = read_name_key(reader, strings)?;
    power_name.power_set = read_name_key(reader, strings)?;
    power_name.power = read_name_key(reader, strings)?;
    power_name.level = bin_read(reader)?;
    power_name.remove = bin_read(reader)?;
    power_name.dont_set_stance = bin_read(reader)?;

    verify_struct_length(power_name, expected_bytes, begin_pos, reader)
}

fn read_villain_level_def<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<VillainLevelDef>
where
    T: Read + Seek,
{
    let mut villain_level = VillainLevelDef::new();

    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    villain_level.level = bin_read(reader)?;
    read_pool_string_arr(&mut villain_level.display_names, reader, strings, messages)?;
    read_pool_string_arr(&mut villain_level.costumes, reader, strings, messages)?;
    villain_level.experience = bin_read(reader)?;

    verify_struct_length(villain_level, expected_bytes, begin_pos, reader)
}

fn read_pet_command_strings<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<PetCommandStrings>
where
    T: Read + Seek,
{
    let mut pet_command = PetCommandStrings::new();

    macro_rules! pc_string_arr {
        ($($field:ident),+) => {
            $( read_pool_string_arr(&mut pet_command.$field, reader, strings, messages)?; )+
        };
    }

    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    pc_string_arr!(
        ppch_passive,
        ppch_defensive,
        ppch_aggressive,
        ppch_attack_target,
        ppch_attack_no_target,
        ppch_stay_here,
        ppch_use_power,
        ppch_use_power_none,
        ppch_follow_me,
        ppch_goto_spot,
        ppch_dismiss
    );

    verify_struct_length(pet_command, expected_bytes, begin_pos, reader)
}
