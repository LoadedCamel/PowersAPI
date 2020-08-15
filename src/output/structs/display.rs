use super::effects::{AttribModOutput, EffectGroupOutput, ScaledUnit};
use super::powers::PowerOutput;
use crate::structs::*;
use std::borrow::Cow;

// Used by format!() calls for common attrib mod types.
macro_rules! g_abs {
    () => {
        "{}"
    };
}
macro_rules! g_cur {
    () => {
        "{}"
    };
}
macro_rules! g_max {
    () => {
        "max {}"
    };
}
macro_rules! g_res {
    () => {
        "resistance to {}"
    };
}
macro_rules! g_str {
    () => {
        "strength to {}"
    };
}

/// Adds descriptive information about an attribute modifier.
///
/// # Arguments:
/// * `attrib_mod` - An `AttribModOutput`.
/// * `template` - The `AttribModTemplate` that was the basis for `attrib_mod`.
/// * `effect_group` - The `EffectGroupOutput` that owns `attrib_mod`.
/// * `effect_requires` - The original requirements clause from `effect_group`.
/// * `attrib_names` - An `AttribNames`.
///
/// # Returns:
/// On exit, the `display_info` in each `scaled` value in `attrib_mod` may contain a human
/// readable string describing the effect.
///
/// # Notes:
/// See `Game/UI/uiPowerInfo.c` for the basis for this information.
pub fn describe_attrib_mod(
    attrib_mod: &mut AttribModOutput,
    template: &AttribModTemplate,
    effect_group: &EffectGroupOutput,
    effect_requires: &Vec<String>,
    attrib_names: &AttribNames,
) {
    // TODO: pet effects would be nice but requires looking up the entity def

    // check for a few special cases and bail
    if matches!(attrib_mod.attr_type, Some(AttribType::kAttribType_Special)) {
        return;
    }
    if template.p_attrib.iter().any(|a| match a {
        SpecialAttrib::kSpecialAttrib_Character(a) => match *a as usize {
            CharacterAttributes::OFFSET_MOVEMENT_CONTROL
            | CharacterAttributes::OFFSET_MOVEMENT_FRICTION
            | CharacterAttributes::OFFSET_EVADE => true,
            _ => false,
        },
        _ => false,
    }) {
        return;
    }

    // Miss Chance
    let chance_for_miss = if !effect_group.chance_percent.is_normal() {
        // (#1) (#8) avoiding skipping the display_info for now, but this is something goofy with power redirects
        None
    } else if effect_group.chance_percent != 100.0 && !attrib_mod.duration_seconds.is_some() {
        Some(format!("{:.2}% chance for ", effect_group.chance_percent))
    } else if let Some(t) = attrib_mod.tick_chance_percent {
        if t != 100.0 && template.i_flags.contains(AttribModFlag::CancelOnMiss) {
            Some(format!("{:.2}% chance for ", t))
        } else {
            None
        }
    } else {
        None
    };

    // Target
    let target = if matches!(template.e_target, ModTarget::kModTarget_Caster) {
        Some(String::from(" on self"))
    } else {
        Some(String::from(" on target"))
    };

    // Delay
    let after_delay = if attrib_mod.after_delay_seconds.is_normal() {
        let duration = get_pretty_duration(attrib_mod.after_delay_seconds);
        Some(format!(" after {} delay", duration))
    } else {
        None
    };

    // Effect types
    let effect_types = get_effect_types(&attrib_mod, &template, &attrib_names);

    // Tags
    let mut is_critical = false;
    let mut tags = Vec::new();
    for tag in &effect_group.tags {
        match *tag {
            "DualPistolsLethalMode" => tags.push("Only using Standard Ammo"),
            "DualPistolsFireMode" => tags.push("Only using Incendiary Ammo"),
            "DualPistolsColdMode" => tags.push("Only using Cryo Ammo"),
            "DualPistolsToxicMode" => tags.push("Only using Chemical Ammo"),
            "FieryEmbrace" => tags.push("Only using Fiery Embrace"),
            "Critical" => is_critical = true,
            _ => (),
        }
    }
    add_requires_tags(&mut tags, effect_requires);
    if template.i_flags.contains(AttribModFlag::IgnoreStrength) {
        tags.push("Ignores buffs and enhancements");
    }
    if template.i_flags.contains(AttribModFlag::IgnoreResistance) {
        tags.push("Unresistable");
    }

    // Final adjustments for each AT scale
    for scaled in &mut attrib_mod.scaled {
        // DoT/HoT
        let mut ticks = 1;
        let mut ticks_of = None;
        let mut chance_for_miss_2 = None;
        if matches!(attrib_mod.attr_type, Some(AttribType::kAttribType_Abs))
            && (matches!(scaled.scaled_effect, ScaledUnit::Damage(_) |
                    ScaledUnit::Healing(_) | ScaledUnit::Value(_)))
        {
            if let Some(duration) = attrib_mod.duration_seconds {
                if attrib_mod.continuous_apply_seconds.is_normal() {
                    ticks = (duration / attrib_mod.continuous_apply_seconds).floor() as i32 + 1;
                    ticks_of = Some(format!("{} ticks of ", ticks));
                    if let Some(t) = attrib_mod.tick_chance_percent {
                        if t != 100.0 && template.i_flags.contains(AttribModFlag::CancelOnMiss) {
                            chance_for_miss_2 = Some(format!("{:.2}% chance for ", t));
                        }
                    }
                }
            }
        }

        // Scaled Value
        let value = Some(match scaled.scaled_effect {
            ScaledUnit::Damage(dmg) => format!("{:.2}", dmg),
            ScaledUnit::Healing(healing) => format!("{:.2}", healing),
            ScaledUnit::Distance(distance) => format!("{:.2} ft.", distance),
            ScaledUnit::DurationSeconds(duration) => get_pretty_duration(duration),
            ScaledUnit::Magnitude(mag) => format!("{:.1}", mag),
            ScaledUnit::Percent(percent) => format!("{:.2}%", percent),
            ScaledUnit::Value(val) => format!("{:.2}", val),
        });

        // Duration
        let duration = if let Some(d) = attrib_mod.duration_seconds {
            d
        } else if let ScaledUnit::DurationSeconds(d) = scaled.scaled_effect {
            d
        } else {
            std::f32::NAN
        };
        let over_time = if duration.is_normal() {
            if ticks > 1 {
                Some(format!(" over {}", get_pretty_duration(duration)))
            } else {
                Some(format!(" for {}", get_pretty_duration(duration)))
            }
        } else {
            None
        };

        // Build final display string!
        let mut final_string = String::with_capacity(128);
        if chance_for_miss_2.is_none() {
            if let Some(p) = &chance_for_miss {
                final_string.push_str(p);
            }
        }
        for part in &[&ticks_of, &chance_for_miss_2] {
            if let Some(p) = part {
                final_string.push_str(p);
            }
        }
        if let ScaledUnit::DurationSeconds(_) = scaled.scaled_effect {
            if let Some(mag) = attrib_mod.magnitude {
                final_string.push_str(&format!("{:.1} ", mag));
            }
        } else if let Some(val) = &value {
            final_string.push_str(val);
            final_string.push(' ');
        }
        for (i, part) in effect_types.iter().enumerate() {
            if i > 0 {
                final_string.push_str(", ");
            }
            final_string.push_str(part);
        }
        for part in &[&over_time, &target, &after_delay] {
            if let Some(p) = part {
                final_string.push_str(p);
            }
        }

        if is_critical {
            final_string.push_str(" [CRITICAL]");
        }

        scaled.display_info.push(Cow::Owned(final_string));

        for part in &tags {
            scaled.display_info.push(Cow::Borrowed(*part));
        }
    }
}

