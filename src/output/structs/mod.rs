mod display;
mod effects;
mod powers;

use super::{make_file_name, JSON_FILE};
use crate::structs::config::{AssetsConfig, PowersConfig};
use crate::structs::*;
use powers::PowerOutput;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

/// Used when joining parts of an URL together.
const URL_SEP: char = '/';

/// Used in attribute name tables.
const ORIGINS_SIZE: usize = 5;

/// Common fields added to other structs.
#[derive(Serialize)]
pub struct HeaderOutput {
    pub issue: Option<String>,
    pub source: Option<String>,
    pub extract_date: Option<String>,
}

impl HeaderOutput {
    /// Creates a `HeaderOutput` from a `PowersConfig`.
    fn from_config(config: &PowersConfig) -> Self {
        HeaderOutput {
            issue: Some(config.issue.clone()),
            source: Some(config.source.clone()),
            extract_date: Some(config.extract_date.unwrap().to_rfc3339()),
        }
    }
}

/// Additional fields to include in `ArchetypeOutput` if we're dumping a full
/// view of the archetypes.
#[derive(Serialize)]
pub struct ExtendedArchetypeOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_short_help: Option<String>,
    allowed_origins: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    restrictions: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    level_up_respecs: Vec<i32>,
    primary_category: Option<NameKey>,
    secondary_category: Option<NameKey>,
}

impl ExtendedArchetypeOutput {
    /// Creates an `ExtendedArchetypeOutput` from an `Archetype`.
    fn from_archetype(at: &Archetype) -> Self {
        ExtendedArchetypeOutput {
            display_help: at.pch_display_help.clone(),
            display_short_help: at.pch_display_short_help.clone(),
            allowed_origins: at.ppch_allowed_origin_names.clone(),
            restrictions: at.ppch_special_restrictions.clone(),
            level_up_respecs: at.pi_level_up_respecs.clone(),
            primary_category: at.pch_primary_category.clone(),
            secondary_category: at.pch_secondary_category.clone(),
        }
    }
}

/// Serializable representation of an archetype.
#[derive(Serialize)]
pub struct ArchetypeOutput {
    pub name: Option<String>,
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_or_secondary: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended: Option<ExtendedArchetypeOutput>,
}

impl ArchetypeOutput {
    /// Creates an `ArchetypeOutput` from an `Archetype`.
    fn from_archetype(
        at: &Archetype,
        pri_sec: &PrimarySecondary,
        extended: bool,
        config: &PowersConfig,
    ) -> Self {
        let mut at_out = ArchetypeOutput {
            name: at.pch_name.clone(),
            display_name: at.pch_display_name.clone(),
            icon: None,
            primary_or_secondary: match pri_sec {
                PrimarySecondary::Secondary => Some(String::from("Secondary")),
                PrimarySecondary::Primary => Some(String::from("Primary")),
                PrimarySecondary::None => None,
            },
            extended: if extended {
                Some(ExtendedArchetypeOutput::from_archetype(at))
            } else {
                None
            },
        };
        if let Some(assets_config) = &config.assets {
            if let Some(icon) = &at.pch_icon {
                at_out.icon = Some(format_at_icon_to_asset(icon, assets_config));
            }
        }
        at_out
    }
}

#[derive(Serialize)]
pub struct ArchetypesOutput {
    #[serde(flatten)]
    pub header: HeaderOutput,
    pub archetypes: Vec<ArchetypeOutput>,
}

impl ArchetypesOutput {
    /// Creates an `ArchetypesOuput` from an array of `Archetype`.
    pub fn from_archetypes(ats: &Keyed<Archetype>, config: &PowersConfig) -> Self {
        let mut ats_out = ArchetypesOutput {
            header: HeaderOutput::from_config(config),
            archetypes: Vec::new(),
        };
        for at in ats.values() {
            ats_out.archetypes.push(ArchetypeOutput::from_archetype(
                at,
                &PrimarySecondary::None,
                true,
                config,
            ));
        }
        ats_out
    }
}

/// Serializable representation of a power category in the root index.
#[derive(Serialize)]
pub struct RootPowerCategory {
    pub name: Option<NameKey>,
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archetype: Option<ArchetypeOutput>,
    pub url: String,
}

