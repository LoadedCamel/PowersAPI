use super::*;
use crate::structs::*;
use std::convert::TryFrom;
use std::rc::Rc;

const MAX_ATTRIBMOD_FX: usize = 4;
const ATTRIBMOD_FLAGS_SIZE: usize = 2;

/// Reads all of the powers in the current .bin file.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for power sets
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a map containing zero or more `BasePower` structs.
/// Otherwise, a `ParseError` with the error information.
pub fn serialized_read_powers<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Keyed<BasePower>>
where
    T: Read + Seek,
{
    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    // first read the length of the TOK_EARRAY ParseBasePower[]
    let pbp_size: usize = bin_read(reader)?;
    let mut powers = Keyed::<_>::new();
    for _ in 0..pbp_size {
        let power = read_base_power(reader, strings, messages)?;
        if let Some(power_name) = &power.pch_full_name {
            powers.insert(power_name.clone(), Rc::new(power));
        }
    }
    verify_struct_length(powers, expected_bytes, begin_pos, reader)
}

/// Reads a BasePower struct from a .bin file.
/// Refer to Common/entity/powers_load.c TokenizerParseInfo structs.
fn read_base_power<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<BasePower>
where
    T: Read + Seek,
{
    let mut power = BasePower::new();

    macro_rules! pwr_string {
        ($($field:ident),+) => { $( power.$field = read_pool_string(reader, strings, messages)?; )+ }
    }

    macro_rules! pwr_string_arr {
        ($($field:ident), +) => {
            $( read_pool_string_arr(&mut power.$field, reader, strings, messages)?; )+
        };
    }

    macro_rules! pwr {
        ($($field:ident),+) => { $(
            power.$field = bin_read(reader)?;
         )+ }
    }

    macro_rules! pwr_enum {
        ($($field:ident),+) => {
            $( power.$field = bin_read_enum(reader)?; )+
        };
    }

    macro_rules! pwr_enum_arr {
        ($($field:ident),+) => {
            $( bin_read_arr_fn(&mut power.$field, |re| bin_read_enum(re), reader)?; )+
        };
    }

    macro_rules! pwr_attrib_arr {
        ($field:ident) => {
            bin_read_arr_fn(
                &mut power.$field,
                |re| Ok(SpecialAttrib::from_i32(bin_read(re)?)),
                reader,
            )?;
        };
    }

    // TOK_STRUCT data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    // beginning of BasePower
    power.pch_full_name = read_name_key(reader, strings)?;
    if power.pch_full_name.is_none() {
        return Err(ParseError::new(ParseErrorKind::MissingNameKey));
    }
    let _: u32 = bin_read(reader)?; // crc_full_name
    pwr_string!(source_file, pch_name, pch_source_name);
    pwr_enum!(e_system);
    pwr!(b_auto_issue, b_auto_issue_save_level, b_free);
    pwr_string!(pch_display_name, pch_display_help, pch_display_short_help);
    // display caster help TOK_REDUNDANTNAME
    // display caster short help TOK_REDUNDANTNAME
    pwr_string!(
        pch_display_target_help,
        pch_display_target_short_help,
        pch_display_attacker_attack,
        pch_display_attacker_attack_floater,
        pch_display_attacker_hit,
        pch_display_victim_hit,
        pch_display_confirm,
        pch_display_float_rewarded,
        pch_display_defense_float,
        pch_icon_name
    );
    // fx name TOK_IGNORE
    pwr_enum!(e_type);
    pwr!(i_num_allowed);
    pwr_attrib_arr!(pe_attack_types);
    pwr_string_arr!(
        ppch_buy_requires,
        ppch_activate_requires,
        ppch_slot_requires,
        ppch_target_requires,
        ppch_reward_requires,
        ppch_auction_requires
    );
    pwr_string!(pch_reward_fallback);
    pwr!(f_accuracy, b_near_ground, b_target_near_ground);
    pwr_enum!(e_death_castable_setting);
    pwr!(
        b_cast_through_hold,
        b_cast_through_sleep,
        b_cast_through_stun,
        b_cast_through_terrorize,
        b_toggle_ignore_hold,
        b_toggle_ignore_sleep,
        b_toggle_ignore_stun,
        b_ignore_level_bought,
        b_shoot_through_untouchable,
        b_interrupt_like_sleep
    );

    pwr_enum!(e_ai_report);
    pwr_enum!(e_effect_area);
    pwr!(i_max_targets_hit);

    // added i26p4
    pwr_string_arr!(ppch_max_targets_expr);

    pwr!(f_radius, f_arc, f_chain_delay);
    pwr_string_arr!(ppch_chain_eff);
    bin_read_arr(&mut power.pi_chain_fork, reader)?;

    // added i26p5
    bin_read_arr(&mut power.pi_unknown, reader)?;

    pwr!(vec_box_offset, vec_box_size);
    pwr!(
        f_range,
        f_range_secondary,
        f_time_to_activate,
        f_recharge_time,
        f_activate_period,
        f_endurance_cost
    );
    // insight cost TOK_REDUNDANTNAME
    pwr!(
        f_insight_cost, // IdeaCost
        i_time_to_confirm,
        b_self_confirm
    ); // This is originally tokenized as an int, but it behaves like a bool so I changed it.
    pwr_string_arr!(ppch_confirm_requires);
    pwr!(
        b_destroy_on_limit,
        b_stacking_usage,
        i_num_charges,
        i_max_num_charges,
        f_usage_time,
        f_max_usage_time,
        f_lifetime,
        f_max_lifetime,
        f_lifetime_in_game,
        f_max_lifetime_in_game,
        f_interrupt_time
    );
    pwr_enum!(e_target_visibility, e_target_type, e_target_type_secondary);
    pwr_enum_arr!(p_auto_hit, p_affected);
    pwr!(b_targets_through_vision_phase);
    pwr_attrib_arr!(pe_boosts_allowed);
    pwr_attrib_arr!(pe_group_membership);
    pwr_attrib_arr!(pe_modes_required);
    pwr_attrib_arr!(pe_modes_disallowed);
    pwr_string_arr!(ppch_ai_groups);

    bin_read_arr_fn(
        &mut power.pp_redirect,
        |re| read_power_redirect(re, strings, messages),
        reader,
    )?;
    bin_read_arr_fn(
        &mut power.pp_effects,
        |re| Ok(Rc::new(read_effect_group(re, strings, messages)?)),
        reader,
    )?;

    // attrib mod TOK_NO_BIN
    pwr!(b_ignore_strength);
    // ignore str TOK_REDUNDANTNAME
    pwr!(b_show_buff_icon);
    pwr_enum!(e_show_in_inventory);
    pwr!(
        b_show_in_manage,
        b_show_in_info,
        b_deletable,
        b_tradeable,
        i_max_boosts,
        b_do_not_save
    );
    // does not expire TOK_REDUNDANTNAME
    pwr!(
        b_boost_ignore_effectiveness,
        b_boost_always_count_for_set,
        b_boost_tradeable,
        b_boost_combinable,
        b_boost_account_bound,
        b_boost_boostable,
        b_boost_use_player_level
    );
    pwr_string!(pch_boost_catalyst_conversion, pch_store_product);
    pwr!(
        i_boost_invention_license_required_level,
        i_min_slot_level,
        i_max_slot_level,
        i_max_boost_level
    );

    // next 3 added i26p5
    let _: f32 = bin_read(reader)?; // default 1.0?
    let _: f32 = bin_read(reader)?; // default 999999.0?
    let _: f32 = bin_read(reader)?; // default 1.0?

    // changed i26p5: pp_vars appears to be an array of character attributes now
    pwr_attrib_arr!(pp_vars);

    pwr_enum!(e_toggle_droppable);
    // toggles droppable TOK_REDUNDANTNAME
    pwr_enum!(e_proc_allowed);

    // changed i26p5: removed these?
    //pwr_attrib_arr!(p_strengths_disallowed);
    //pwr!(b_use_non_boost_templates_on_main_target, b_main_target_only);

    pwr_string_arr!(ppch_highlight_eval);
    pwr_string!(pch_highlight_icon);
    pwr!(
        rgba_highlight_ring,
        f_travel_suppression,
        f_preference_multiplier,
        b_dont_set_stance,
        f_point_val,
        f_point_multiplier
    );
    pwr_string!(pch_chain_into_power_name);
    pwr!(
        b_instance_locked,
        b_is_environment_hit,
        b_shuffle_target_list,
        i_force_level_bought,
        b_refreshes_on_active_player_change,
        b_cancelable,
        b_ignore_toggle_max_distance,
        i_server_tray_priority
    );
    pwr_string_arr!(ppch_server_tray_requires);
    pwr!(b_abusive_buff);
    pwr_enum!(e_position_center);
    pwr!(
        f_position_distance,
        f_position_height,
        f_position_yaw,
        b_face_target
    );

    pwr_attrib_arr!(pe_attrib_cache);
    let fx_source_file = read_pool_string(reader, strings, messages)?;
    power.p_fx = Some(read_power_fx(fx_source_file, reader, strings, messages)?);

    bin_read_arr_fn(
        &mut power.pp_custom_fx,
        |re| read_custom_power_fx(re, strings, messages),
        reader,
    )?;
    // power redirector TOK_IGNORE

    verify_struct_length(power, expected_bytes, begin_pos, reader)
}

