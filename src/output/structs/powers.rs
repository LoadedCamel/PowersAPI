use super::effects::*;
use super::*;
use crate::structs::*;
use display;
use serde::Serialize;
use std::borrow::Cow;

/// Serializable representation of crowd control flags.
#[derive(Serialize)]
pub struct StatusOptionsOutput {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cast_through: Vec<&'static str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub toggle_ignores: Vec<&'static str>,
}

impl StatusOptionsOutput {
    /// Reads fields from a `BasePower` to create a `StatusOptionsOutput`.
    fn from_base_power(power: &BasePower) -> Self {
        let mut opts = StatusOptionsOutput {
            cast_through: Vec::new(),
            toggle_ignores: Vec::new(),
        };
        if power.b_cast_through_hold {
            opts.cast_through.push("Hold");
        }
        if power.b_cast_through_sleep {
            opts.cast_through.push("Sleep");
        }
        if power.b_cast_through_stun {
            opts.cast_through.push("Stun");
        }
        if power.b_cast_through_terrorize {
            opts.cast_through.push("Terrorize");
        }
        if power.b_toggle_ignore_hold {
            opts.toggle_ignores.push("Hold");
        }
        if power.b_toggle_ignore_sleep {
            opts.cast_through.push("Sleep");
        }
        if power.b_toggle_ignore_stun {
            opts.cast_through.push("Stun");
        }
        opts
    }

    /// Returns true if `opts` doesn't contain any valus.
    fn is_empty(opts: &StatusOptionsOutput) -> bool {
        opts.cast_through.is_empty() && opts.toggle_ignores.is_empty()
    }
}

/// Serializable representation of a power's area of effect and range.
#[derive(Serialize)]
pub struct EffectAreaOutput {
    pub area: Option<&'static str>,
    #[serde(skip_serializing_if = "is_zero")]
    pub max_targets_hit: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_targets_expression: Option<String>,
    #[serde(skip_serializing_if = "not_normal")]
    pub radius_feet: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub jump_distance_feet: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub arc_degrees: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub chain_delay_time: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub range_feet: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub range_feet_secondary: f32,
}

impl EffectAreaOutput {
    /// Reads fields from a `BasePower` to create an `EffectAreaOutput`.
    fn from_base_power(power: &BasePower) -> Self {
        EffectAreaOutput {
            area: Some(power.e_effect_area.get_string()),
            max_targets_hit: power.i_max_targets_hit,
            max_targets_expression: requires_to_string(&power.ppch_max_targets_expr),
            radius_feet: if !matches!(power.e_effect_area, EffectArea::kEffectArea_Chain) {
                normalize(power.f_radius)
            } else {
                0.0
            },
            jump_distance_feet: if matches!(power.e_effect_area, EffectArea::kEffectArea_Chain) {
                normalize(power.f_radius)
            } else {
                0.0
            },
            arc_degrees: normalize(power.f_arc.to_degrees()),
            chain_delay_time: normalize(power.f_chain_delay),
            range_feet: normalize(power.f_range),
            range_feet_secondary: normalize(power.f_range_secondary),
        }
    }
}

/// Serializable representation of a power's activation time and cost.
#[derive(Serialize)]
pub struct ActivationOutput {
    pub cast_time: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub animation_time: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub animation_time_before_hit: f32,
    pub recharge_time: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub interrupt_time: f32,
    #[serde(skip_serializing_if = "not_normal")]
    pub auto_cast_interval: f32,
    pub endurance_cost: f32,
}

impl ActivationOutput {
    /// Reads fields from a `BasePower` to create an `ActivationOutput`.
    fn from_base_power(power: &BasePower) -> Self {
        let mut activate = ActivationOutput {
            cast_time: normalize(power.f_time_to_activate),
            animation_time: 0.0,
            animation_time_before_hit: 0.0,
            recharge_time: normalize(power.f_recharge_time),
            interrupt_time: normalize(power.f_interrupt_time),
            auto_cast_interval: normalize(power.f_activate_period),
            endurance_cost: normalize(power.f_endurance_cost),
        };
        if let Some(fx) = &power.p_fx {
            activate.animation_time = normalize(PowerFX::frames_as_seconds(fx.i_frames_attack));
            activate.animation_time_before_hit =
                normalize(PowerFX::frames_as_seconds(fx.i_frames_before_hit));
        }
        activate
    }
}

