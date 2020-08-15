use super::powers::make_power_ref_url;
use super::*;
use crate::structs::{Archetype, AttribModParam, AttribModTemplate, AttribNames, EffectGroup};
use display;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashSet;

// Tags PvP vs PvE rules.
const PVE_TAG: &'static str = "PVE";
const PVP_TAG: &'static str = "PVP";

// Offsets into character tables.
const OFFSET_MODIFIERS: u32 = 0;
const OFFSET_MAXIMUM: u32 = 8;
const OFFSET_STRENGTH: u32 = 16;
const OFFSET_RESIST: u32 = 24;
const OFFSET_ABSOLUTE: u32 = 32;

/// Describes the different types of scaled effects.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaledUnit {
    Damage(f32),
    Healing(f32),
    Percent(f32),
    DurationSeconds(f32),
    Magnitude(f32),
    Value(f32),
    Distance(f32),
}

#[derive(Serialize)]
pub struct AttribModParamPowerOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<String>,
}

#[derive(Serialize)]
pub struct AttribModParamScriptValueOutput {
    id: Option<String>,
    value: Option<String>,
}

#[derive(Serialize)]
pub struct AttribModParamPowerRefAndUrl {
    pub name: Option<NameKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttribModParamOutput {
    Costume {
        costume_name: Option<String>,
    },
    Effect {
        tags: Vec<String>,
    },
    CreateEntity {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<NameKey>,
        #[serde(skip_serializing_if = "Option::is_none")]
        display_name: Option<String>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        powers: Vec<AttribModParamPowerRefAndUrl>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        power_names: Vec<NameKey>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        redirects: Vec<AttribModParamPowerRefAndUrl>,
    },
    Phase {
        exclusive_vision_phase: i32,
        combat_phases: Vec<i32>,
        vision_phases: Vec<i32>,
    },
    Power {
        #[serde(skip_serializing_if = "Option::is_none")]
        count: Option<i32>,
        powers: Vec<AttribModParamPowerRefAndUrl>,
    },
    Reward {
        rewards: Vec<String>,
    },
    Teleport {
        destination: Option<String>,
    },
    Token {
        tokens: Vec<String>,
    },
    Behavior {
        behaviors: Vec<String>,
    },
    ScriptValue {
        values: Vec<AttribModParamScriptValueOutput>,
    },
}

impl AttribModParamOutput {
    fn from_attrib_mod_param(param: &AttribModParam, config: &PowersConfig) -> Option<Self> {
        match param {
            AttribModParam::Costume(c) => Some(AttribModParamOutput::Costume {
                costume_name: c.pch_costume_name.clone(),
            }),
            AttribModParam::EffectFilter(f) => Some(AttribModParamOutput::Effect {
                tags: f.ppch_tags.clone(),
            }),
            AttribModParam::EntCreate(e) => {
                if let Some(villain) = &e.villain_def {
                    let mut display_name = None;
                    let mut powers = Vec::new();
                    if let Some(level_def) = villain.levels.get(config.at_level as usize) {
                        display_name = level_def.display_names.get(0).cloned();
                    }
                    for power in &e.power_refs {
                        powers.push(AttribModParamPowerRefAndUrl {
                            name: Some(power.clone()),
                            display_name: None, // TODO
                            url: make_power_ref_url(Some(power), config),
                        });
                    }
                    Some(AttribModParamOutput::CreateEntity {
                        name: e.pch_entity_def.clone(),
                        display_name,
                        powers,
                        power_names: Vec::new(),
                        redirects: Vec::new(),
                    })
                } else if e.redirects.len() > 0 {
                    let mut powers = Vec::new();
                    for power in &e.redirects {
                        powers.push(AttribModParamPowerRefAndUrl {
                            name: Some(power.clone()),
                            display_name: None, // TODO
                            url: make_power_ref_url(Some(power), config),
                        });
                    }
                    Some(AttribModParamOutput::CreateEntity {
                        name: None,
                        display_name: None,
                        powers: Vec::new(),
                        power_names: Vec::new(),
                        redirects: powers,
                    })
                } else {
                    Some(AttribModParamOutput::CreateEntity {
                        name: e.pch_entity_def.clone(),
                        display_name: e.pch_display_name.clone(),
                        powers: Vec::new(),
                        power_names: e.ppch_power_names.clone(),
                        redirects: Vec::new(),
                    })
                }
            }
            AttribModParam::Phase(ph) => Some(AttribModParamOutput::Phase {
                exclusive_vision_phase: ph.i_exclusive_vision_phase,
                combat_phases: ph.pi_combat_phases.clone(),
                vision_phases: ph.pi_vision_phases.clone(),
            }),
            AttribModParam::Power(p) => {
                let mut powers = Vec::new();
                for power_name in &p.ppch_power_names {
                    powers.push(AttribModParamPowerRefAndUrl {
                        name: Some(power_name.to_owned()),
                        display_name: None, // TODO
                        url: make_power_ref_url(Some(power_name), config),
                    });
                }
                let count = if p.i_count > 1 { Some(p.i_count) } else { None };
                Some(AttribModParamOutput::Power { count, powers })
            }
            AttribModParam::Reward(r) => Some(AttribModParamOutput::Reward {
                rewards: r.ppch_rewards.clone(),
            }),
            AttribModParam::Teleport(t) => Some(AttribModParamOutput::Teleport {
                destination: t.pch_destination.clone(),
            }),
            AttribModParam::Token(tk) => Some(AttribModParamOutput::Token {
                tokens: tk.ppch_tokens.clone(),
            }),
            AttribModParam::Behavior(b) => Some(AttribModParamOutput::Behavior {
                behaviors: b.ppch_behaviors.clone(),
            }),
            AttribModParam::SZEValue(s) => {
                let mut values = Vec::new();
                for (id, value) in s.ppch_script_id.iter().zip(&s.ppch_script_value) {
                    values.push(AttribModParamScriptValueOutput {
                        id: Some(id.clone()),
                        value: Some(value.clone()),
                    });
                }
                Some(AttribModParamOutput::ScriptValue { values })
            }
            AttribModParam::Param11(_) => {
                // TODO: not sure what this is yet, something related to chains?
                // Currently it doesn't seem to be used by any player power
                None
            }
        }
    }
}

#[derive(Serialize)]
pub struct AttribModScaled {
    pub archetype: Option<String>,
    #[serde(flatten)]
    pub scaled_effect: ScaledUnit,
    #[serde(skip_serializing_if = "not_normal")]
    pub average: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub per_activation: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub per_cast_cycle: f32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub display_info: Vec<Cow<'static, str>>,
    pub base_value: f32,
    pub scale: f32,
}

#[derive(Default, Serialize)]
pub struct StackingOutput {
    pub behavior: &'static str,
    pub by_caster: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl StackingOutput {
    fn from_attrib_mod_template(
        attrib_mod: &AttribModTemplate,
        attrib_names: &AttribNames,
    ) -> Self {
        let mut stacking = StackingOutput {
            behavior: attrib_mod.e_stack.get_string(),
            by_caster: matches!(
                attrib_mod.e_caster_stack,
                CasterStackType::kCasterStackType_Individual
            ),
            ..Default::default()
        };
        if matches!(
            attrib_mod.e_stack,
            StackType::kStackType_StackThenIgnore | StackType::kStackType_RefreshToCount
        ) {
            stacking.limit = Some(attrib_mod.i_stack_limit);
        }
        if attrib_mod.i_stack_key > 0 {
            if let Some(name) = attrib_names
                .pp_stack_key
                .get(attrib_mod.i_stack_key as usize)
            {
                stacking.key = name.pch_name.clone();
            }
        }
        stacking
    }
}

#[derive(Default, Serialize)]
pub struct AttribModOutput {
    pub attributes: Vec<Cow<'static, str>>,
    pub applies_to: Option<&'static str>,
    pub application_type: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_chance_percent: Option<f32>,
    pub target_type: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnitude_expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_expression: Option<String>,
    #[serde(skip_serializing_if = "not_normal")]
    pub after_delay_seconds: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub continuous_apply_seconds: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<AttribModParamOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacking: Option<StackingOutput>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scaled: Vec<AttribModScaled>,
    // unserialized fields
    #[serde(skip)]
    pub attr_type: Option<AttribType>,
}

impl AttribModOutput {
    fn from_attrib_mod_template(
        attrib_mod: &AttribModTemplate,
        attrib_names: &AttribNames,
        archetypes: &Vec<Rc<Archetype>>,
        config: &PowersConfig,
    ) -> Self {
        let mut output = AttribModOutput {
            application_type: Some(attrib_mod.e_application_type.get_string()),
            tick_chance_percent: Some(normalize(attrib_mod.f_tick_chance * 100.0)),
            target_type: Some(attrib_mod.e_target.get_string()),
            after_delay_seconds: normalize(attrib_mod.f_delay),
            continuous_apply_seconds: normalize(attrib_mod.f_period),
            flags: attrib_mod.i_flags.get_strings(),
            ..Default::default()
        };
        // Stacking rules
        if !matches!(attrib_mod.e_stack, StackType::kStackType_Ignore) {
            output.stacking = Some(StackingOutput::from_attrib_mod_template(
                attrib_mod,
                attrib_names,
            ));
        }
        // Handle different expressions
        if attrib_mod.ppch_magnitude.len() > 0 {
            output.magnitude_expression = requires_to_string(&attrib_mod.ppch_magnitude);
        }
        if attrib_mod.ppch_duration.len() > 0 {
            output.duration_expression = requires_to_string(&attrib_mod.ppch_duration);
        } else {
            match attrib_mod.f_duration {
                // describing InSeconds(0) as Instant is probably easier to parse
                ModDuration::InSeconds(secs) if not_normal(&secs) => {
                    output.duration = Some(ModDuration::kModDuration_Instant.get_string());
                    match attrib_mod.e_application_type {
                        // If the effect doesn't have a duration, the tick qualities don't matter.
                        ModApplicationType::kModApplicationType_OnTick
                            if attrib_mod.f_tick_chance == 1.0 =>
                        {
                            output.application_type = Some("Immediate");
                            output.tick_chance_percent = None;
                        }
                        _ => (),
                    }
                }
                ModDuration::InSeconds(secs) => {
                    output.duration = Some(attrib_mod.f_duration.get_string());
                    output.duration_seconds = Some(secs);
                    if output.continuous_apply_seconds.is_normal() {
                        output.ticks =
                            Some((secs / output.continuous_apply_seconds).floor() as i32 + 1);
                    }
                }
                _ => output.duration = Some(attrib_mod.f_duration.get_string()),
            }
        }
        // attribs
        for attrib in &attrib_mod.p_attrib {
            if let Some(attrib_name) = character_attrib_to_string(attrib, attrib_names) {
                output.attributes.push(attrib_name);
            }
        }
        if let Some(SpecialAttrib::kSpecialAttrib_Character(a)) = attrib_mod.p_attrib.get(0) {
            output.attr_type = attrib_type(attrib_mod.off_aspect, *a);
        } else {
            output.attr_type = Some(AttribType::kAttribType_Special);
        }
        output.applies_to = Some(output.attr_type.as_ref().unwrap().get_string());
        // special cases for "booleans"
        if let Some(attrib) = attrib_mod.p_attrib.get(0) {
            match attrib {
                SpecialAttrib::kSpecialAttrib_Character(a) => {
                    let a = *a as usize;
                    // if a >= CharacterAttributes::OFFSET_CONFUSED
                    //     && a <= CharacterAttributes::OFFSET_ONLY_AFFECTS_SELF
                    // {
                        // base magnitude is only relevant if this is a boolean attribute
                        output.magnitude = Some(normalize(attrib_mod.f_magnitude));
                    // }
                    match attrib_mod.e_type {
                        // if the mod is of type duration, it's scaled effect will be the duration
                        ModType::kModType_Duration => {
                            // duration is calculated
                            output.duration = Some("InSecondsScaled");
                            // probably got overwritten above
                            output.application_type =
                                Some(attrib_mod.e_application_type.get_string());
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        // params
        if let Some(param) = &attrib_mod.p_params {
            output.parameter = AttribModParamOutput::from_attrib_mod_param(param, config);
        }
        // scaling per archetype
        if !matches!(output.attr_type, Some(AttribType::kAttribType_Special)) {
            output.add_effect_scales(attrib_mod, archetypes, config.at_level);
            if let Some(scaled) = output.scaled.get(0) {
                match scaled.scaled_effect {
                    // Reduce confusion by blanking the base magnitude (would always be 1.0 in this case anyways)
                    ScaledUnit::Magnitude(_) => output.magnitude = None,
                    _ => (),
                }
            }
        }
        output
    }

    /// Calculates the scaled effects for an attribute modifier.
    fn add_effect_scales(
        &mut self,
        attrib_mod: &AttribModTemplate,
        archetypes: &Vec<Rc<Archetype>>,
        at_level: i32,
    ) {
        if let Some(table_name) = &attrib_mod.pch_table {
            for at in archetypes {
                // calculate scaled effect for each archetype attached to this power
                if let Some(named_table) = at.pp_named_tables.get(&table_name.to_lowercase()) {
                    let base_value = named_table.pf_values[(at_level - 1) as usize];
                    let scaled_value = base_value * attrib_mod.f_scale;
                    if let Some(scaled_effect) = get_scaled_effect(
                        attrib_mod,
                        self.attr_type.as_ref().unwrap(),
                        scaled_value,
                    ) {
                        self.scaled.push(AttribModScaled {
                            archetype: at.pch_display_name.clone(),
                            scaled_effect,
                            average: 0.0,
                            per_activation: 0.0,
                            per_cast_cycle: 0.0,
                            display_info: Vec::new(),
                            base_value: normalize4(base_value),
                            scale: normalize4(attrib_mod.f_scale),
                        });
                    }
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct EffectGroupOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pve_or_pvp: Option<&'static str>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub tags: HashSet<&'static str>,
    pub visible_in_info_window: bool,
    pub chance_percent: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub procs_per_minute: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub after_delay_seconds: f32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<&'static str>,
    pub effects: Vec<AttribModOutput>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub child_effect_groups: Vec<EffectGroupOutput>,
}

impl EffectGroupOutput {
    pub fn from_effect_group(
        effect: &EffectGroup,
        attrib_names: &AttribNames,
        base_power: &BasePower,
        archetypes: &Vec<Rc<Archetype>>,
        config: &PowersConfig,
    ) -> Self {
        let mut group = EffectGroupOutput {
            pve_or_pvp: get_pve_or_pvp(&effect.ppch_tags, &effect.i_flags, &effect.ppch_requires),
            tags: HashSet::new(),
            visible_in_info_window: true,
            chance_percent: normalize(effect.f_chance * 100.0),
            procs_per_minute: normalize(effect.f_procs_per_minute),
            after_delay_seconds: normalize(effect.f_delay),
            requires: Vec::new(),
            flags: effect.i_flags.get_strings(),
            effects: Vec::new(),
            child_effect_groups: Vec::new(),
        };
        if let Some(rule) = requires_to_string(&effect.ppch_requires) {
            group.requires.push(rule);
        }
        check_tags_group(&mut group, &effect.ppch_tags);
        check_special_requires(&mut group, &effect.ppch_requires);
        let filtered_archetypes = filter_archetypes_eg(effect, archetypes);
        for attrib_mod in &effect.pp_templates {
            let mut attrib_mod_output = AttribModOutput::from_attrib_mod_template(
                attrib_mod,
                attrib_names,
                &filtered_archetypes,
                config,
            );
            check_tags_effect(&mut attrib_mod_output, &effect.ppch_tags);
            display::describe_attrib_mod(
                &mut attrib_mod_output,
                attrib_mod,
                &group,
                &effect.ppch_requires,
                &attrib_names,
            );
            calculate_damage(&mut attrib_mod_output, &group, attrib_mod, base_power);
            group.effects.push(attrib_mod_output);
        }
        for child_group in &effect.pp_effects {
            group
                .child_effect_groups
                .push(EffectGroupOutput::from_effect_group(
                    child_group,
                    attrib_names,
                    base_power,
                    archetypes,
                    config,
                ));
        }
        group
    }
}

/// Derives additional damage stats on an `attrib_mod`.
fn calculate_damage(
    attrib_mod: &mut AttribModOutput,
    effect_group: &EffectGroupOutput,
    template: &AttribModTemplate,
    base_power: &BasePower,
) {
    for scaled in &mut attrib_mod.scaled {
        // look for damage/healing attributes
        if matches!(attrib_mod.attr_type, Some(AttribType::kAttribType_Abs))
            && (matches!(scaled.scaled_effect, ScaledUnit::Damage(_) |
                ScaledUnit::Healing(_) | ScaledUnit::Value(_)))
        {
            // get value
            let amount = match scaled.scaled_effect {
                ScaledUnit::Damage(d) => d,
                ScaledUnit::Healing(h) => h,
                ScaledUnit::Value(v) => v,
                _ => 0.0,
            };

            // check for continuous effect
            let mut ticks = 1;
            let mut tick_chance = 1.0;
            if let Some(duration) = attrib_mod.duration_seconds {
                if attrib_mod.continuous_apply_seconds.is_normal() {
                    ticks = (duration / attrib_mod.continuous_apply_seconds).floor() as i32 + 1;
                    if let Some(t) = attrib_mod.tick_chance_percent {
                        if t != 100.0 && template.i_flags.contains(AttribModFlag::CancelOnMiss) {
                            tick_chance = t / 100.0;
                        }
                    }
                }
            }

            // total
            scaled.average = normalize(if tick_chance < 1.0 {
                // cancel on miss average
                let mut avg_ticks = 0.0;
                for k in 1..ticks {
                    avg_ticks += tick_chance.powi(k) * (1.0 - tick_chance) * k as f32;
                }
                avg_ticks += tick_chance.powi(ticks) * ticks as f32;
                amount * avg_ticks
            } else {
                // consistent damage
                amount * (effect_group.chance_percent / 100.0)
            });

            // derived
            if base_power.f_time_to_activate.is_normal() {
                scaled.per_activation = normalize(scaled.average / base_power.f_time_to_activate);
                if base_power.f_recharge_time.is_normal() {
                    scaled.per_cast_cycle = normalize(
                        scaled.average
                            / (base_power.f_time_to_activate + base_power.f_recharge_time),
                    );
                }
            }
        }
    }
}

/// Filters the archetypes vector based on any purchase requirements specified in `effect`.
/// If `effect` has no requirements, all archetypes passed in will be returned.
fn filter_archetypes_eg(
    effect: &EffectGroup,
    archetypes: &Vec<Rc<Archetype>>,
) -> Vec<Rc<Archetype>> {
    // filter out the MLCrit and BossCrit effects, they use arch to test for NPC archetypes
    if !effect
        .ppch_tags
        .iter()
        .any(|tag| matches!(&tag[..], "MLCrit" | "BossCrit"))
        && effect.ppch_requires.iter().any(|rule| rule == "arch")
    {
        // second form of this rule compares to the latter half of the class key
        archetypes
            .iter()
            .filter(|at| {
                if let Some(class_key) = &at.class_key {
                    effect.ppch_requires.iter().any(|rule| {
                        rule.to_ascii_lowercase() == &class_key.get()[Archetype::CLASS_PREFIX_LEN..]
                    })
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    } else {
        archetypes.clone()
    }
}

/// Checks the requires clause of an effect group to see if it applies to pve, pvp, or both.
fn get_pve_or_pvp(
    tags: &Vec<String>,
    flags: &EffectGroupFlag,
    requires: &Vec<String>,
) -> Option<&'static str> {
    // check the mode flags first, they're the most explicit
    if flags.contains(EffectGroupFlag::PVEOnly) {
        return Some(PVE_TAG);
    } else if flags.contains(EffectGroupFlag::PVPOnly) {
        return Some(PVP_TAG);
    }

    let requires_str = requires.iter().map(|s| &**s).collect::<Vec<_>>();

    // check for the MLCrit and BossCrit effects, they use player to test for non-pvp in most cases
    // but may have an explicit expression
    if tags
        .iter()
        .any(|tag| matches!(&tag[..], "MLCrit" | "BossCrit"))
    {
        match requires_str[..] {
            ["enttype", "target>", "critter", "eq"] => return Some(PVE_TAG),
            ["enttype", "target>", "player", "eq"] => return Some(PVP_TAG),
            _ => (),
        }
        // NOTE: may false positive
        return Some(PVE_TAG);
    }

    // iterate through requirements to find common ways of checking for PVP/PVE
    // NOTE: can potentially result in false positives, especially the first 2 clauses
    for i in 0..requires_str.len() {
        match requires_str[i..] {
            ["enttype", "target>", "critter", "eq", ..] => return Some(PVE_TAG),
            ["enttype", "target>", "player", "eq", ..] => return Some(PVP_TAG),
            ["isPVPMap?", "!", ..] => return Some(PVE_TAG),
            ["isPVPMap?", ..] => return Some(PVP_TAG),
            _ => (),
        }
    }
    None
}

/// Modifies `effect_group` based on the content of `requires`.
fn check_special_requires(effect_group: &mut EffectGroupOutput, requires: &Vec<String>) {
    let requires_str = requires.iter().map(|s| &**s).collect::<Vec<_>>();
    for i in 0..requires_str.len() {
        match &requires_str[i..] {
            // domination
            ["kStealth", "source>", "0.5", ">", ..] => {
                effect_group.tags.insert("Domination");
            }
            // scourge
            ["kHitPoints%", "target>", "10", "-", "100", "*", "50", "10", "-", "/", "0", "100", "minmax", "rand", "100", "*", ..] =>
            {
                effect_group.tags.insert("Scourge");
            }
            // containment
            ["kImmobilized", "target>", "0", ">", "kHeld", "target>", "0", ">", "||", "kSleep", "target>", "0", ">", "||", "kStunned", "target>", "0", ">", "||", ..] =>
            {
                effect_group.tags.insert("Containment");
            }
            _ => (),
        }
    }
}

/// Modifies `effect_group` based on the content of `tags`.
fn check_tags_group(effect_group: &mut EffectGroupOutput, tags: &Vec<String>) {
    let tags_str = tags.iter().map(|s| &**s).collect::<Vec<_>>();
    for tag in tags_str {
        // several tags modify the chance of an effect, these refer to "global chance mods"
        // that are handled in code
        match tag {
            "FieryEmbrace" => {
                effect_group.tags.insert("FieryEmbrace");
                effect_group.chance_percent = 100.0;
            }
            "Lethal" | "FireDamage" | "ColdDamage" | "ToxicDamage" => {
                effect_group.chance_percent = 100.0;
            }
            "LethalKB10" => effect_group.chance_percent = 10.0,
            "LethalKB25" => effect_group.chance_percent = 25.0,
            "LethalKB50" => effect_group.chance_percent = 50.0,
            "LethalKB70" => effect_group.chance_percent = 70.0,
            "HailofBulletsKnockdown" => effect_group.chance_percent = 5.0,
            "HailofBulletsEndKnockback" => effect_group.chance_percent = 40.0,
            "FireDamageDoT" => {
                effect_group.chance_percent = 100.0;
            }
            "HailofBulletsFire" | "HailofBulletsCold" | "HailofBulletsToxic" => {
                effect_group.chance_percent = 60.0;
            }
            _ => (),
        }
        // gather certain tags together and promote them to effect tags
        match tag {
            "MLCrit" | "BossCrit" | "PlayerCrit" => {
                effect_group.tags.insert("Critical");
            }
            "Lethal"
            | "LethalKB10"
            | "LethalKB25"
            | "LethalKB50"
            | "LethalKB70"
            | "HailofBulletsKnockdown"
            | "HailofBulletsEndKnockback" => {
                effect_group.tags.insert("DualPistolsLethalMode");
            }
            "FireDamage" | "FireDamageDoT" | "HailofBulletsFire" => {
                effect_group.tags.insert("DualPistolsFireMode");
            }
            "ColdDamage" | "HailofBulletsCold" => {
                effect_group.tags.insert("DualPistolsColdMode");
            }
            "ToxicDamage" | "HailofBulletsToxic" => {
                effect_group.tags.insert("DualPistolsToxicMode");
            }
            _ => (),
        }
    }
}

/// Modifies `effect` based on the content of `tags`.
fn check_tags_effect(effect: &mut AttribModOutput, tags: &Vec<String>) {
    let tags_str = tags.iter().map(|s| &**s).collect::<Vec<_>>();
    for tag in tags_str {
        match tag {
            "FireDamageDoT" => effect.tick_chance_percent = Some(80.0),
            _ => (),
        }
    }
}

/// Converts the offset of the character attributes to a type
/// which indicates what we're modifying.
/// See Common/entity/character_attribs.h CharacterAttribSet
fn attrib_type(off_aspect: u32, off_attrib: i32) -> Option<AttribType> {
    match off_attrib as usize {
        CharacterAttributes::OFFSET_HELD
        | CharacterAttributes::OFFSET_IMMOBILIZED
        | CharacterAttributes::OFFSET_STUNNED
        | CharacterAttributes::OFFSET_SLEEP
        | CharacterAttributes::OFFSET_KNOCKBACK
        | CharacterAttributes::OFFSET_KNOCKUP
        | CharacterAttributes::OFFSET_TERRORIZED
        | CharacterAttributes::OFFSET_CONFUSED
            if off_aspect != OFFSET_RESIST =>
        {
            Some(AttribType::kAttribType_Mod)
        }
        CharacterAttributes::OFFSET_TELEPORT | CharacterAttributes::OFFSET_REPEL => {
            Some(AttribType::kAttribType_Mod)
        }
        _ => match off_aspect {
            OFFSET_MODIFIERS => Some(AttribType::kAttribType_Cur),
            OFFSET_MAXIMUM => Some(AttribType::kAttribType_Max),
            OFFSET_STRENGTH => Some(AttribType::kAttribType_Str),
            OFFSET_RESIST => Some(AttribType::kAttribType_Res),
            OFFSET_ABSOLUTE => Some(AttribType::kAttribType_Abs),
            _ => {
                debug_assert!(false, "Unknown offset aspect: {}", off_aspect);
                None
            }
        },
    }
}

/// Depending on what the power's doing, the "scaled effect" could actually
/// represent several different things. This attempts to clarify what the
/// scaled effect actually is.
fn get_scaled_effect(
    attrib_mod: &AttribModTemplate,
    attrib_type: &AttribType,
    scaled_value: f32,
) -> Option<ScaledUnit> {
    // duration in seconds
    if matches!(attrib_mod.e_type, ModType::kModType_Duration) {
        return Some(ScaledUnit::DurationSeconds(normalize(scaled_value)));
    }
    // strengths and resists are always percent
    if matches!(
        attrib_type,
        AttribType::kAttribType_Str | AttribType::kAttribType_Res
    ) {
        return Some(ScaledUnit::Percent(normalize(scaled_value * 100.0)));
    }
    // character attributes depend on what we're modifying
    if let Some(attrib) = attrib_mod.p_attrib.get(0) {
        match attrib {
            // Unfortunately there's no standard way to determine these, check the
            // comments on `CharacterAttributes` for some hints.
            SpecialAttrib::kSpecialAttrib_Character(a) => match *a as usize {
                CharacterAttributes::OFFSET_DMG_0..=CharacterAttributes::OFFSET_DMG_19
                | CharacterAttributes::OFFSET_HIT_POINTS
                | CharacterAttributes::OFFSET_ABSORB => {
                    if scaled_value < 0.0 {
                        return Some(ScaledUnit::Damage(normalize(scaled_value.abs())));
                    } else {
                        return Some(ScaledUnit::Healing(normalize(scaled_value)));
                    }
                }
                // Percentage based attributes
                CharacterAttributes::OFFSET_TOHIT
                | CharacterAttributes::OFFSET_DEF_0..=CharacterAttributes::OFFSET_DEF_19
                | CharacterAttributes::OFFSET_DEFENSE..=CharacterAttributes::OFFSET_STEALTH
                | CharacterAttributes::OFFSET_REGENERATION
                    ..=CharacterAttributes::OFFSET_INSIGHT_RECOVERY
                | CharacterAttributes::OFFSET_TELEPORT
                | CharacterAttributes::OFFSET_ACCURACY..=CharacterAttributes::OFFSET_RANGE
                | CharacterAttributes::OFFSET_ELUSIVITY_0
                    ..=CharacterAttributes::OFFSET_ELUSIVITY_BASE => {
                    return Some(ScaledUnit::Percent(normalize(scaled_value * 100.0)));
                }
                CharacterAttributes::OFFSET_ENDURANCE
                    if matches!(
                        attrib_type,
                        AttribType::kAttribType_Cur | AttribType::kAttribType_Mod
                    ) =>
                {
                    return Some(ScaledUnit::Percent(normalize(scaled_value * 100.0)));
                }
                // Distance based attributes
                CharacterAttributes::OFFSET_STEALTH_RADIUS_PVE
                    ..=CharacterAttributes::OFFSET_PERCEPTION_RADIUS
                    if matches!(attrib_type, AttribType::kAttribType_Cur) =>
                {
                    // if current value, they're actually % instead of dist
                    return Some(ScaledUnit::Percent(normalize(scaled_value * 100.0)));
                }
                CharacterAttributes::OFFSET_STEALTH_RADIUS_PVE
                    ..=CharacterAttributes::OFFSET_PERCEPTION_RADIUS => {
                    return Some(ScaledUnit::Distance(normalize(scaled_value)));
                }
                // The following are "boolean".. which actually means that the magnitude
                // of total effects are reduced by the total magnitude of protection, and then if the
                // result is >0, the status is applied to the character.
                CharacterAttributes::OFFSET_CONFUSED
                    ..=CharacterAttributes::OFFSET_ONLY_AFFECTS_SELF
                | CharacterAttributes::OFFSET_KNOCKUP..=CharacterAttributes::OFFSET_REPEL => {
                    return Some(ScaledUnit::Magnitude(normalize(scaled_value)));
                }
                // Any other character attribute is a raw value to be applied.
                _ => return Some(ScaledUnit::Value(normalize(scaled_value))),
            },
            _ => (),
        }
    }
    // anything else is a special case and doesn't use scaling (creating entities, granting powers, etc.)
    None
}