/// Reads a `PowerRedirect` struct from a .bin file.
/// Refer to Common/entity/powers_load.c TokenizerParseInfo structs.
fn read_power_redirect<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<PowerRedirect>
where
    T: Read + Seek,
{
    let mut redirect = PowerRedirect::new();
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    redirect.pch_name = read_name_key(reader, strings)?;
    read_pool_string_arr(&mut redirect.ppch_requires, reader, strings, messages)?;
    redirect.b_show_in_info = bin_read(reader)?;
    Ok(verify_struct_length(
        redirect,
        expected_bytes,
        begin_pos,
        reader,
    )?)
}

/// Reads an `EffectGroup` struct from a .bin file.
/// Refer to Common/entity/attribmod.h TokenizerParseInfo structs.
fn read_effect_group<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<EffectGroup>
where
    T: Read + Seek,
{
    let mut egroup = EffectGroup::new();
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    read_pool_string_arr(&mut egroup.ppch_tags, reader, strings, messages)?;
    egroup.f_chance = bin_read(reader)?;
    egroup.f_procs_per_minute = bin_read(reader)?;
    egroup.f_delay = bin_read(reader)?;
    egroup.f_radius_inner = bin_read(reader)?;
    egroup.f_radius_outer = bin_read(reader)?;
    read_pool_string_arr(&mut egroup.ppch_requires, reader, strings, messages)?;
    egroup.i_flags = EffectGroupFlag::from_bits_truncate(bin_read(reader)?);
    egroup.i_eval_flags = bin_read(reader)?;
    bin_read_arr_fn(
        &mut egroup.pp_templates,
        |re| read_attrib_mod_template(re, strings, messages),
        reader,
    )?;
    bin_read_arr_fn(
        &mut egroup.pp_effects,
        |re| read_effect_group(re, strings, messages),
        reader,
    )?;
    Ok(verify_struct_length(
        egroup,
        expected_bytes,
        begin_pos,
        reader,
    )?)
}