/// Gets the display text for a particular character attribute.
/// Will return "(unknown)" if there's no equivalent UI string.
fn get_attrib_name<'a>(offset: &usize, attrib_names: &'a AttribNames) -> &'a str {
    if let Some(name) = attrib_names.attr_names.get(offset) {
        if let Some(name_str) = &name {
            return &name_str[..];
        }
    }
    match *offset {
        // for some reason, knockback isn't in the table
        //CharacterAttributes::OFFSET_KNOCKBACK => "knockback",
        _ => "(unknown)",
    }
}

/// Gets the display text for all the attribute modifiers in an effect.
fn get_effect_types(
    attrib_mod: &AttribModOutput,
    template: &AttribModTemplate,
    attrib_names: &AttribNames,
) -> Vec<String> {
    let mut effect_types = Vec::new();
    macro_rules! effect {
        ($tag:expr, $offset:expr) => {
            effect_types.push(format!($tag, get_attrib_name($offset, attrib_names)))
        };
    }
    for (idx, attrib) in template.p_attrib.iter().enumerate() {
        if let SpecialAttrib::kSpecialAttrib_Character(a) = attrib {
            let a = *a as usize;
            // damage types
            if matches!(
                a,
                CharacterAttributes::OFFSET_DMG_0..=CharacterAttributes::OFFSET_HIT_POINTS
            ) {
                match attrib_mod.attr_type {
                    Some(AttribType::kAttribType_Str) if idx == 0 => effect!(g_str!(), &a),
                    Some(AttribType::kAttribType_Cur) => effect!(g_cur!(), &a),
                    Some(AttribType::kAttribType_Res) if idx == 0 => effect!(g_res!(), &a),
                    Some(AttribType::kAttribType_Abs) if a == CharacterAttributes::OFFSET_DMG_7 => {
                        effect_types.push(String::from("heal"));
                    }
                    _ => effect!(g_abs!(), &a),
                }
            }
            // defense types
            else if matches!(
                a,
                CharacterAttributes::OFFSET_DEF_0..=CharacterAttributes::OFFSET_DEFENSE |
                CharacterAttributes::OFFSET_ELUSIVITY_0..=CharacterAttributes::OFFSET_ELUSIVITY_BASE
            ) {
                match attrib_mod.attr_type {
                    Some(AttribType::kAttribType_Cur) | Some(AttribType::kAttribType_Str) => {
                        if idx > 0 {
                            effect!(g_abs!(), &a);
                        } else {
                            effect!(g_str!(), &a);
                        }
                    }
                    Some(AttribType::kAttribType_Res) => {
                        if idx > 0 {
                            effect!("{} debuff", &a);
                        } else {
                            effect!("resistance to {} debuff", &a);
                        }
                    }
                    _ => (),
                }
            }
            // status effects
            else if matches!(a, CharacterAttributes::OFFSET_CONFUSED..=CharacterAttributes::OFFSET_ONLY_AFFECTS_SELF |
                CharacterAttributes::OFFSET_KNOCKUP..=CharacterAttributes::OFFSET_REPEL)
            {
                match attrib_mod.attr_type {
                    Some(AttribType::kAttribType_Mod) | Some(AttribType::kAttribType_Cur) => {
                        // HACK: the base attrib_mod doesn't have any info about the final magnitude, so we'll check the first scaled effect
                        if let Some(scaled) = attrib_mod.scaled.get(0) {
                            match scaled.scaled_effect {
                                ScaledUnit::Magnitude(m) if m < 0.0 => {
                                    effect!("{} protection", &a);
                                }
                                ScaledUnit::Magnitude(_) => {
                                    effect!("magnitude {}", &a);
                                }
                                _ => {
                                    effect!(g_abs!(), &a);
                                }
                            }
                        } else {
                            effect!(g_abs!(), &a);
                        }
                    }
                    Some(AttribType::kAttribType_Str) if idx == 0 => effect!(g_str!(), &a),
                    Some(AttribType::kAttribType_Res) if idx == 0 => effect!(g_res!(), &a),
                    _ => effect!(g_abs!(), &a),
                }
            }
            // everything else
            else {
                match attrib_mod.attr_type {
                    Some(AttribType::kAttribType_Cur) | Some(AttribType::kAttribType_Mod) => {
                        effect!(g_cur!(), &a)
                    }
                    Some(AttribType::kAttribType_Res) => {
                        if idx > 0 {
                            effect!("{} debuff", &a);
                        } else {
                            effect!("resistance to {} debuff", &a);
                        }
                    }
                    Some(AttribType::kAttribType_Max) if idx == 0 => effect!(g_max!(), &a),
                    Some(AttribType::kAttribType_Str) if idx == 0 => effect!(g_str!(), &a),
                    _ => effect!(g_abs!(), &a),
                }
            }
        }
    }
    effect_types
}