/// Serializable representation of the root index.
#[derive(Serialize)]
pub struct RootOutput {
    #[serde(flatten)]
    pub header: HeaderOutput,
    pub archetypes: String,
    pub power_categories: Vec<RootPowerCategory>,
}

impl RootOutput {
    /// Converts a set of `PowerCategory` to a `RootOutput` ready for serialization.
    ///
    /// Arguments:
    ///
    /// * `power_categories` - A `Vec<Rc<PowerCategory>>`.
    /// * `config` - Configuration information.
    ///
    /// Returns:
    ///
    /// A `RootOutput`.
    pub fn from_power_categories(
        power_categories: &Vec<Rc<PowerCategory>>,
        config: &PowersConfig,
    ) -> Self {
        let mut at_url = String::new();
        if let Some(base_url) = config.base_json_url.as_ref() {
            at_url.push_str(base_url);
        }
        at_url.push_str(&make_file_name("archetypes"));
        at_url.push(URL_SEP);
        if config.base_json_url.is_none() {
            at_url.push_str(JSON_FILE);
        }
        let mut root = RootOutput {
            header: HeaderOutput::from_config(config),
            archetypes: at_url,
            power_categories: Vec::new(),
        };
        for pcat in power_categories {
            if !pcat.top_level || !pcat.include_in_output {
                continue;
            }
            let mut url = String::new();
            if let Some(base_url) = config.base_json_url.as_ref() {
                url.push_str(base_url);
            }
            if let Some(pcat_name) = pcat.pch_name.as_ref() {
                url.push_str(&make_file_name(pcat_name.get()));
                url.push(URL_SEP);
                if config.base_json_url.is_none() {
                    url.push_str(JSON_FILE);
                }
                let mut rpc = RootPowerCategory {
                    name: Some(pcat_name.clone()),
                    display_name: pcat.pch_display_name.clone(),
                    archetype: None,
                    url,
                };
                if pcat.archetypes.len() == 1 {
                    // if there's only 1 archetype attached, then this is a group of sets intended for that archetype
                    rpc.archetype = Some(ArchetypeOutput::from_archetype(
                        &pcat.archetypes[0],
                        &pcat.pri_sec,
                        false,
                        config,
                    ));
                }
                root.power_categories.push(rpc);
            }
        }
        root
    }
}

/// Serializable representation of a power set in a power category.
#[derive(Serialize)]
pub struct PowerCategoryPowerSetOutput {
    pub name: Option<NameKey>,
    pub display_name: Option<String>,
    pub url: Option<String>,
}

/// Serializable representation of a power category.
#[derive(Serialize)]
pub struct PowerCategoryOutput {
    #[serde(flatten)]
    pub header: HeaderOutput,
    pub name: Option<NameKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archetype: Option<ArchetypeOutput>,
    pub power_sets: Vec<PowerCategoryPowerSetOutput>,
}

impl PowerCategoryOutput {
    /// Converts a `PowerCategory` to a `PowerCategoryOutput` ready for serialization.
    ///
    /// Arguments:
    ///
    /// * `power_category` - A `PowerCategory`.
    /// * `config` - Configuration information.
    ///
    /// Returns:
    ///
    /// A `PowerCategoryOutput`.
    pub fn from_power_category(power_category: &PowerCategory, config: &PowersConfig) -> Self {
        let mut pcat = PowerCategoryOutput {
            header: HeaderOutput::from_config(config),
            name: power_category.pch_name.clone(),
            archetype: None,
            power_sets: Vec::new(),
        };
        if power_category.archetypes.len() == 1 {
            // if there's only 1 archetype attached, then this is a group of sets intended for that archetype
            pcat.archetype = Some(ArchetypeOutput::from_archetype(
                &power_category.archetypes[0],
                &power_category.pri_sec,
                false,
                config,
            ));
        }
        for pset in &power_category.pp_power_sets {
            if !pset.include_in_output
            {
                continue;
            }
            let mut url = String::new();
            if let Some(base_url) = config.base_json_url.as_ref() {
                url.push_str(base_url);
            }
            if config.base_json_url.is_some() {
                if let Some(pcat_name) = &pcat.name {
                    url.push_str(&make_file_name(pcat_name.get()));
                }
                url.push(URL_SEP);
            }
            if let Some(name) = &pset.pch_name {
                url.push_str(&make_file_name(name));
            }
            url.push(URL_SEP);
            if config.base_json_url.is_none() {
                url.push_str(JSON_FILE);
            }
            pcat.power_sets.push(PowerCategoryPowerSetOutput {
                name: pset.pch_full_name.clone(),
                display_name: pset.pch_display_name.clone(),
                url: Some(url),
            });
        }
        pcat
    }
}