/// Reads an `AttribModTemplate` struct from a .bin file.
/// Refer to Common/entity/attribmod.h TokenizerParseInfo structs.
fn read_attrib_mod_template<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<AttribModTemplate>
where
    T: Read + Seek,
{
    let mut template = AttribModTemplate::new();

    macro_rules! tpl_string_arr {
        ($field:ident) => {
            read_pool_string_arr(&mut template.$field, reader, strings, messages)?;
        };
    }

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    bin_read_arr_fn(
        &mut template.p_attrib,
        |re| Ok(SpecialAttrib::from_i32(bin_read(re)?)),
        reader,
    )?;
    template.off_aspect = bin_read(reader)?; // TODO: AspectEnum
    template.e_application_type = bin_read_enum(reader)?;
    template.e_type = bin_read_enum(reader)?;
    template.e_target = bin_read_enum(reader)?;
    // AttribModTemplate is one of the only places where we see TOK_OPTIONALSTRUCT, which just has a 0 or 1 for size
    if bin_read::<u32, _>(reader)? > 0 {
        template.p_target_info = Some(read_attrib_mod_target_info(reader, strings, messages)?);
    }
    template.pch_table = read_pool_string(reader, strings, messages)?;
    template.f_scale = bin_read(reader)?;
    template.f_duration = ModDuration::from_f32(bin_read(reader)?);
    template.f_magnitude = bin_read(reader)?; // TODO: ParsePowerDefines
    tpl_string_arr!(ppch_duration);
    tpl_string_arr!(ppch_magnitude);
    template.f_delay = bin_read(reader)?;
    template.f_period = bin_read(reader)?;
    template.f_tick_chance = bin_read(reader)?;
    tpl_string_arr!(ppch_delayed_requires);
    template.e_caster_stack = bin_read_enum(reader)?;
    template.e_stack = bin_read_enum(reader)?;
    template.i_stack_limit = bin_read(reader)?;
    template.i_stack_key = bin_read(reader)?; // TODO: ParsePowerDefines
    let size = bin_read(reader)?;
    for _ in 0..size {
        template.pi_cancel_events.push(
            if let Ok(val) = PowerEvent::try_from(bin_read::<u32, _>(reader)?) {
                val
            } else {
                PowerEvent::default()
            },
        );
    }
    bin_read_arr_fn(
        &mut template.pp_suppress,
        |re| read_suppress_pair(re),
        reader,
    )?;
    template.boost_mod_allowed = SpecialAttrib::from_i32(bin_read(reader)?);

    // i_flags are stored in two separate 32-bit ints, I combine them into a single 64-bit flag
    let mut i_flags = [0u32; ATTRIBMOD_FLAGS_SIZE];
    for idx in 0..ATTRIBMOD_FLAGS_SIZE {
        i_flags[idx] = bin_read(reader)?;
    }
    let u_flags = i_flags[0] as u64 | ((i_flags[1] as u64) << 32);
    template.i_flags = AttribModFlag::from_bits_truncate(u_flags);

    // TOK_OPTIONALSTRUCT
    if bin_read::<u32, _>(reader)? > 0 {
        template.p_messages = Some(read_attrib_mod_messages(reader, strings, messages)?);
    }
    // TOK_OPTIONALSTRUCT
    if bin_read::<u32, _>(reader)? > 0 {
        template.p_fx = Some(read_attrib_mod_fx(reader, strings, messages)?);
    }
    template.p_params = read_attrib_mod_params(reader, strings, messages)?;
    Ok(verify_struct_length(
        template,
        expected_bytes,
        begin_pos,
        reader,
    )?)
}