/// Formats `time` in seconds to a display string.
fn get_pretty_duration(time: f32) -> String {
    const MIN_SECS: f32 = 60.0;
    const HOUR_SECS: f32 = 3600.0;
    const DAY_SECS: f32 = 86400.0;
    if time < MIN_SECS {
        // less than a minute
        format!("{:.}s", super::normalize(time))
    } else if time < HOUR_SECS {
        // include minutes
        let seconds = time % MIN_SECS;
        let minutes = (time - seconds) / MIN_SECS;
        format!("{}m {}s", minutes as i32, seconds as i32)
    } else if time < DAY_SECS {
        // include hours
        let minutes = time % HOUR_SECS / MIN_SECS;
        let hours = (time - minutes * MIN_SECS) / HOUR_SECS;
        format!("{}h {}m", hours as i32, minutes as i32)
    } else {
        // include days
        let hours = time % DAY_SECS / HOUR_SECS;
        let days = (time - hours * HOUR_SECS) / DAY_SECS;
        format!("{}d {}h", days as i32, hours as i32)
    }
}

/// Adds descriptive information about a target type. This attempts to match
/// what is normally seen in the powers info window rather than using the full
/// description of the target type.
///
/// # Arguments:
/// * `target_type` - A `TargetType` value.
///
/// # Returns:
/// A string describing `target_type` in general terms, if possible.
/// Otherwise, `None`.
pub fn describe_target_type(target_type: &TargetType) -> Option<&'static str> {
    match target_type {
        TargetType::kTargetType_Caster => Some("Self"),
        TargetType::kTargetType_Location | TargetType::kTargetType_Teleport => Some("Location"),
        TargetType::kTargetType_Player
        | TargetType::kTargetType_PlayerHero
        | TargetType::kTargetType_PlayerVillain
        | TargetType::kTargetType_DeadPlayer
        | TargetType::kTargetType_DeadPlayerFriend
        | TargetType::kTargetType_Teammate
        | TargetType::kTargetType_DeadTeammate
        | TargetType::kTargetType_DeadOrAliveTeammate
        | TargetType::kTargetType_NPC
        | TargetType::kTargetType_DeadOrAliveFriend
        | TargetType::kTargetType_DeadFriend
        | TargetType::kTargetType_Friend
        | TargetType::kTargetType_Leaguemate
        | TargetType::kTargetType_DeadOrAliveLeaguemate
        | TargetType::kTargetType_DeadLeaguemate => Some("Friendlies"),
        TargetType::kTargetType_Villain
        | TargetType::kTargetType_DeadPlayerFoe
        | TargetType::kTargetType_DeadVillain
        | TargetType::kTargetType_DeadOrAliveFoe
        | TargetType::kTargetType_DeadFoe
        | TargetType::kTargetType_Foe => Some("Enemies"),
        TargetType::kTargetType_MyPet
        | TargetType::kTargetType_DeadOrAliveMyPet
        | TargetType::kTargetType_DeadMyPet
        | TargetType::kTargetType_MyCreation
        | TargetType::kTargetType_DeadOrAliveMyCreation
        | TargetType::kTargetType_DeadMyCreation => Some("Pet"),
        TargetType::kTargetType_Any
        | TargetType::kTargetType_DeadAny
        | TargetType::kTargetType_DeadOrAliveAny => Some("Any"),
        _ => None,
    }
}