/// Serializable representation of a power set.
#[derive(Serialize)]
pub struct PowerSetOutput {
    #[serde(flatten)]
    header: HeaderOutput,
    name: Option<NameKey>,
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specialize_at_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specialize_requires: Option<String>,
    show_in_inventory: Option<String>,
    show_in_power_management: bool,
    show_in_power_info: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_buy_requires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_set_buy_requires_failed: Option<String>,
    ordered_power_names: Vec<NameKey>,
    powers: Vec<PowerOutput>,
}

impl PowerSetOutput {
    /// Converts a `BasePowerSet` to a `PowerSetOutput` ready for serialization.
    ///
    /// Arguments:
    ///
    /// * `power_set` - A `BasePowerSet`.
    /// * `attrib_names` - An `AttribNames`.
    /// * `config` - Configuration information.
    ///
    /// Returns:
    ///
    /// A `PowerSetOutput`.
    pub fn from_base_power_set(
        power_set: &BasePowerSet,
        attrib_names: &AttribNames,
        config: &PowersConfig,
    ) -> Self {
        let mut pset = PowerSetOutput {
            header: HeaderOutput::from_config(config),
            name: power_set.pch_full_name.clone(),
            display_name: power_set.pch_display_name.clone(),
            display_help: power_set.pch_display_help.clone(),
            icon: None,
            specialize_at_level: None,
            specialize_requires: requires_to_string(&power_set.pp_specialize_requires),
            show_in_inventory: match power_set.e_show_in_inventory {
                ShowPowerSetting::kShowPowerSetting_Always => Some(String::from("Always")),
                ShowPowerSetting::kShowPowerSetting_Default => Some(String::from("Show")),
                ShowPowerSetting::kShowPowerSetting_IfOwned => Some(String::from("IfOwned")),
                ShowPowerSetting::kShowPowerSetting_IfUsable => Some(String::from("IfUsable")),
                ShowPowerSetting::kShowPowerSetting_Never => Some(String::from("Never")),
            },
            show_in_power_management: power_set.b_show_in_manage,
            show_in_power_info: power_set.b_show_in_info,
            set_buy_requires: requires_to_string(&power_set.ppch_set_buy_requires),
            display_set_buy_requires_failed: None,
            ordered_power_names: Vec::new(),
            powers: Vec::new(),
        };
        // specialization info
        if power_set.i_specialize_at > 0 {
            pset.specialize_at_level = Some(power_set.i_specialize_at + 1);
        }
        // purchase requirements
        if power_set.ppch_set_buy_requires.len() > 0 {
            pset.display_set_buy_requires_failed =
                power_set.pch_set_buy_requires_failed_text.clone();
        }
        // map individual powers
        for power in &power_set.pp_powers {
            // skip disabled powers
            if power.include_in_output {
                pset.powers.push(PowerOutput::from_base_power(
                    power,
                    attrib_names,
                    config,
                ));
            }
        }
        // copy minimum levels
        let mut powers_to_levels = HashMap::new();
        power_set
            .pp_power_names
            .iter()
            .zip(&power_set.pi_available)
            .for_each(|(pwr_name, level)| {
                powers_to_levels.insert(pwr_name.clone(), *level);
            });
        for power in &mut pset.powers {
            if let Some(power_name) = &power.name {
                if let Some(level) = powers_to_levels.get(power_name) {
                    power.available_at_level = *level + 1;
                }
            }
            // now that we have minimum level info, we can add display info for available level
            power.display_info.insert(
                "Available Level",
                Cow::Owned(power.available_at_level.to_string()),
            );
        }
        // power set icon
        if let Some(s) = power_set.pch_icon_name.as_ref() {
            if config.assets.is_some() {
                // there's not really power set icons, we use the icon from the first power
                if let Some(power_name) = power_set.pp_power_names.get(0) {
                    if let Some(first_power) = pset
                        .powers
                        .iter()
                        .find(|pwr| pwr.name.as_ref().unwrap() == power_name)
                    {
                        pset.icon = first_power.icon.clone();
                    }
                }
            } else {
                pset.icon = Some(s.to_owned());
            }
        }
        // ordered powers list
        pset.ordered_power_names = power_set
            .pp_power_names
            .iter()
            .filter(|pname| {
                pset.powers
                    .iter()
                    .any(|pwr| pwr.name.as_ref() == Some(*pname))
            })
            .cloned()
            .collect();
        // sort powers
        pset.powers
            .sort_by(|a, b| a.available_at_level.cmp(&b.available_at_level));
        pset
    }
}