/// Reads a `SuppressPair` struct from a .bin file.
/// Refer to Common/entity/attribmod.h TokenizerParseInfo structs.
fn read_suppress_pair<T>(reader: &mut T) -> ParseResult<SuppressPair>
where
    T: Read + Seek,
{
    let mut pair = SuppressPair::new();

    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    pair.idx_event = bin_read(reader)?;
    pair.ul_seconds = bin_read(reader)?;
    pair.b_always = bin_read(reader)?;

    verify_struct_length(pair, expected_bytes, begin_pos, reader)
}

/// Reads an `AttribModTargetInfo` struct from a .bin file.
/// Refer to Common/entity/attribmod.h TokenizerParseInfo structs.
fn read_attrib_mod_target_info<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<AttribModTargetInfo>
where
    T: Read + Seek,
{
    let mut target = AttribModTargetInfo::new();
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    read_pool_string_arr(&mut target.ppch_marker_names, reader, strings, messages)?;
    bin_read_arr(&mut target.pi_marker_count, reader)?;
    verify_struct_length(target, expected_bytes, begin_pos, reader)
}

/// Reads an `AttribModMessages` struct from a .bin file.
/// Refer to Common/entity/attribmod.h TokenizerParseInfo structs.
fn read_attrib_mod_messages<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<AttribModMessages>
where
    T: Read + Seek,
{
    let mut amodmsg = AttribModMessages::new();
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    amodmsg.pch_display_attacker_hit = read_pool_string(reader, strings, messages)?;
    amodmsg.pch_display_victim_hit = read_pool_string(reader, strings, messages)?;
    amodmsg.pch_display_float = read_pool_string(reader, strings, messages)?;
    amodmsg.pch_display_defense_float = read_pool_string(reader, strings, messages)?;
    verify_struct_length(amodmsg, expected_bytes, begin_pos, reader)
}