#[derive(Serialize)]
pub struct UsageOutput {
    remove_on_limit: bool,
    extend_on_additional_grant: bool,
    #[serde(skip_serializing_if = "is_zero")]
    charges: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_charges_on_extend: Option<i32>,
    #[serde(skip_serializing_if = "not_normal")]
    toggle_usage_time: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    toggle_max_usage_time_on_extend: Option<f32>,
    #[serde(skip_serializing_if = "not_normal")]
    lifetime: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_lifetime_on_extend: Option<f32>,
    #[serde(skip_serializing_if = "not_normal")]
    in_game_lifetime: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_in_game_lifetime_on_extend: Option<f32>,
}

impl UsageOutput {
    fn from_base_power(power: &BasePower) -> Self {
        let mut usage = UsageOutput {
            remove_on_limit: power.b_destroy_on_limit,
            extend_on_additional_grant: power.b_stacking_usage,
            charges: power.i_num_charges,
            max_charges_on_extend: None,
            toggle_usage_time: power.f_usage_time,
            toggle_max_usage_time_on_extend: None,
            lifetime: power.f_lifetime,
            max_lifetime_on_extend: None,
            in_game_lifetime: power.f_lifetime_in_game,
            max_in_game_lifetime_on_extend: None,
        };
        if usage.extend_on_additional_grant {
            if power.i_max_num_charges > 0 {
                usage.max_charges_on_extend = Some(power.i_max_num_charges);
            }
            if power.f_max_usage_time.is_normal() {
                usage.toggle_max_usage_time_on_extend = Some(power.f_max_usage_time);
            }
            if power.f_max_lifetime.is_normal() {
                usage.max_lifetime_on_extend = Some(power.f_max_lifetime);
            }
            if power.f_max_lifetime_in_game.is_normal() {
                usage.max_in_game_lifetime_on_extend = Some(power.f_max_lifetime_in_game);
            }
        }
        usage
    }

    fn is_empty(usage: &UsageOutput) -> bool {
        !(usage.charges > 0
            || usage.toggle_usage_time.is_normal()
            || usage.lifetime.is_normal()
            || usage.in_game_lifetime.is_normal())
    }
}