/// Rewrites an icon name from a .bin file into a file name with new extension and
/// also calculates the MD5 of the name.
fn make_icon_name_and_digest(icon: &str, ext: &str) -> (String, md5::Digest) {
    let mut filename = String::new();
    let offset = icon.find('.').unwrap_or(icon.len());
    for c in icon[..offset].chars() {
        for cl in c.to_lowercase() {
            filename.push(cl);
        }
    }
    filename.push_str(ext);

    let digest = md5::compute(filename.bytes().collect::<Vec<u8>>());

    (filename, digest)
}

/// Formats an archetype icon filename into a full URL.
fn format_at_icon_to_asset(icon: &str, assets: &AssetsConfig) -> String {
    let mut url = String::new();
    url.push_str(&assets.base_asset_url);
    let (filename, digest) = make_icon_name_and_digest(icon, &assets.ext);

    let url_path = assets
        .archetype_icon_format
        .replace("{md5}", &format!("{:02x}", digest[0]))
        .replace("{icon}", &filename);
    url.push_str(&url_path);
    url
}

/// Formats a power icon filename into a full URL.
fn format_power_icon_to_asset(icon: &str, assets: &AssetsConfig) -> String {
    let mut url = String::new();
    url.push_str(&assets.base_asset_url);
    let (filename, digest) = make_icon_name_and_digest(icon, &assets.ext);

    let url_path = assets
        .powers_icon_format
        .replace("{md5}", &format!("{:02x}", digest[0]))
        .replace("{icon}", &filename);

    url.push_str(&url_path);
    url
}

/// Returns true if `val` is 0.
fn is_zero(val: &i32) -> bool {
    *val == 0
}

/// Returns true if `val` is 0, infinite, or NaN.
fn not_normal(val: &f32) -> bool {
    !val.is_normal()
}

/// Trims `val` to 2 decimal places via rounding.
fn normalize(val: f32) -> f32 {
    if val.is_normal() {
        (val * 100.0).round() / 100.0
    } else {
        val
    }
}

/// Trims `val` to 4 decimal places via rounding.
fn normalize4(val: f32) -> f32 {
    if val.is_normal() {
        (val * 10000.0).round() / 10000.0
    } else {
        val
    }
}

/// Converts a stacked requirements expression into a concise string representation.
fn requires_to_string(requires: &Vec<String>) -> Option<String> {
    if requires.len() == 1 && requires[0] == "1" {
        // always evaluates to true, dump it
        return None;
    }
    let mut iter = requires.iter().rev();
    if let Some(expression) = requires_to_string_inner(&mut iter) {
        // remove excess parens
        if expression.starts_with('(') && expression.ends_with(')') {
            Some(expression[1..expression.len() - 1].to_owned())
        } else {
            Some(expression)
        }
    } else {
        None
    }
}