/// Reads an `AttribModFX` struct from a .bin file.
/// Refer to Common/entity/attribmod.h TokenizerParseInfo structs.
fn read_attrib_mod_fx<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<AttribModFX>
where
    T: Read + Seek,
{
    let mut amodfx = AttribModFX::new();
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    bin_read_arr(&mut amodfx.pi_continuing_bits, reader)?; // TODO: ParsePowerDefines
    amodfx.pch_continuing_fx = read_pool_string(reader, strings, messages)?;
    bin_read_arr(&mut amodfx.pi_conditional_bits, reader)?; // TODO: ParsePowerDefines
    amodfx.pch_conditional_fx = read_pool_string(reader, strings, messages)?;
    verify_struct_length(amodfx, expected_bytes, begin_pos, reader)
}

/// Reads an `AttribModParam` enum from a .bin file.
/// Refer to Common/entity/attribmod.h TokenizerParseInfo structs.
fn read_attrib_mod_params<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<Option<AttribModParam>>
where
    T: Read + Seek,
{
    let struct_id = bin_read(reader)?;
    if struct_id > 0 {
        let (expected_bytes, begin_pos) = read_struct_length(reader)?;
        let ret_val = match struct_id {
            1 => {
                // Costume
                let mut costume = AttribModParam_Costume::new();
                costume.pch_costume_name = read_pool_string(reader, strings, messages)?;
                costume.i_priority = bin_read(reader)?;
                Some(AttribModParam::Costume(costume))
            }
            2 => {
                // Reward
                let mut reward = AttribModParam_Reward::new();
                read_pool_string_arr(&mut reward.ppch_rewards, reader, strings, messages)?;
                Some(AttribModParam::Reward(reward))
            }
            3 => {
                // EntCreate
                let mut entcreate = AttribModParam_EntCreate::new();
                entcreate.pch_entity_def = read_name_key(reader, strings)?;
                entcreate.pch_class = read_pool_string(reader, strings, messages)?;
                entcreate.pch_costume_name = read_pool_string(reader, strings, messages)?;
                entcreate.pch_display_name = read_pool_string(reader, strings, messages)?;
                entcreate.pch_priority_list = read_pool_string(reader, strings, messages)?;
                entcreate.pch_ai_config = read_pool_string(reader, strings, messages)?;
                read_name_key_arr(&mut entcreate.ppch_category_names, reader, strings)?;
                read_name_key_arr(&mut entcreate.ppch_powerset_names, reader, strings)?;
                read_name_key_arr(&mut entcreate.ppch_power_names, reader, strings)?;
                // i26p5: this appears to be an array that wasn't being used before
                read_name_key_arr(&mut entcreate.redirects, reader, strings)?;
                // pp powers TOK_NO_BIN
                // p villain TOK_NO_BIN
                // p class TOK_NO_BIN
                // npc costume TOK_NO_BIN
                // pch doppelganger TOK_NO_BIN
                Some(AttribModParam::EntCreate(entcreate))
            }
            4 => {
                // Power
                let mut power = AttribModParam_Power::new();
                read_name_key_arr(&mut power.ppch_category_names, reader, strings)?;
                read_name_key_arr(&mut power.ppch_powerset_names, reader, strings)?;
                read_name_key_arr(&mut power.ppch_power_names, reader, strings)?;
                power.i_count = bin_read(reader)?;
                Some(AttribModParam::Power(power))
            }
            5 => {
                // Phase
                let mut phase = AttribModParam_Phase::new();
                bin_read_arr(&mut phase.pi_combat_phases, reader)?;
                bin_read_arr(&mut phase.pi_vision_phases, reader)?;
                phase.i_exclusive_vision_phase = bin_read(reader)?;
                Some(AttribModParam::Phase(phase))
            }
            6 => {
                // Teleport
                let mut teleport = AttribModParam_Teleport::new();
                teleport.pch_destination = read_pool_string(reader, strings, messages)?;
                Some(AttribModParam::Teleport(teleport))
            }
            7 => {
                // Behavior
                let mut behavior = AttribModParam_Behavior::new();
                read_pool_string_arr(&mut behavior.ppch_behaviors, reader, strings, messages)?;
                Some(AttribModParam::Behavior(behavior))
            }
            8 => {
                // SZEValue
                let mut sze_value = AttribModParam_SZEValue::new();
                read_pool_string_arr(&mut sze_value.ppch_script_id, reader, strings, messages)?;
                read_pool_string_arr(&mut sze_value.ppch_script_value, reader, strings, messages)?;
                Some(AttribModParam::SZEValue(sze_value))
            }
            9 => {
                // Token
                let mut token = AttribModParam_Token::new();
                read_pool_string_arr(&mut token.ppch_tokens, reader, strings, messages)?;
                Some(AttribModParam::Token(token))
            }
            10 => {
                // EffectFilter
                let mut filter = AttribModParam_EffectFilter::new();
                read_pool_string_arr(&mut filter.ppch_category_names, reader, strings, messages)?;
                read_pool_string_arr(&mut filter.ppch_powerset_names, reader, strings, messages)?;
                read_pool_string_arr(&mut filter.ppch_power_names, reader, strings, messages)?;
                read_pool_string_arr(&mut filter.ppch_tags, reader, strings, messages)?;
                Some(AttribModParam::EffectFilter(filter))
            }
            11 => {
                // Added i26p5. Unknown.
                let mut param11 = AttribModParam_Param11::new();
                param11.i_unknown_1 = bin_read(reader)?;
                param11.i_unknown_2 = bin_read(reader)?;
                param11.i_unknown_3 = bin_read(reader)?;
                param11.f_unknown_4 = bin_read(reader)?;
                param11.i_unknown_5 = bin_read(reader)?;
                param11.i_unknown_6 = bin_read(reader)?;
                param11.f_unknown_7 = bin_read(reader)?;
                param11.f_unknown_8 = bin_read(reader)?;
                param11.f_unknown_9 = bin_read(reader)?;
                param11.f_unknown_10 = bin_read(reader)?;
                Some(AttribModParam::Param11(param11))
            }
            _ => {
                panic!("AttribModParam unknown struct_id: {}", struct_id);
            }
        };
        verify_struct_length(ret_val, expected_bytes, begin_pos, reader)
    } else {
        Ok(None)
    }
}

