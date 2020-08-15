use super::*;
use crate::structs::{
    Archetype, CharacterAttributes, CharacterAttributesTable, Keyed, NameKey, NamedTable,
};
use std::rc::Rc;

/// Reads all of the archetypes in the current .bin file.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a map containing zero or more `Archetype` structs.
/// Otherwise, a `ParseError` with the error information.
pub fn serialized_read_archetypes<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Keyed<Archetype>>
where
    T: Read + Seek,
{
    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    let mut archetypes = Keyed::<_>::new();
    let at_size: usize = bin_read(reader)?;
    for _ in 0..at_size {
        let archetype = read_archetype(reader, strings, messages)?;
        if let Some(class_key) = &archetype.class_key {
            archetypes.insert(class_key.clone(), Rc::new(archetype));
        }
    }

    verify_struct_length(archetypes, expected_bytes, begin_pos, reader)
}

/// Reads an `Archetype` struct from a .bin file.
/// Refer to Common/entity/classesh TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, an `Archetype`.
/// Otherwise, a `ParseError` with the error information.
fn read_archetype<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Archetype>
where
    T: Read + Seek,
{
    let mut archetype = Archetype::new();

    macro_rules! at_string {
        ($($field:ident),+) => { $( archetype.$field = read_pool_string(reader, strings, messages)?; )+ }
    }

    macro_rules! at_string_arr {
        ($field:ident) => {
            read_pool_string_arr(&mut archetype.$field, reader, strings, messages)?;
        };
    }

    macro_rules! at_attrib_arr {
        ($field:ident) => {
            bin_read_arr_fn(
                &mut archetype.$field,
                |re| read_character_attributes(re),
                reader,
            )?;
        };
    }

    macro_rules! at_table_arr {
        ($field:ident) => {
            bin_read_arr_fn(
                &mut archetype.$field,
                |re| read_character_attributes_table(re),
                reader,
            )?;
        };
    }

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    at_string!(pch_name, pch_display_name, pch_display_help);
    if let Some(name) = &archetype.pch_name {
        // This is used later to speed up matching against "requires" fields.
        let mut class_key = String::new();
        class_key.push_str(&Archetype::CLASS_PREFIX[0..1]);
        let lcase_name = name.to_ascii_lowercase().replace(' ', "_");
        if !lcase_name.starts_with(&Archetype::CLASS_PREFIX[1..]) {
            class_key.push_str(&Archetype::CLASS_PREFIX[1..]);
        }
        class_key.push_str(&lcase_name);
        archetype.class_key = Some(NameKey::new(class_key));
    }
    at_string_arr!(ppch_allowed_origin_names);
    at_string_arr!(ppch_special_restrictions);
    at_string!(
        pch_store_restrictions,
        pch_locked_tooltip,
        pch_product_code,
        pch_reduction_class
    );
    archetype.b_reduce_as_av = bin_read(reader)?;
    bin_read_arr(&mut archetype.pi_level_up_respecs, reader)?;
    at_string!(pch_display_short_help, pch_icon);
    archetype.pch_primary_category = read_name_key(reader, strings)?;
    archetype.pch_secondary_category = read_name_key(reader, strings)?;
    archetype.pch_power_pool_category = read_name_key(reader, strings)?;
    archetype.pch_epic_pool_category = read_name_key(reader, strings)?;
    at_attrib_arr!(pp_attrib_min);
    at_attrib_arr!(pp_attrib_base);
    at_attrib_arr!(pp_attrib_strength_min);
    at_attrib_arr!(pp_attrib_resistance_min);

    // For each of Strength, Current, Resistances: Read the inner and out diminishing returns tables.
    // (Will be 6 reads.)
    for i in &[
        Archetype::kClassesDiminish_Inner,
        Archetype::kClassesDiminish_Outer,
    ] {
        bin_read_arr_fn(
            &mut archetype.pp_attrib_diminishing_str[*i],
            |re| read_character_attributes(re),
            reader,
        )?;
    }
    for i in &[
        Archetype::kClassesDiminish_Inner,
        Archetype::kClassesDiminish_Outer,
    ] {
        bin_read_arr_fn(
            &mut archetype.pp_attrib_diminishing_cur[*i],
            |re| read_character_attributes(re),
            reader,
        )?;
    }
    for i in &[
        Archetype::kClassesDiminish_Inner,
        Archetype::kClassesDiminish_Outer,
    ] {
        bin_read_arr_fn(
            &mut archetype.pp_attrib_diminishing_res[*i],
            |re| read_character_attributes(re),
            reader,
        )?;
    }

    at_table_arr!(pp_attrib_temp_max);
    at_table_arr!(pp_attrib_temp_max_max);
    at_table_arr!(pp_attrib_temp_strength_max);
    at_table_arr!(pp_attrib_temp_resistance_max);
    let size: u32 = bin_read(reader)?;
    for _ in 0..size {
        let table = read_named_table(reader, strings, messages)?;
        if let Some(table_name) = &table.pch_name {
            archetype
                .pp_named_tables
                .insert(table_name.to_lowercase(), table);
        }
    }
    archetype.b_connect_hp_and_status = bin_read(reader)?;
    // connect hp and integrity TOK_REDUNDANTNAME
    archetype.off_defiant_hit_points_attrib = bin_read(reader)?;
    archetype.f_defiant_scale = bin_read(reader)?;

    verify_struct_length(archetype, expected_bytes, begin_pos, reader)
}