/// Used by `requires_to_string`, don't call this directly.
fn requires_to_string_inner<'a, I>(requires: &mut I) -> Option<String>
where
    I: Iterator<Item = &'a String>,
{
    if let Some(token) = requires.next() {
        match token.as_ref() {
            "!" => {
                // unary operators
                let mut expression = String::new();
                expression.push_str(token);
                if let Some(arg) = requires_to_string_inner(requires) {
                    expression.push_str(&arg);
                } else {
                    debug_assert!(false, "Unary operator {} should have 1 argument", token);
                }
                return Some(expression);
            }
            "==" | "eq" | "||" | "&&" | "/" | "+" | "-" | "*" | "<" | "<=" | ">" | ">=" => {
                // binary operators/functions
                let mut expression = String::new();
                expression.push('(');
                let arg2 = requires_to_string_inner(requires);
                let arg1 = requires_to_string_inner(requires);
                debug_assert!(
                    arg2.is_some() & arg1.is_some(),
                    "Binary operator {} should have 2 arguments",
                    token
                );
                if let Some(arg) = arg1 {
                    expression.push_str(&arg);
                }
                expression.push(' ');
                // internally, 'eq' is actually a string comparison function
                if token == "eq" {
                    expression.push_str("==");
                } else {
                    expression.push_str(token);
                }
                expression.push(' ');
                if let Some(arg) = arg2 {
                    expression.push_str(&arg);
                }
                expression.push(')');
                return Some(expression);
            }
            "drop" | "dup" | "rand" => {
                // no-argument functions
                return Some(format!("{}()", token));
            }
            "negate" => {
                // single-argument functions
                let mut expression = String::new();
                expression.push_str(token);
                expression.push('(');
                if let Some(arg) = requires_to_string_inner(requires) {
                    expression.push_str(&arg);
                } else {
                    debug_assert!(false, "{} function should have 1 argument", token);
                }
                expression.push(')');
                return Some(expression);
            }
            "minmax" => {
                // minmax function - minmax(val,min,max)
                let mut expression = String::new();
                expression.push_str(token);
                expression.push('(');
                let max = requires_to_string_inner(requires);
                let min = requires_to_string_inner(requires);
                let val = requires_to_string_inner(requires);
                debug_assert!(
                    max.is_some() && min.is_some() && val.is_some(),
                    "{} function should have 3 arguments",
                    token
                );
                if let Some(arg) = val {
                    expression.push_str(&arg);
                }
                expression.push_str(", ");
                if let Some(arg) = min {
                    expression.push_str(&arg);
                }
                expression.push_str(", ");
                if let Some(arg) = max {
                    expression.push_str(&arg);
                }
                expression.push(')');
                return Some(expression);
            }
            "source.MapTeamArea>" => {
                // weird exception to below
                return Some(token[0..token.len() - 1].to_owned());
            }
            _ => {
                if token.ends_with('>') {
                    // struct pointer
                    let mut combined = token.to_owned();
                    if let Some(next_token) = requires.next() {
                        combined.push_str(next_token);
                    }
                    return Some(combined);
                } else if token.ends_with('?') {
                    // function
                    let mut combined = token.to_owned();
                    combined.push('(');
                    // this is probably inaccurate
                    if !(token.find(".is").is_some() || token.find(".Is").is_some())
                        && !(token.starts_with("is") || token.starts_with("Is"))
                    {
                        if let Some(next_token) = requires.next() {
                            combined.push_str(next_token);
                        }
                    }
                    combined.push(')');
                    return Some(combined);
                } else {
                    // some other token
                    return Some(token.clone());
                }
            }
        }
    }
    None
}

/// Converts a boost attribute to a human readable string.
fn boost_attrib_to_string(attrib: &SpecialAttrib, attrib_names: &AttribNames) -> Option<String> {
    if let SpecialAttrib::kSpecialAttrib_Character(a) = attrib {
        match *a as usize {
            i @ ORIGINS_SIZE..=99 => {
                // Why do we subtract ORIGINS_SIZE? Good question! Check this lovely note I found in the code:
                //
                // > mw 3.10.06 added guard here because it's everywhere else this calc is done,
                // > and there's reported crash here that I can't repro, so I'm doing this and hoping for the best
                // > (subtracting off the number of origins seems insane and neither Jered nor CW can remember why its needed)
                //
                // Coding is weird, folks :)
                //
                // Follow up: It's possible the weird 4..3..2..1..0 sequence seen in several powers (such as incarnates) is a
                // reference to those origins that's been trimmed out here.
                if let Some(name) = attrib_names.pp_boost.get(i - ORIGINS_SIZE) {
                    return name.pch_display_name.clone();
                }
            }
            _ => (),
        }
    }
    None
}