#[derive(Serialize)]
pub struct PowerRedirectOutput {
    pub name: Option<NameKey>,
    pub fallback: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PowerRedirectOutput {
    fn from_power_redirect(redirect: &PowerRedirect, config: &PowersConfig) -> Self {
        PowerRedirectOutput {
            name: redirect.pch_name.clone(),
            fallback: redirect.ppch_requires.len() == 0
                || (redirect.ppch_requires.len() == 1 && redirect.ppch_requires[0] == "1"),
            requires: requires_to_string(&redirect.ppch_requires),
            url: make_power_ref_url(redirect.pch_name.as_ref(), config),
        }
    }
}

/// Serializable representation of a power.
#[derive(Serialize)]
pub struct PowerOutput {
    pub name: Option<NameKey>,
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_short_help: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub display_info: HashMap<&'static str, Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attack_types: Vec<Option<Cow<'static, str>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enhancements_allowed: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enhancement_set_categories_allowed: Vec<String>,
    pub available_at_level: i32,
    pub auto_issue: bool,
    pub power_type: Option<&'static str>,
    pub accuracy: f32,
    pub effect_area: EffectAreaOutput,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_type_tags: Vec<&'static str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_type_secondary_tags: Vec<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_target_type: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_target_type_secondary: Option<&'static str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_auto_hit_tags: Vec<Vec<&'static str>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub display_target_auto_hit: Vec<&'static str>,
    pub requires_line_of_sight: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modes_required: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modes_disallowed: Vec<String>,
    #[serde(skip_serializing_if = "StatusOptionsOutput::is_empty")]
    pub status_interaction: StatusOptionsOutput,
    pub activate: ActivationOutput,
    #[serde(skip_serializing_if = "UsageOutput::is_empty")]
    pub usage: UsageOutput,
    pub effect_groups: Vec<EffectGroupOutput>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub redirects: Vec<PowerRedirectOutput>,
}

impl PowerOutput {
    /// Converts a `BasePower` to a `PowerOutput` ready for serialization.
    pub fn from_base_power(
        power: &BasePower,
        attrib_names: &AttribNames,
        config: &PowersConfig,
    ) -> Self {
        let mut pwr = PowerOutput {
            name: power.pch_full_name.clone(),
            display_name: power.pch_display_name.clone(),
            icon: None,
            display_help: power.pch_display_help.clone(),
            display_short_help: power.pch_display_short_help.clone(),
            display_info: HashMap::new(),
            requires: requires_to_string(&power.ppch_buy_requires),
            attack_types: Vec::new(),
            enhancements_allowed: Vec::new(),
            enhancement_set_categories_allowed: power
                .enhancement_set_categories_allowed
                .iter()
                .cloned()
                .collect(),
            available_at_level: 0,
            auto_issue: power.b_auto_issue,
            power_type: Some(power.e_type.get_string()),
            accuracy: normalize(power.f_accuracy),
            effect_area: EffectAreaOutput::from_base_power(power),
            target_type_tags: power.e_target_type.get_strings(),
            target_type_secondary_tags: power.e_target_type_secondary.get_strings(),
            display_target_type: display::describe_target_type(&power.e_target_type),
            display_target_type_secondary: display::describe_target_type(
                &power.e_target_type_secondary,
            ),
            target_auto_hit_tags: Vec::new(),
            display_target_auto_hit: Vec::new(),
            requires_line_of_sight: match power.e_target_visibility {
                TargetVisibility::kTargetVisibility_LineOfSight => true,
                TargetVisibility::kTargetVisibility_None => false,
            },
            modes_required: Vec::new(),
            modes_disallowed: Vec::new(),
            status_interaction: StatusOptionsOutput::from_base_power(power),
            activate: ActivationOutput::from_base_power(power),
            usage: UsageOutput::from_base_power(power),
            effect_groups: Vec::new(),
            redirects: Vec::new(),
        };
        // power icon
        if let Some(icon) = power.pch_icon_name.as_ref() {
            if let Some(assets_config) = config.assets.as_ref() {
                pwr.icon = Some(format_power_icon_to_asset(icon, assets_config));
            } else {
                pwr.icon = Some(icon.to_owned());
            }
        }
        // attack types
        for atk in &power.pe_attack_types {
            pwr.attack_types
                .push(character_attrib_to_string(atk, attrib_names));
        }
        // enhancements
        for enh in &power.pe_boosts_allowed {
            if let Some(enh_allowed) = boost_attrib_to_string(enh, attrib_names) {
                pwr.enhancements_allowed.push(enh_allowed);
            }
        }
        // disallowed/required modes
        for mode in &power.pe_modes_required {
            if let Some(m) = mode_attrib_to_string(mode, attrib_names) {
                pwr.modes_required.push(m);
            }
        }
        for mode in &power.pe_modes_disallowed {
            if let Some(m) = mode_attrib_to_string(mode, attrib_names) {
                pwr.modes_disallowed.push(m);
            }
        }
        // auto hit tags
        for target in &power.p_auto_hit {
            if !matches!(target, TargetType::kTargetType_None) {
                pwr.target_auto_hit_tags.push(target.get_strings());
                if let Some(s) = display::describe_target_type(target) {
                    pwr.display_target_auto_hit.push(s);
                }
            }
        }
        // filter archetypes to only those that can purchase this power, if necessary
        let archetypes = filter_archetypes_pwr(power, &power.archetypes);
        // effect groups
        for effect_group in &power.pp_effects {
            pwr.effect_groups.push(EffectGroupOutput::from_effect_group(
                effect_group,
                attrib_names,
                power,
                &archetypes,
                config,
            ));
        }
        // redirected powers
        for redirect in &power.pp_redirect {
            pwr.redirects
                .push(PowerRedirectOutput::from_power_redirect(&redirect, config));
        }
        // set display information
        display::describe_power(&mut pwr, &power, attrib_names);
        pwr
    }
}

/// Filters the archetypes vector based on any purchase requirements specified in `power`.
/// If `power` has no requirements, all archetypes passed in will be returned.
fn filter_archetypes_pwr(power: &BasePower, archetypes: &Vec<Rc<Archetype>>) -> Vec<Rc<Archetype>> {
    if power
        .ppch_buy_requires
        .iter()
        .any(|rule| rule == "$archetype")
    {
        // This is naive, but it looks like all of the rules are written in a positive-test so we'll see if it works out
        archetypes
            .iter()
            .filter(|at| {
                if let Some(class_key) = &at.class_key {
                    power
                        .ppch_buy_requires
                        .iter()
                        .any(|rule| class_key == &rule[..])
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

/// Creates a URL link to be used inside a power to another power set in an external file.
/// `power_ref` must have at least 2 parts (category & set) or this will return `None`.
pub fn make_power_ref_url(power_ref: Option<&NameKey>, config: &PowersConfig) -> Option<String> {
    if power_ref.is_none() {
        return None;
    }
    let name_parts = power_ref.unwrap().split();
    if name_parts.len() < 2 {
        return None;
    }
    let mut url = String::new();
    if let Some(base_url) = config.base_json_url.as_ref() {
        url.push_str(base_url);
    } else {
        url.push_str("../../");
    }
    url.push_str(&make_file_name(name_parts[0]));
    url.push(URL_SEP);
    url.push_str(&make_file_name(name_parts[1]));
    url.push(URL_SEP);
    if config.base_json_url.is_none() {
        url.push_str(JSON_FILE);
    }
    Some(url)
}