/// Reads a `CharacterAttributes` struct from a .bin file.
/// Refer to Common/entity/character_attribs.h TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `CharacterAttributes`.
/// Otherwise, a `ParseError` with the error information.
fn read_character_attributes<T>(reader: &mut T) -> ParseResult<CharacterAttributes>
where
    T: Read + Seek,
{
    let mut attrib = CharacterAttributes::new();

    macro_rules! attr {
        ($($field:ident),+) => { $( attrib.$field = bin_read(reader)?; )+ }
    }

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    for i in 0..attrib.f_damage_type.len() {
        attrib.f_damage_type[i] = bin_read(reader)?;
    }
    attr!(f_hit_points, f_absorb, f_endurance, f_insight);
    // idea TOK_REDUNDANTNAME
    attr!(f_rage, f_to_hit);
    for i in 0..attrib.f_defense_type.len() {
        attrib.f_defense_type[i] = bin_read(reader)?;
    }
    attr!(f_defense, f_speed_running);
    // run speed TOK_REDUNDANTNAME
    attr!(f_speed_flying);
    // fly speed TOK_REDUNDANTNAME
    attr!(
        f_speed_swimming,
        f_speed_jumping,
        f_jump_height,
        f_movement_control,
        f_movement_friction,
        f_stealth,
        f_stealth_radius,
        f_stealth_radius_player,
        f_perception_radius,
        f_regeneration,
        f_recovery,
        f_insight_recovery,
        f_threat_level,
        f_taunt,
        f_placate
    );

    attr!(f_confused);
    // confuse TOK_REDUNDANTNAME
    attr!(f_afraid, f_terrorized);
    // terrorize TOK_REDUNDANTNAME
    attr!(f_held, f_immobilized);
    // immobilize TOK_REDUNDANTNAME
    attr!(f_stunned);
    // stun TOK_REDUNDANTNAME
    attr!(
        f_sleep,
        f_fly,
        f_jump_pack,
        f_teleport,
        f_untouchable,
        f_intangible,
        f_only_affects_self,
        f_experience_gain,
        f_influence_gain,
        f_prestige_gain,
        f_null_bool
    );
    // evade TOK_REDUNDANTNAME
    attr!(
        f_knock_up,
        f_knock_back,
        f_repel,
        f_accuracy,
        f_radius,
        f_arc,
        f_range,
        f_time_to_activate,
        f_recharge_time,
        f_interrupt_time,
        f_endurance_discount,
        f_insight_discount,
        f_meter
    );

    for i in 0..attrib.f_elusivity.len() {
        attrib.f_elusivity[i] = bin_read(reader)?;
    }
    attr!(f_elusivity_base);

    verify_struct_length(attrib, expected_bytes, begin_pos, reader)
}