/// Adds some descriptive text for specific requirements.
fn add_requires_tags(tags: &mut Vec<&'static str>, requires: &Vec<String>) {
    let requires_str: Vec<_> = requires.iter().map(|s| &**s).collect();
    for i in 0..requires_str.len() {
        match &requires_str[i..] {
            // scrapper crits
            ["arch", "target>", "Class_Minion_Grunt", "eq", "arch", "target>", "Class_Minion_Small", "eq", "||", "arch", "target>", "Class_Minion_Pets", "eq", "||", "arch", "target>", "Class_Minion_Swarm", "eq", "||", "enttype", "target>", "player", "eq", "||", "!", ..] => {
                tags.push("Only against targets tougher than minion class")
            }
            ["arch", "target>", "Class_Minion_Grunt", "eq", "arch", "target>", "Class_Minion_Small", "eq", "||", "arch", "target>", "Class_Minion_Pets", "eq", "||", "arch", "target>", "Class_Minion_Swarm", "eq", "||", ..] => {
                tags.push("Only against minions and underlings")
            }
            // stalker crits
            ["kMeter", "source>", ".9", "<", ..] => {
                for j in 0..requires_str.len() {
                    match &requires_str[j..] {
                        ["kHeld", "target>", "0", ">", "kSleep", "target>", "0", ">", "||", ..] => {
                            tags.push("Only against held or sleeping targets when not hidden")
                        }
                        _ => (),
                    }
                }
            }
            ["kMeter", "source>", "0", ">", ..] => tags.push("Only if hidden or target placated"),
            // domination
            ["kStealth", "source>", "0.5", ">", ..] => tags.push("Only while domination is active"),
            // scourge
            ["kHitPoints%", "target>", "10", "-", "100", "*", "50", "10", "-", "/", "0", "100", "minmax", "rand", "100", "*", ..] => {
                tags.push("2.5% chance per every percentage point targets health is below 50%")
            }
            // containment
            ["kImmobilized", "target>", "0", ">", "kHeld", "target>", "0", ">", "||", "kSleep", "target>", "0", ">", "||", "kStunned", "target>", "0", ">", "||", ..] => {
                tags.push("Only against immobilized, held, sleeping or stunned targets")
            }
            // origins
            ["origin", "source>", "Magic", ..] => tags.push("Only if the player's origin is magic"),
            ["origin", "source>", "Mutation", ..] => {
                tags.push("Only if the player's origin is mutation")
            }
            ["origin", "source>", "Natural", ..] => {
                tags.push("Only if the player's origin is natural")
            }
            ["origin", "source>", "Science", ..] => {
                tags.push("Only if the player's origin is science")
            }
            ["origin", "source>", "Technology", ..] => {
                tags.push("Only if the player's origin is technology")
            }
            // enemy types
            ["Electronic", "target.HasTag?", ..] => tags.push("Against electronic targets only"),
            ["Undead", "target.HasTag?", ..] => tags.push("Against undead targets only"),
            ["Ghost", "target.HasTag?", ..] => tags.push("Against ghost targets only"),
            // combos
            ["kDD_BonusDotMode_2", "source.mode?", ..] => {
                tags.push("Only when Attack Vitals combo is completed")
            }
            ["kDD_DebuffMode_2", "source.mode?", ..] => {
                tags.push("Only when Weaken combo is completed")
            }
            ["kDD_BonusAoEMode_2", "source.mode?", ..] => {
                tags.push("Only when Sweep combo is completed")
            }
            ["kDD_StatusMode_2", "source.mode?", ..] => {
                tags.push("Only when Empower combo is completed")
            }
            ["Temporary_Powers.Temporary_Powers.Combo_Level_1", "source.ownPower?", "!", "Temporary_Powers.Temporary_Powers.Combo_Level_2", "source.ownPower?", "!", "&&", "Temporary_Powers.Temporary_Powers.Combo_Level_3", "source.ownPower?", "!", "&&", ..] => {
                break
            }
            ["Temporary_Powers.Temporary_Powers.Combo_Level_1", "source.ownPower?", "!", "&&", "Temporary_Powers.Temporary_Powers.Combo_Level_2", "source.ownPower?", "!", "&&", "Temporary_Powers.Temporary_Powers.Combo_Level_3", "source.ownPower?", "!", "&&", ..] => {
                break
            }
            ["Temporary_Powers.Temporary_Powers.Combo_Level_1", "source.ownPower?", ..] => {
                tags.push("Only at Combo Level 1")
            }
            ["Temporary_Powers.Temporary_Powers.Combo_Level_2", "source.ownPower?", ..] => {
                tags.push("Only at Combo Level 2")
            }
            ["Temporary_Powers.Temporary_Powers.Combo_Level_3", "source.ownPower?", ..] => {
                tags.push("Only at Combo Level 3")
            }
            ["Temporary_Powers.Temporary_Powers.Temporal_Selection_Buff", "target.ownPower?", "!", ..] => {
                break
            }
            ["Temporary_Powers.Temporary_Powers.Temporal_Selection_Buff", "target.ownPower?", ..] => {
                tags.push("Only when Accelerated")
            }
            ["Temporary_Powers.Temporary_Powers.Time_Crawl_Debuff", "target.ownPower?", "!", ..] => {
                break
            }
            ["Temporary_Powers.Temporary_Powers.Time_Crawl_Debuff", "target.ownPower?", ..] => {
                tags.push("Only when Delayed")
            }
            ["Temporary_Powers.Temporary_Powers.Beam_Rifle_Debuff", "target.ownPower?", "!", ..] => {
                break
            }
            ["Temporary_Powers.Temporary_Powers.Beam_Rifle_Debuff", "target.ownPower?", ..] => {
                tags.push("Only when Disintegrated")
            }
            _ => (),
        }
    }
}