/// Reads a `PowerFX` struct from a .bin file.
/// Refer to Common/entity/powers_load.c TokenizerParseInfo structs.
/// NOTE: This one doesn't work like other structs. It gets included in the middle
/// of a parent, so it doesn't read its own struct length and verify it.
fn read_power_fx<T>(
    source_file: Option<String>,
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<PowerFX>
where
    T: Read + Seek,
{
    let mut fx = PowerFX::new();

    macro_rules! fx_string {
        ($($field:ident),+) => {
            $( fx.$field = read_pool_string(reader, strings, messages)?; )+
        }
    }

    macro_rules! fx {
        ($($field:ident),+) => {
            $( fx.$field = bin_read(reader)?; )+
        }
    }

    macro_rules! fx_arr {
        ($($field:ident),+) => {
            $( bin_read_arr(&mut fx.$field, reader)?; )+
        };
    }

    // We don't read the struct len here because this struct packed in-line with the BasePower struct
    fx.pch_source_file = source_file;
    fx_arr!(
        pi_attack_bits,
        pi_block_bits,
        pi_wind_up_bits,
        pi_hit_bits,
        pi_death_bits,
        pi_activation_bits,
        pi_deactivation_bits,
        pi_initial_attack_bits,
        pi_continuing_bits,
        pi_conditional_bits
    );
    fx_string!(
        pch_activation_fx,
        pch_deactivation_fx,
        pch_attack_fx,
        pch_secondary_attack_fx,
        pch_hit_fx,
        pch_wind_up_fx,
        pch_block_fx,
        pch_death_fx,
        pch_initial_attack_fx
    );

    let _: u32 = bin_read(reader)?; // the original token parser reads the first value into pch_continuing_fx[0] twice, we'll just skip it
    for _ in 0..MAX_ATTRIBMOD_FX {
        if let Some(s) = read_pool_string(reader, strings, messages)? {
            fx.ppch_continuing_fx.push(s);
        }
    }
    let _: u32 = bin_read(reader)?; // same deal as above
    for _ in 0..MAX_ATTRIBMOD_FX {
        if let Some(s) = read_pool_string(reader, strings, messages)? {
            fx.ppch_conditional_fx.push(s);
        }
    }

    fx_arr!(pi_mode_bits);
    fx!(i_frames_before_hit, i_frames_before_secondary_hit);
    if fx.i_frames_before_hit == 0 {
        fx.i_frames_before_hit = 15; // see comments on field
    }
    // seq bits TOK_REDUNDANTNAME
    // cast anim TOK_REDUNDANTNAME
    // hit anim TOK_REDUNDANTNAME
    // deathanimbits TOK_REDUNDANTNAME
    // AttachedAnim TOK_REDUNDANTNAME
    // AttachedFxName TOK_REDUNDANTNAME
    // TravellingProjectileEffect TOK_REDUNDANTNAME
    // AttachedToVictimFxName TOK_REDUNDANTNAME
    // TimeBeforePunchHit TOK_REDUNDANTNAME
    // TimeBeforeMissileSpawns TOK_REDUNDANTNAME
    fx.b_delayed_hit = bin_read(reader)?;
    // toggle power TOK_IGNORE
    fx!(i_frames_attack);
    if fx.i_frames_attack == 0 {
        fx.i_frames_attack = 35; // see comments on field
    }
    // non interrupt frames TOK_REDUNDANTNAME
    fx!(i_initial_frames_before_hit, i_initial_attack_fx_frame_delay);
    if fx.i_initial_frames_before_hit == 0 {
        fx.i_initial_frames_before_hit = 15; // see comments on field
    }
    fx!(
        f_projectile_speed,
        f_secondary_projectile_speed,
        i_initial_frames_before_block
    );
    fx_string!(pch_ignore_attack_time_errors);
    fx!(i_frames_before_block);
    fx!(
        // added i26p5
        b_fx_important,
        rgba_default_tint_primary,
        rgba_default_tint_secondary,
        // added i26p5
        b_hide_original
    );

    Ok(fx)
}

/// Reads a `CustomPowerFX` struct from a .bin file.
/// Refer to Common/entity/powers_load.c TokenizerParseInfo structs.
fn read_custom_power_fx<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<CustomPowerFX>
where
    T: Read + Seek,
{
    let mut cfx = CustomPowerFX::new();

    // i26p5 change: this appears to be a proper struct now
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    cfx.pch_token = read_pool_string(reader, strings, messages)?;
    read_pool_string_arr(&mut cfx.ppch_alt_themes, reader, strings, messages)?;
    let source_file = read_pool_string(reader, strings, messages)?;
    cfx.pch_category = read_pool_string(reader, strings, messages)?;
    cfx.pch_display_name = read_pool_string(reader, strings, messages)?;
    cfx.p_fx = Some(read_power_fx(source_file, reader, strings, messages)?);
    cfx.pch_palette_name = read_pool_string(reader, strings, messages)?;
    verify_struct_length(cfx, expected_bytes, begin_pos, reader)
}