/// Reads a `CharacterAttributesTable` struct from a .bin file.
/// Refer to Common/entity/character_attribs.h TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `CharacterAttributesTable`.
/// Otherwise, a `ParseError` with the error information.
fn read_character_attributes_table<T>(reader: &mut T) -> ParseResult<CharacterAttributesTable>
where
    T: Read + Seek,
{
    let mut table = CharacterAttributesTable::new();

    macro_rules! tbl_arr {
        ($($field:ident),+) => { $(
            bin_read_arr(&mut table.$field, reader)?;
        )+ }
    }

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    for i in 0..table.pf_damage_type.len() {
        bin_read_arr(&mut table.pf_damage_type[i], reader)?;
    }
    tbl_arr!(pf_hit_points, pf_endurance, pf_insight);
    // idea TOK_REDUNDANTNAME
    tbl_arr!(pf_rage, pf_to_hit);
    for i in 0..table.pf_defense_type.len() {
        bin_read_arr(&mut table.pf_defense_type[i], reader)?;
    }
    tbl_arr!(pf_defense, pf_speed_running);
    // run speed TOK_REDUNDANTNAME
    tbl_arr!(pf_speed_flying);
    // fly speed TOK_REDUNDANTNAME
    tbl_arr!(
        pf_speed_swimming,
        pf_speed_jumping,
        pf_jump_height,
        pf_movement_control,
        pf_movement_friction,
        pf_stealth,
        pf_stealth_radius,
        pf_stealth_radius_player,
        pf_perception_radius,
        pf_regeneration,
        pf_recovery,
        pf_insight_recovery,
        pf_threat_level,
        pf_taunt,
        pf_placate,
        pf_confused
    );
    // confuse TOK_REDUNDANTNAME
    tbl_arr!(pf_afraid, pf_terrorized);
    // terrorize TOK_REDUNDANTNAME
    tbl_arr!(pf_held, pf_immobilized);
    // immobilize TOK_REDUNDANTNAME
    tbl_arr!(pf_stunned);
    // stun TOK_REDUNDANTNAME
    tbl_arr!(
        pf_sleep,
        pf_fly,
        pf_jump_pack,
        pf_teleport,
        pf_untouchable,
        pf_intangible,
        pf_only_affects_self,
        pf_experience_gain,
        pf_influence_gain,
        pf_prestige_gain,
        pf_null_bool
    );
    // evade TOK_REDUNDANTNAME
    tbl_arr!(
        pf_knock_up,
        pf_knock_back,
        pf_repel,
        pf_accuracy,
        pf_radius,
        pf_arc,
        pf_range,
        pf_time_to_activate,
        pf_recharge_time,
        pf_interrupt_time,
        pf_endurance_discount,
        pf_insight_discount,
        pf_meter
    );
    for i in 0..table.pf_elusivity.len() {
        bin_read_arr(&mut table.pf_elusivity[i], reader)?;
    }
    tbl_arr!(pf_defense, pf_absorb);

    verify_struct_length(table, expected_bytes, begin_pos, reader)
}

/// Reads a `NamedTable` struct from a .bin file.
/// Refer to Common/entity/classes.h TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `NamedTable`.
/// Otherwise, a `ParseError` with the error information.
fn read_named_table<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<NamedTable>
where
    T: Read + Seek,
{
    let mut table = NamedTable::new();

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    table.pch_name = read_pool_string(reader, strings, messages)?;
    bin_read_arr(&mut table.pf_values, reader)?;

    verify_struct_length(table, expected_bytes, begin_pos, reader)
}