/// Converts a mode attribute to a human readable string.
fn mode_attrib_to_string(attrib: &SpecialAttrib, attrib_names: &AttribNames) -> Option<String> {
    if let SpecialAttrib::kSpecialAttrib_Character(a) = attrib {
        if let Some(name) = attrib_names.pp_mode.get(*a as usize) {
            return name.pch_name.clone();
        }
    }
    None
}

/// Converts a character attribute to a human readable string.
fn character_attrib_to_string(
    attrib: &SpecialAttrib,
    attrib_names: &AttribNames,
) -> Option<Cow<'static, str>> {
    macro_rules! retopt {
        ($string:literal) => {
            return Some(Cow::Borrowed($string));
        };
    }
    match attrib {
        SpecialAttrib::kSpecialAttrib_UNSET => return None,
        SpecialAttrib::kSpecialAttrib_Character(a) => {
            match *a as usize {
                // The below entries are divided by 4 to get the name because they originally refer to
                // memory offsets into the C structs.
                // ppDamage starts at offset OFFSET_DMG_0
                i @ CharacterAttributes::OFFSET_DMG_0..=CharacterAttributes::OFFSET_DMG_19 => {
                    if let Some(name) = attrib_names.pp_damage.get(i / 4) {
                        return Some(Cow::Owned(format!(
                            "{}_Dmg",
                            name.pch_display_name.as_ref().unwrap()
                        )));
                    }
                }
                // There are a few different versions of these strings stored for use in the UI
                // but I prefer to use my own.
                CharacterAttributes::OFFSET_HIT_POINTS => retopt!("HitPoints"),
                CharacterAttributes::OFFSET_ABSORB => retopt!("Absorb"),
                CharacterAttributes::OFFSET_ENDURANCE => retopt!("Endurance"),
                CharacterAttributes::OFFSET_INSIGHT => retopt!("Insight"),
                CharacterAttributes::OFFSET_RAGE => retopt!("Rage"),
                CharacterAttributes::OFFSET_TOHIT => retopt!("ToHit"),
                // ppDefense starts at offset OFFSET_DEF_0
                i @ CharacterAttributes::OFFSET_DEF_0..=CharacterAttributes::OFFSET_DEF_19 => {
                    if let Some(name) = attrib_names
                        .pp_defense
                        .get((i - CharacterAttributes::OFFSET_DEF_0) / 4)
                    {
                        return Some(Cow::Owned(format!(
                            "{}_Def",
                            name.pch_display_name.as_ref().unwrap()
                        )));
                    }
                }
                CharacterAttributes::OFFSET_DEFENSE => retopt!("Defense"),
                CharacterAttributes::OFFSET_RUNNING_SPEED => retopt!("RunningSpeed"),
                CharacterAttributes::OFFSET_FLYING_SPEED => retopt!("FlyingSpeed"),
                CharacterAttributes::OFFSET_SWIMMING_SPEED => retopt!("SwimmingSpeed"),
                CharacterAttributes::OFFSET_JUMPING_SPEED => retopt!("JumpingSpeed"),
                CharacterAttributes::OFFSET_JUMP_HEIGHT => retopt!("JumpHeight"),
                CharacterAttributes::OFFSET_MOVEMENT_CONTROL => retopt!("MovementControl"),
                CharacterAttributes::OFFSET_MOVEMENT_FRICTION => retopt!("MovementFriction"),
                CharacterAttributes::OFFSET_STEALTH => retopt!("Stealth"),
                CharacterAttributes::OFFSET_STEALTH_RADIUS_PVE => retopt!("StealthRadius_PVE"),
                CharacterAttributes::OFFSET_STEALTH_RADIUS_PVP => retopt!("StealthRadius_PVP"),
                CharacterAttributes::OFFSET_PERCEPTION_RADIUS => retopt!("PerceptionRadius"),
                CharacterAttributes::OFFSET_REGENERATION => retopt!("Regeneration"),
                CharacterAttributes::OFFSET_RECOVERY => retopt!("Recovery"),
                CharacterAttributes::OFFSET_INSIGHT_RECOVERY => retopt!("InsightRecovery"),
                CharacterAttributes::OFFSET_THREAT_LEVEL => retopt!("ThreatLevel"),
                CharacterAttributes::OFFSET_TAUNT => retopt!("Taunt"),
                CharacterAttributes::OFFSET_PLACATE => retopt!("Placate"),
                CharacterAttributes::OFFSET_CONFUSED => retopt!("Confused"),
                CharacterAttributes::OFFSET_AFRAID => retopt!("Afraid"),
                CharacterAttributes::OFFSET_TERRORIZED => retopt!("Terrorized"),
                CharacterAttributes::OFFSET_HELD => retopt!("Held"),
                CharacterAttributes::OFFSET_IMMOBILIZED => retopt!("Immobilized"),
                CharacterAttributes::OFFSET_STUNNED => retopt!("Stunned"),
                CharacterAttributes::OFFSET_SLEEP => retopt!("Sleep"),
                CharacterAttributes::OFFSET_FLY => retopt!("Fly"),
                CharacterAttributes::OFFSET_JUMP_PACK => retopt!("Jump Pack"),
                CharacterAttributes::OFFSET_TELEPORT => retopt!("Teleport"),
                CharacterAttributes::OFFSET_UNTOUCHABLE => retopt!("Untouchable"),
                CharacterAttributes::OFFSET_INTANGIBLE => retopt!("Intangible"),
                CharacterAttributes::OFFSET_ONLY_AFFECTS_SELF => retopt!("OnlyAffectsSelf"),
                CharacterAttributes::OFFSET_EXPERIENCE_GAIN => retopt!("ExperienceGain"),
                CharacterAttributes::OFFSET_INFLUENCE_GAIN => retopt!("InfluenceGain"),
                CharacterAttributes::OFFSET_PRESTIGE_GAIN => retopt!("PrestigeGain"),
                CharacterAttributes::OFFSET_EVADE => retopt!("Evade"),
                CharacterAttributes::OFFSET_KNOCKUP => retopt!("Knockup"),
                CharacterAttributes::OFFSET_KNOCKBACK => retopt!("Knockback"),
                CharacterAttributes::OFFSET_REPEL => retopt!("Repel"),
                CharacterAttributes::OFFSET_ACCURACY => retopt!("Accuracy"),
                CharacterAttributes::OFFSET_RADIUS => retopt!("Radius"),
                CharacterAttributes::OFFSET_ARC => retopt!("Arc"),
                CharacterAttributes::OFFSET_RANGE => retopt!("Range"),
                CharacterAttributes::OFFSET_TIME_TO_ACTIVATE => retopt!("TimeToActivate"),
                CharacterAttributes::OFFSET_RECHARGE_TIME => retopt!("RechargeTime"),
                CharacterAttributes::OFFSET_INTERRUPT_TIME => retopt!("InterruptTime"),
                CharacterAttributes::OFFSET_ENDURANCE_DISCOUNT => retopt!("EnduranceDiscount"),
                CharacterAttributes::OFFSET_INSIGHT_DISCOUNT => retopt!("InsightDiscount"),
                CharacterAttributes::OFFSET_METER => retopt!("Meter"),
                // ppElusivity starts at offset OFFSET_ELUSIVITY_0
                i
                @
                CharacterAttributes::OFFSET_ELUSIVITY_0
                    ..=CharacterAttributes::OFFSET_ELUSIVITY_19 => {
                    if let Some(name) = attrib_names
                        .pp_elusivity
                        .get((i - CharacterAttributes::OFFSET_ELUSIVITY_0) / 4)
                    {
                        return Some(Cow::Owned(format!(
                            "{}_Elusivity",
                            name.pch_display_name.as_ref().unwrap()
                        )));
                    }
                }
                CharacterAttributes::OFFSET_ELUSIVITY_BASE => retopt!("ElusivityBase"),
                _ => (),
            }
        }
        _ => return Some(Cow::Borrowed(attrib.get_string())),
    }
    debug_assert!(false, "Unampped attrib: {:?}", attrib);
    None
}