/// Adds descriptive information about a power.
///
/// # Arguments:
/// * `power` - A `PowerOutput` value.
/// * `base_power` - The `BasePower` that was the basis for `power`.
///
/// # Returns:
/// On exit, the `display_info` in `power` may contain a human
/// readable strings describing the power.
///
/// # Notes:
/// See `Game/UI/uiPowerInfo.c` for the basis for this information.
pub fn describe_power(power: &mut PowerOutput, base_power: &BasePower, attrib_names: &AttribNames) {
    // activation traits
    if power.activate.cast_time.is_normal() {
        power.display_info.insert(
            "Activation Time",
            Cow::Owned(get_pretty_duration(power.activate.cast_time)),
        );
    }
    if power.activate.recharge_time.is_normal() {
        power.display_info.insert(
            "Recharge Time",
            Cow::Owned(get_pretty_duration(power.activate.recharge_time)),
        );
    }
    if power.activate.endurance_cost.is_normal() {
        match base_power.e_type {
            PowerType::kPowerType_Toggle if power.activate.auto_cast_interval.is_normal() => {
                let end_cost = power.activate.endurance_cost / power.activate.auto_cast_interval;
                power
                    .display_info
                    .insert("Endurance Cost", Cow::Owned(format!("{:.2}/s", end_cost)));
            }
            _ => {
                power.display_info.insert(
                    "Endurance Cost",
                    Cow::Owned(format!("{:.2}", power.activate.endurance_cost)),
                );
            }
        }
    }
    // acc
    if base_power.p_auto_hit.len() == 0 && power.accuracy.is_normal() {
        power
            .display_info
            .insert("Accuracy", Cow::Owned(format!("{:.2}x", power.accuracy)));
    }
    // target characteristics
    power
        .display_info
        .insert("Power Type", Cow::Borrowed(base_power.e_type.get_string()));
    if let Some(s) = describe_target_type(&base_power.e_target_type) {
        power.display_info.insert("Target Type", Cow::Borrowed(s));
    }
    if let Some(s) = describe_target_type(&base_power.e_target_type_secondary) {
        power
            .display_info
            .insert("Secondary Target Type", Cow::Borrowed(s));
    }
    if power.effect_area.range_feet.is_normal() {
        power.display_info.insert(
            "Power Range",
            Cow::Owned(format!("{} ft.", power.effect_area.range_feet)),
        );
    }
    if power.effect_area.range_feet_secondary.is_normal() {
        power.display_info.insert(
            "Secondary Power Range",
            Cow::Owned(format!("{} ft.", power.effect_area.range_feet_secondary)),
        );
    }
    match base_power.e_effect_area {
        EffectArea::kEffectArea_Character => {
            power
                .display_info
                .insert("Effect Area", Cow::Borrowed("Single Target"));
        }
        EffectArea::kEffectArea_Location => {
            power
                .display_info
                .insert("Effect Area", Cow::Borrowed("Location"));
        }
        EffectArea::kEffectArea_Chain => {
            let mut effect_area = String::with_capacity(64);
            effect_area.push_str(base_power.e_effect_area.get_string());
            effect_area.push_str(" —");
            if power.effect_area.jump_distance_feet.is_normal() {
                effect_area.push_str(&format!(
                    " {} ft. jump distance",
                    power.effect_area.jump_distance_feet
                ));
            }
            if power.effect_area.max_targets_hit > 0 {
                effect_area.push_str(&format!(
                    " ({} targets max)",
                    power.effect_area.max_targets_hit
                ));
            }
            power
                .display_info
                .insert("Effect Area", Cow::Owned(effect_area));
        }
        EffectArea::kEffectArea_Cone | EffectArea::kEffectArea_Sphere => {
            let mut effect_area = String::with_capacity(64);
            effect_area.push_str(base_power.e_effect_area.get_string());
            effect_area.push_str(" —");
            if power.effect_area.radius_feet.is_normal() {
                effect_area.push_str(&format!(" {} ft. radius", power.effect_area.radius_feet));
            }
            if power.effect_area.arc_degrees.is_normal() {
                effect_area.push_str(&format!(" {}° arc", power.effect_area.arc_degrees));
            }
            if power.effect_area.max_targets_hit > 0 {
                effect_area.push_str(&format!(
                    " ({} targets max)",
                    power.effect_area.max_targets_hit
                ));
            }
            power
                .display_info
                .insert("Effect Area", Cow::Owned(effect_area));
        }
        _ => (),
    }
    // attack types and aggro
    if power.attack_types.len() > 0 {
        let mut attack_types = Vec::new();
        for special_attrib in &base_power.pe_attack_types {
            match special_attrib {
                SpecialAttrib::kSpecialAttrib_Character(a) => {
                    match *a as usize {
                        // ppDefense starts at offset OFFSET_DEF_0
                        i
                        @
                        CharacterAttributes::OFFSET_DEF_0
                            ..=CharacterAttributes::OFFSET_DEF_19 => {
                            if let Some(name) = attrib_names
                                .pp_defense
                                .get((i - CharacterAttributes::OFFSET_DEF_0) / 4)
                            {
                                attack_types.push(&name.pch_display_name.as_ref().unwrap()[..]);
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        if attack_types.len() > 0 {
            power
                .display_info
                .insert("Attack Types", Cow::Owned(attack_types.join(", ")));
        }
    }
    match base_power.e_ai_report {
        AIReport::kAIReport_Never => {
            power.display_info.insert(
                "Aggro Type",
                Cow::Borrowed("Enemies will not notice this attack"),
            );
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pretty_duration_seconds_test() {
        let time = 2.543f32;
        let output = get_pretty_duration(time);
        assert_eq!(output, "2.54s");

        let time = 19.987f32;
        let output = get_pretty_duration(time);
        assert_eq!(output, "19.99s");
    }

    #[test]
    fn get_pretty_duration_minutes_test() {
        let time = 105.4f32;
        let output = get_pretty_duration(time);
        assert_eq!(output, "1m 45s");

        let time = 320.0f32;
        let output = get_pretty_duration(time);
        assert_eq!(output, "5m 20s");
    }

    #[test]
    fn get_pretty_duration_hours_test() {
        let time = 5544.0f32;
        let output = get_pretty_duration(time);
        assert_eq!(output, "1h 32m");

        let time = 22032.0f32;
        let output = get_pretty_duration(time);
        assert_eq!(output, "6h 7m");
    }

    #[test]
    fn get_pretty_duration_days_test() {
        let time = 475_200.0f32;
        let output = get_pretty_duration(time);
        assert_eq!(output, "5d 12h");

        let time = 321_408.0f32;
        let output = get_pretty_duration(time);
        assert_eq!(output, "3d 17h");
    }
}
