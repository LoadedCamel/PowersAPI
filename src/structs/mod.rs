//! This module contains representations of the different data structs that can be read
//! from the bins supported by this tool. These are based on the ones found in the CoH codebase,
//! but they are not 1-for-1. (I discard any fields that aren't actually serialized in the bins,
//! and in some case add new fields for convenience.)
//!
//! In particular, I've preserved comments where they were available to describe what the
//! fields were for, and in some places edited them for clarity. Additionally, I preserved
//! the original type notations so it's easy to tell what they are at a glance (and changed
//! or added some in places for consistency):
//!
//! * `f_` - A 32-bit floating point. (C: `float`, Rust: `f32`)
//! * `pf_` - An array of 32-bit floating points. (C: `float *`, Rust: `Vec<f32>`)
//! * `i_` - A 32-bit signed integer. (C: `int`, Rust: `i32`)
//! * `pi_` - An array of 32-bit signed integers. (C: `int *`, Rust: `Vec<i32>`)
//! * `ul_` - An unsigned 32-bit integer. (C: `unsigned long int`, Rust: `u32`)
//! * `b_` - A Boolean value. (C: `bool`, Rust: `bool`)
//! * `pch_` - A string, potentially NUL. (C: `char *`, Rust: `Option<String>`) Important:
//!    CoH uses ASCII strings (ISO 8859-1 code page), while Rust uses UTF-8. There will
//!    be some lossy conversion in places.
//! * `ppch_` - An array of strings. (C: `char **`, Rust: `Vec<String>`)
//! * `p_` - A pointer to another struct. Typically I just use direct ownership in Rust.
//! * `pp_` - An array of pointers to another struct.
//! * `e_` - An enum value (see the `enums` module).
//! * `pe_` - An array of enum values.
//! * `rgba_` - An `RGBA` value.
//! * `vec_` - A `Vec3` value.
mod boosts;
pub mod config;
mod enums;
mod flags;
mod namekey;
mod strings;
mod villains;

pub use boosts::*;
pub use enums::*;
pub use flags::*;
pub use namekey::*;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::fmt;
use std::rc::Rc;
pub use strings::*;
pub use villains::*;

pub type Keyed<T> = HashMap<NameKey, Rc<T>>;

/// Defines the attributes which can be modified by effects.
#[derive(Debug, Default)]
pub struct CharacterAttributes {
	/// Mod: The number of points to add or remove from current hit points.
	/// ModBase: 0.0, Add, TimesMax, Absolute, HitPoints, DumpAttribs: NO_CUR
	pub f_damage_type: [f32; Self::DAMAGE_TYPE_SIZE],
	/// Cur: Number of hitpoints the player currently has. Running tally.
	/// Mod: How many hitpoints to add or remove from the tally.
	/// ModBase: 0.0, Add, TimesMax, Absolute, HitPoints, DumpAttribs: ALWAYS
	pub f_hit_points: f32,
	/// Max: Number of absorb points the player currently has. Running tally.
	/// ModBase: 0.0, Add, TimesMax, Absolute, HitPoints, DumpAttribs: ALWAYS
	pub f_absorb: f32,
	/// Cur: Measure of endurance the player currently has. Running tally.
	/// Mod: How many points to add or remove from the tally.
	/// ModBase: 0.0, Add, TimesMax, Absolute, DumpAttribs: ALWAYS
	pub f_endurance: f32,
	/// Cur: Measure of Insight the player currently has. Running tally.
	/// Mod: How many points to add or remove from the tally.
	/// ModBase: 0.0, Add, TimesMax, Absolute, DumpAttribs: ALWAYS, Synonym: Idea
	pub f_insight: f32,
	/// Cur: Measure of Rage the player currently has. Running tally.
	/// Mod: How many points to add or remove from the tally.
	/// ModBase: 0.0, Add, TimesMax, Absolute, DumpAttribs: ALWAYS
	pub f_rage: f32,
	/// Cur: The change to hit a target. .75==75%, min 5%, max 95%
	/// Mod: This is a percentage to be added to the base percentage value.
	/// ModBase: 0.0, Add, CLAMP_CUR: No
	pub f_to_hit: f32,
	/// Cur: The chance to avoid being hit by a certain kind of attack. Opposes ToHit.
	/// Mod: This is a percentage added to the base percentage value.
	/// ModBase: 0.0, Add
	pub f_defense_type: [f32; Self::DEFENSE_TYPE_SIZE],
	/// Cur: The chance of avoiding being hit by a direct attack.
	/// Mod: This is a percentage to be added to the base percentage value.
	/// ModBase: 0.0, Add
	pub f_defense: f32,
	/// Cur: How fast the character travels as a percentage of basic character speed. Defaults to 1.0 (100%) (30ft/s).
	/// Mod: A percentage to be multiplied with the base speed value.
	/// ModBase: 1.0, Multiply
	pub f_speed_running: f32,
	/// Cur: How fast the character travels as a percentage of basic character speed. Defaults to 1.0 (100%) (30ft/s).
	/// Mod: A percentage to be multiplied with the base speed value.
	/// ModBase: 1.0, Multiply
	pub f_speed_flying: f32,
	/// Cur: How fast the character travels as a percentage of basic character speed. Defaults to 1.0 (100%) (30ft/s).
	/// Mod: A percentage to be multiplied with the base speed value.
	/// ModBase: 1.0, Multiply
	pub f_speed_swimming: f32,
	/// Cur: How fast the character travels as a percentage of basic character speed. Defaults to 1.0 (100%) (30ft/s).
	/// Mod: A percentage to be multiplied with the base speed value.
	/// ModBase: 1.0, Multiply
	pub f_speed_jumping: f32,
	/// Cur: How well the character jumps as a percentage of basic character jump velocity. Defaults to 1.0 (100%) (12ft).
	/// Mod: A percentage to be multiplied with the base value.
	/// ModBase: 1.0, Multiply
	pub f_jump_height: f32,
	/// Cur: Controls the character's ability to move. Default is 0.0 (use built-ins), running is 1.0, jumping is 0.03.
	/// Mod: This is a percentage to be multiplied with the base value.
	/// ModBase: 1.0, Multiply
	pub f_movement_control: f32,
	/// Cur: Controls the character's ability to move. Default is 0.0 (use built-ins), running is 0.3, jumping is 0.
	/// Mod: This is a percentage to be multiplied with the base value.
	/// ModBase: 1.0, Multiply
	pub f_movement_friction: f32,
	/// Cur: The chance of avoiding being seen when in eyeshot of an enemy.
	/// Mod: This is a percentage to be added to the base percentage value.
	/// ModBase: 0.0, Add
	pub f_stealth: f32,
	/// Cur: This is the distance subtracted from an enemy's perception distance.
	/// Mod: This is a distance to be added to the base distance value.
	/// ModBase: 0.0, Add
	pub f_stealth_radius: f32,
	/// Cur: This is the distance subtracted from an enemy player's perception distance.
	/// Mod: This is a distance to be added to the base distance value.
	/// ModBase: 0.0, Add
	pub f_stealth_radius_player: f32,
	/// Cur: This is the distance the character can see.
	/// Mod: This is a percentage improvement over the base.
	/// ModBase: 1.0, Mutliply, PlusAbsolute
	pub f_perception_radius: f32,
	/// Cur: This is the rate at which hit points are regenerated. (1.0 = 100% max HP per minute.)
	/// Mod: This is a rate which will be multiplied by the base rate.
	/// ModBase: 1.0, Multiply
	pub f_regeneration: f32,
	/// Cur: This is the rate at which endurance is recovered. (1.0 = 100% max endurance per minute.)
	/// Mod: This is a rate which will be multiplied by the base rate.
	/// ModBase: 1.0, Multiply
	pub f_recovery: f32,
	/// Cur: This is the rate at which insight will recover. (1.0 = 100% max insight per minute.)
	/// Mod: This is a rate which will be multiplied by the base rate.
	/// ModBase: 1.0, Multiply
	pub f_insight_recovery: f32,
	/// Cur: The general threat level of the character, used by AI.
	/// Mod: N/A
	/// ModBase: 0.0, Add
	pub f_threat_level: f32,
	/// Cur: This is how much the character is taunting a target. (Not really useful, modifying makes the AI more belligerent to you.)
	/// Mod: N/A
	/// ModBase: 1.0, Add
	pub f_taunt: f32,
	/// Cur: This is how much the character is being placated. (Not really useful, modifying makes the AI less belligerent to you.)
	/// Mod: N/A
	/// ModBase: 1.0, Add
	pub f_placate: f32,
	/// Cur: Wanders around. Boolean.
	/// ModBase: 0.0, Add
	pub f_confused: f32,
	/// Cur: Wants to run away. Boolean.
	/// ModBase: 0.0, Add
	pub f_afraid: f32,
	/// Cur: Cowers. Boolean.
	/// ModBase: 0.0, Add
	pub f_terrorized: f32,
	/// Cur: Cannot move or execute powers. Boolean.
	/// ModBase: 0.0, Add
	pub f_held: f32,
	/// Cur: Cannot move. Boolean.
	/// ModBase: 0.0, Add
	pub f_immobilized: f32,
	/// Cur: Cannot execute powers. Boolean.
	/// ModBase: 0.0, Add
	pub f_stunned: f32,
	/// Cur: Immobilize + stun unless awoken. Boolean.
	/// ModBase: 0.0, Add
	pub f_sleep: f32,
	/// Cur: Can fly. Boolean.
	/// ModBase: 0.0, Add
	pub f_fly: f32,
	/// Cur: Can use jump pack. Boolean.
	/// ModBase: 0.0, Add
	pub f_jump_pack: f32,
	/// Cur: Initiates a teleport. Boolean.
	/// ModBase: 0.0, Add
	pub f_teleport: f32,
	/// Cur: Only caster can hit themself. Boolean.
	/// ModBase: 0.0, Add
	pub f_untouchable: f32,
	/// Cur: Doesn't collide with others. Boolean.
	/// ModBase: 0.0, Add
	pub f_intangible: f32,
	/// Cur: Powers only affect self. Boolean.
	/// ModBase: 0.0, Add
	pub f_only_affects_self: f32,
	/// Cur: XP gain factor.
	/// ModBase: 0.0, Add
	pub f_experience_gain: f32,
	/// Cur: Influence gain factor.
	/// ModBase: 0.0, Add
	pub f_influence_gain: f32,
	/// Cur: Prestige gain factor.
	/// ModBase: 0.0, Add
	pub f_prestige_gain: f32,
	/// Cur: Doesn't do anything.
	/// ModBase: 0.0, Add
	pub f_null_bool: f32,
	/// Cur: How hard the character knocks enemies up as a percentage of base. Default to 1.0 (100%).
	/// Mod: A percentage to be multiplied with the base value.
	/// ModBase: 0.0, Multiply
	pub f_knock_up: f32,
	/// Cur: How hard the character knocks enemies back as a percentage of base. Default to 1.0 (100%).
	/// Mod: A percentage to be multiplied with the base value.
	/// ModBase: 0.0, Multiply
	pub f_knock_back: f32,
	/// Cur: How hard the character repels enemies as a percentage of base. Default to 1.0 (100%).
	/// Mod: A percentage to be multiplied with the base value.
	/// ModBase: 0.0, Multiply
	pub f_repel: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: A percentage which is multiplied with a power's facets.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
	pub f_accuracy: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: A percentage which is multiplied with a power's facets.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
	pub f_radius: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: A percentage which is multiplied with a power's facets.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
	pub f_arc: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: A percentage which is multiplied with a power's facets.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
	pub f_range: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: A rate which will be multiplied by the base (hard-coded) rate.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
	pub f_time_to_activate: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: A rate which will be multiplied by the base (hard-coded) rate.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
	pub f_recharge_time: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: A rate which will be multiplied by the base (hard-coded) rate.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
	pub f_interrupt_time: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: This is a magnitude which will divide into the cost.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES
	pub f_endurance_discount: f32,
	/// Cur: Unused.
	/// Mod: Unused.
	/// Str: This is a magnitude which will divide into the cost.
	/// ModBase: 1.0, Multiply, DumpAttribs: STR_RES, NoDump
	pub f_insight_discount: f32,
	/// Cur: A "fake" attribute which shows up as a meter in the UI.
	/// Mod: Amount to increase or decrease the meter.
	/// ModBase: 0.0, Add, PlusAbsolute
	pub f_meter: f32,
	/// Cur: The chance to avoid being hit by a certain kind of attack. Opposes Accuracy. PvP only.
	/// Mod: This is a percentage added to the base percentage value.
	/// Str: Anti-accuracy.
	/// ModBase: 0.0, Add
	pub f_elusivity: [f32; Self::ELUSIVITY_SIZE],
	pub f_elusivity_base: f32,
}

macro_rules! offsets {
	($($name:ident, $offset:literal),+ $(,)?) => {
		$( pub const $name: usize = $offset; )+
	}
}

#[allow(dead_code)]
impl CharacterAttributes {
	pub const DAMAGE_TYPE_SIZE: usize = 20;
	pub const DEFENSE_TYPE_SIZE: usize = 20;
	pub const ELUSIVITY_SIZE: usize = 20;

	// This is pretty annoying but the bin files refer to the
	// various fields in CharacterAttributes by their struct offset.
	// Hopefully no one modifies the source struct...
	// If they do, make sure to fix ranges in effect.rs:get_scaled_effect()
	#[rustfmt::skip]
	offsets!(
		OFFSET_DMG_0, 0,
		OFFSET_DMG_1, 4,
		OFFSET_DMG_2, 8,
		OFFSET_DMG_3, 12,
		OFFSET_DMG_4, 16,
		OFFSET_DMG_5, 20,
		OFFSET_DMG_6, 24,
		OFFSET_DMG_7, 28,
		OFFSET_DMG_8, 32,
		OFFSET_DMG_9, 36,
		OFFSET_DMG_10, 40,
		OFFSET_DMG_11, 44,
		OFFSET_DMG_12, 48,
		OFFSET_DMG_13, 52,
		OFFSET_DMG_14, 56,
		OFFSET_DMG_15, 60,
		OFFSET_DMG_16, 64,
		OFFSET_DMG_17, 68,
		OFFSET_DMG_18, 72,
		OFFSET_DMG_19, 76,
		OFFSET_HIT_POINTS, 80,
		OFFSET_ABSORB, 84,
		OFFSET_ENDURANCE, 88,
		OFFSET_INSIGHT, 92,
		OFFSET_RAGE, 96,
		OFFSET_TOHIT, 100,
		OFFSET_DEF_0, 104,
		OFFSET_DEF_1, 108,
		OFFSET_DEF_2, 112,
		OFFSET_DEF_3, 116,
		OFFSET_DEF_4, 120,
		OFFSET_DEF_5, 124,
		OFFSET_DEF_6, 128,
		OFFSET_DEF_7, 132,
		OFFSET_DEF_8, 136,
		OFFSET_DEF_9, 140,
		OFFSET_DEF_10, 144,
		OFFSET_DEF_11, 148,
		OFFSET_DEF_12, 152,
		OFFSET_DEF_13, 156,
		OFFSET_DEF_14, 160,
		OFFSET_DEF_15, 164,
		OFFSET_DEF_16, 168,
		OFFSET_DEF_17, 172,
		OFFSET_DEF_18, 176,
		OFFSET_DEF_19, 180,
		OFFSET_DEFENSE, 184,
		OFFSET_RUNNING_SPEED, 188,
		OFFSET_FLYING_SPEED, 192,
		OFFSET_SWIMMING_SPEED, 196,
		OFFSET_JUMPING_SPEED, 200,
		OFFSET_JUMP_HEIGHT, 204,
		OFFSET_MOVEMENT_CONTROL, 208,
		OFFSET_MOVEMENT_FRICTION, 212,
		OFFSET_STEALTH, 216,
		OFFSET_STEALTH_RADIUS_PVE, 220,
		OFFSET_STEALTH_RADIUS_PVP, 224,
		OFFSET_PERCEPTION_RADIUS, 228,
		OFFSET_REGENERATION, 232,
		OFFSET_RECOVERY, 236,
		OFFSET_INSIGHT_RECOVERY, 240,
		OFFSET_THREAT_LEVEL, 244,
		OFFSET_TAUNT, 248,
		OFFSET_PLACATE, 252,
		OFFSET_CONFUSED, 256,
		OFFSET_AFRAID, 260,
		OFFSET_TERRORIZED, 264,
		OFFSET_HELD, 268,
		OFFSET_IMMOBILIZED, 272,
		OFFSET_STUNNED, 276,
		OFFSET_SLEEP, 280,
		OFFSET_FLY, 284,
		OFFSET_JUMP_PACK, 288,
		OFFSET_TELEPORT, 292,
		OFFSET_UNTOUCHABLE, 296,
		OFFSET_INTANGIBLE, 300,
		OFFSET_ONLY_AFFECTS_SELF, 304,
		OFFSET_EXPERIENCE_GAIN, 308,
		OFFSET_INFLUENCE_GAIN, 312,
		OFFSET_PRESTIGE_GAIN, 316,
		OFFSET_EVADE, 320,
		OFFSET_KNOCKUP, 324,
		OFFSET_KNOCKBACK, 328,
		OFFSET_REPEL, 332,
		OFFSET_ACCURACY, 336,
		OFFSET_RADIUS, 340,
		OFFSET_ARC, 344,
		OFFSET_RANGE, 348,
		OFFSET_TIME_TO_ACTIVATE, 352,
		OFFSET_RECHARGE_TIME, 356,
		OFFSET_INTERRUPT_TIME, 360,
		OFFSET_ENDURANCE_DISCOUNT, 364,
		OFFSET_INSIGHT_DISCOUNT, 368,
		OFFSET_METER, 372,
		OFFSET_ELUSIVITY_0, 376,
		OFFSET_ELUSIVITY_1, 380,
		OFFSET_ELUSIVITY_2, 384,
		OFFSET_ELUSIVITY_3, 388,
		OFFSET_ELUSIVITY_4, 392,
		OFFSET_ELUSIVITY_5, 396,
		OFFSET_ELUSIVITY_6, 400,
		OFFSET_ELUSIVITY_7, 404,
		OFFSET_ELUSIVITY_8, 408,
		OFFSET_ELUSIVITY_9, 412,
		OFFSET_ELUSIVITY_10, 416,
		OFFSET_ELUSIVITY_11, 420,
		OFFSET_ELUSIVITY_12, 424,
		OFFSET_ELUSIVITY_13, 428,
		OFFSET_ELUSIVITY_14, 432,
		OFFSET_ELUSIVITY_15, 436,
		OFFSET_ELUSIVITY_16, 440,
		OFFSET_ELUSIVITY_17, 444,
		OFFSET_ELUSIVITY_18, 448,
		OFFSET_ELUSIVITY_19, 452,
		OFFSET_ELUSIVITY_BASE, 456,
	);

	pub fn new() -> Self {
		Default::default()
	}
}

/// Defines the attributes which can be modified by effects.
/// This is essentially a version of `CharacterAttributes` where each entry is
/// an array rather than a single value. The arrays are typically 50 entries
/// long, representing values for levels 1-50.
#[derive(Debug, Default)]
pub struct CharacterAttributesTable {
	pub pf_damage_type: [Vec<f32>; CharacterAttributes::DAMAGE_TYPE_SIZE],
	pub pf_hit_points: Vec<f32>,
	pub pf_endurance: Vec<f32>,
	pub pf_insight: Vec<f32>,
	pub pf_rage: Vec<f32>,
	pub pf_to_hit: Vec<f32>,
	pub pf_defense_type: [Vec<f32>; CharacterAttributes::DEFENSE_TYPE_SIZE],
	pub pf_defense: Vec<f32>,
	pub pf_speed_running: Vec<f32>,
	pub pf_speed_flying: Vec<f32>,
	pub pf_speed_swimming: Vec<f32>,
	pub pf_speed_jumping: Vec<f32>,
	pub pf_jump_height: Vec<f32>,
	pub pf_movement_control: Vec<f32>,
	pub pf_movement_friction: Vec<f32>,
	pub pf_stealth: Vec<f32>,
	pub pf_stealth_radius: Vec<f32>,
	pub pf_stealth_radius_player: Vec<f32>,
	pub pf_perception_radius: Vec<f32>,
	pub pf_regeneration: Vec<f32>,
	pub pf_recovery: Vec<f32>,
	pub pf_insight_recovery: Vec<f32>,
	pub pf_threat_level: Vec<f32>,
	pub pf_taunt: Vec<f32>,
	pub pf_placate: Vec<f32>,
	pub pf_confused: Vec<f32>,
	pub pf_afraid: Vec<f32>,
	pub pf_terrorized: Vec<f32>,
	pub pf_held: Vec<f32>,
	pub pf_immobilized: Vec<f32>,
	pub pf_stunned: Vec<f32>,
	pub pf_sleep: Vec<f32>,
	pub pf_fly: Vec<f32>,
	pub pf_jump_pack: Vec<f32>,
	pub pf_teleport: Vec<f32>,
	pub pf_untouchable: Vec<f32>,
	pub pf_intangible: Vec<f32>,
	pub pf_only_affects_self: Vec<f32>,
	pub pf_experience_gain: Vec<f32>,
	pub pf_influence_gain: Vec<f32>,
	pub pf_prestige_gain: Vec<f32>,
	pub pf_null_bool: Vec<f32>,
	pub pf_knock_up: Vec<f32>,
	pub pf_knock_back: Vec<f32>,
	pub pf_repel: Vec<f32>,
	pub pf_accuracy: Vec<f32>,
	pub pf_radius: Vec<f32>,
	pub pf_arc: Vec<f32>,
	pub pf_range: Vec<f32>,
	pub pf_time_to_activate: Vec<f32>,
	pub pf_recharge_time: Vec<f32>,
	pub pf_interrupt_time: Vec<f32>,
	pub pf_endurance_discount: Vec<f32>,
	pub pf_insight_discount: Vec<f32>,
	pub pf_meter: Vec<f32>,
	pub pf_elusivity: [Vec<f32>; CharacterAttributes::ELUSIVITY_SIZE],
	pub pf_elusivity_base: Vec<f32>,
	pub pf_absorb: Vec<f32>,
}

impl CharacterAttributesTable {
	pub fn new() -> Self {
		Default::default()
	}
}

#[derive(Debug, Default)]
pub struct NamedTable {
	pub pch_name: Option<String>,
	pub pf_values: Vec<f32>,
}

impl NamedTable {
	pub fn new() -> Self {
		Default::default()
	}
}

/// Defines the character class (archetype), which sets up the allowable powers and
/// default hit points and defense for the character.
#[derive(Debug, Default)]
pub struct Archetype {
	pub pch_name: Option<String>,
	pub pch_display_name: Option<String>,
	pub pch_display_help: Option<String>,
	/// This determines what origins can be picked by the player.
	pub ppch_allowed_origin_names: Vec<String>,
	/// This is a string array of special restrictions (e.g. Kheldian, Avian, etc).
	pub ppch_special_restrictions: Vec<String>,
	/// This is a string of store restrictions.
	pub pch_store_restrictions: Option<String>,
	/// This message is displayed to the user if this AT is locked describing what they can do to unlock it.
	pub pch_locked_tooltip: Option<String>,
	/// Product code that will be used for the purchase option if this AT is locked.
	pub pch_product_code: Option<String>,
	/// Class that is used for mission difficulty reduction purposes.
	pub pch_reduction_class: Option<String>,
	/// Class uses AV flag for mission difficulty reduction purposes
	pub b_reduce_as_av: bool,
	/// This is an int array of levels at which the character respecs instead of just picking the new power/slots.
	pub pi_level_up_respecs: Vec<i32>,
	pub pch_display_short_help: Option<String>,
	/// The icon for this archetype.
	pub pch_icon: Option<String>,
	/// Names for the various categories and power sets available to this class.
	pub pch_primary_category: Option<NameKey>,
	/// Names for the various categories and power sets available to this class.
	pub pch_secondary_category: Option<NameKey>,
	/// Names for the various categories and power sets available to this class.
	pub pch_power_pool_category: Option<NameKey>,
	/// Names for the various categories and power sets available to this class.
	pub pch_epic_pool_category: Option<NameKey>,
	/// The minimum values allowed for each attribute.
	pub pp_attrib_min: Vec<CharacterAttributes>,
	/// The base values for each attribute.
	pub pp_attrib_base: Vec<CharacterAttributes>,
	/// The minimum values allowed for an attribute's strength.
	pub pp_attrib_strength_min: Vec<CharacterAttributes>,
	/// The minimum values allowed for an attribute's resistance.
	pub pp_attrib_resistance_min: Vec<CharacterAttributes>,
	/// Table for diminishing returns.
	pub pp_attrib_diminishing_str: [Vec<CharacterAttributes>; Self::DIMINISH_SIZE],
	/// Table for diminishing returns.
	pub pp_attrib_diminishing_cur: [Vec<CharacterAttributes>; Self::DIMINISH_SIZE],
	/// Table for diminishing returns.
	pub pp_attrib_diminishing_res: [Vec<CharacterAttributes>; Self::DIMINISH_SIZE],
	/// Only used temporarily by the game, but we have to account for it when reading the .bin.
	pub pp_attrib_temp_max: Vec<CharacterAttributesTable>,
	/// Only used temporarily by the game, but we have to account for it when reading the .bin.
	pub pp_attrib_temp_max_max: Vec<CharacterAttributesTable>,
	/// Only used temporarily by the game, but we have to account for it when reading the .bin.
	pub pp_attrib_temp_strength_max: Vec<CharacterAttributesTable>,
	/// Only used temporarily by the game, but we have to account for it when reading the .bin.
	pub pp_attrib_temp_resistance_max: Vec<CharacterAttributesTable>,
	/// Tables used by powers for scaling powers by level.
	/// Changed this to a `HashMap` to make it easier to lookup tables.
	pub pp_named_tables: HashMap<String, NamedTable>,
	/// Gang together hit points and status points. Modifications to hit points will affect status points and vice-versa. Hit points
	/// are are set to be the same as status points.
	pub b_connect_hp_and_status: bool,
	/// Byte offset to the attribute in the `CharacterAttributes` struct.
	/// If non-zero, points to the attrib which is used as hit points after the character has been defeated.
	/// Once the actual hit point attrib has reached zero, this attrib is then used to determine if the character
	/// has actually been defeated.
	pub off_defiant_hit_points_attrib: u32,
	/// Scale applied to damage before it's remove from the `off_defiant_hit_points_attrib`.
	pub f_defiant_scale: f32,
	// Non-data fields.
	/// Used for lookup table purposes.
	pub class_key: Option<NameKey>,
}

impl Archetype {
	#[allow(non_upper_case_globals)]
	pub const kClassesDiminish_Inner: usize = 0;
	#[allow(non_upper_case_globals)]
	pub const kClassesDiminish_Outer: usize = 1;
	const DIMINISH_SIZE: usize = 2;
	pub const CLASS_PREFIX: &'static str = "@class_";
	pub const CLASS_PREFIX_LEN: usize = Self::CLASS_PREFIX.len();

	pub fn new() -> Self {
		Default::default()
	}
}

/// Defines a set of powers which are group together and become available
/// over time. Again, this defines the shared attributes. Character-specific
/// attributes (length of time held, for example) are found in struct
/// PowerSet.
///
/// If the same Power appears in more than one PowerSet (and this includes
/// each class-specific power-pool sets) then it needs to be defined again.
/// This is true since each BasePower refers to a single PowerSet.
#[derive(Debug, Default)]
pub struct BasePowerSet {
	/// Internal name
	pub pch_name: Option<String>,
	/// Full name, including source category
	pub pch_full_name: Option<NameKey>,
	/// UI string
	pub pch_display_name: Option<String>,
	/// UI string
	pub pch_display_help: Option<String>,
	/// UI string
	pub pch_display_short_help: Option<String>,
	/// Name of icon which represents this power set
	pub pch_icon_name: Option<String>,
	/// Costume keys given to players with this set
	pub ppch_costume_keys: Vec<String>,
	/// Default costume pieces for new players, or parts to add for old ones
	pub ppch_costume_parts: Vec<String>,
	/// Which power system this power set (and all the powers within it) is associated with. (Powers or Skills, for now)
	pub e_system: PowerSystem,
	/// If true, this powerset is not specific to one of a player's multiple builds and is instead shared among them.
	pub b_is_shared: bool,
	/// Determines whether the power is shown in the power inventory or not.
	pub e_show_in_inventory: ShowPowerSetting,
	/// If true, then the power set is shown in the power management (i.e. the enhancement) and enh slot assignment screens, otherwise it is not.
	pub b_show_in_manage: bool,
	/// If true, then the power set is shown in the power tab of the Player Info window.
	pub b_show_in_info: bool,
	/// If non-zero, this powerset is a specialization powerset, available at this level as an additional set to pick from as well as their
	/// chosen set - and thus cannot be picked as a main set.
	pub i_specialize_at: i32,
	/// Requires statement controlling whether specialization powerset can be offered or not (so multiple specializations can be listed
	/// for a class and once one or more is picked the rest are shut off).
	pub pp_specialize_requires: Vec<String>,
	/// Account evaluator statement controlling whether a player has access to this powerset.
	pub pch_account_requires: Option<String>,
	/// Tooltip to display when player has failed to meet the AccountRequires
	pub pch_account_tooltip: Option<String>,
	/// Product that can be bought from the store to help the player meet the AccountRequires
	pub pch_account_product: Option<String>,
	/// Character evaluator statement controlling whether a player has access to this powerset.
	pub ppch_set_buy_requires: Vec<String>,
	/// Error message to display when the player fails the SetBuyRequires
	pub pch_set_buy_requires_failed_text: Option<String>,
	/// The list of powers which are part of this power set.
	pub pp_powers: Vec<Rc<BasePower>>,
	/// The array of names of included powers.
	pub pp_power_names: Vec<NameKey>,
	/// How old the set has to be (in levels) before the power becomes available.
	pub pi_available: Vec<i32>,
	/// AI ONLY: Beyond what level the power will no longer be used by the AI.
	pub pi_ai_max_level: Vec<i32>,
	/// AI ONLY: Minimum rank con that this power is usable by
	pub pi_ai_min_rank_con: Vec<i32>,
	/// AI ONLY: Maximum rank con that this power is usable by
	pub pi_ai_max_rank_con: Vec<i32>,
	pub pi_min_difficulty: Vec<i32>,
	pub pi_max_difficulty: Vec<i32>,
	/// Filename this definition came from.
	pub pch_source_file: Option<String>,
	pub i_force_level_bought: i32,
	// Non-data fields.
	/// Whether or not to include this power set in the output files.
	pub include_in_output: bool,
}

impl BasePowerSet {
	pub fn new() -> Self {
		Default::default()
	}
}

#[derive(Debug, Default)]
pub struct PowerRedirect {
	/// Name of the base power to redirect to.
	pub pch_name: Option<NameKey>,
	/// Expression which must evaluate to true for this redirection to be used. An empty expression is always true and indicates a
	/// last resort fallback power.
	pub ppch_requires: Vec<String>,
	/// If true, this redirection is used to show the detailed power information in the client UI.
	pub b_show_in_info: bool,
}

impl PowerRedirect {
	pub fn new() -> Self {
		Default::default()
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

/// Extended targeting info.
#[derive(Debug, Default)]
pub struct AttribModTargetInfo {
	pub ppch_marker_names: Vec<String>,
	pub pi_marker_count: Vec<i32>,
}

impl AttribModTargetInfo {
	pub fn new() -> Self {
		Default::default()
	}
}

#[derive(Debug, Default)]
pub struct SuppressPair {
	/// The index of the event to check. (See `PowerEvent` enum in character_base.h)
	pub idx_event: i32,
	/// How many seconds it must be after the event before this `AttribMod` is allowed to go off.
	pub ul_seconds: u32,
	/// If true, the `AttribMod` will always be suppressed when in the event window.
	/// If false, then if the `AttribMod` has already been applied once, it continue to gets applied.
	pub b_always: bool,
}

impl SuppressPair {
	pub fn new() -> Self {
		Default::default()
	}
}

/// Messages
#[derive(Debug, Default)]
pub struct AttribModMessages {
	/// Message displayed to the attacker when he hits with this power.
	pub pch_display_attacker_hit: Option<String>,
	/// Message displayed to the victim when he gets hits with this power.
	pub pch_display_victim_hit: Option<String>,
	/// Message displayed over the victim's head when this attrib mod goes off.
	pub pch_display_float: Option<String>,
	/// Message displayed over the victim's head when this attrib mod is the defense that caused some attack to miss the victim.
	pub pch_display_defense_float: Option<String>,
}

impl AttribModMessages {
	pub fn new() -> Self {
		Default::default()
	}
}

/// FX
#[derive(Debug, Default)]
pub struct AttribModFX {
	/// Sets the given bits for the lifetime of the `AttribMod`.
	pub pi_continuing_bits: Vec<i32>,
	/// If non-NULL, plays maintained FX for the lifetime of the `AttribMod`. When it times out, the FX is killed.
	pub pch_continuing_fx: Option<String>,
	/// Sets the given bits while the `AttribMod` is alive and the `attrCur` for the modified attribute is greater than zero.
	pub pi_conditional_bits: Vec<i32>,
	/// If non-NULL, plays maintained FX while the `AttribMod` is alive and the `attrCur` for the modified attribute is greater than zero.
	pub pch_conditional_fx: Option<String>,
}

impl AttribModFX {
	pub fn new() -> Self {
		Default::default()
	}
}

pub struct RGBA([u8; 4]);

impl RGBA {
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		RGBA([r, g, b, a])
	}

	pub fn r(&self) -> u8 {
		self.0[0]
	}

	pub fn g(&self) -> u8 {
		self.0[1]
	}

	pub fn b(&self) -> u8 {
		self.0[2]
	}

	pub fn a(&self) -> u8 {
		self.0[3]
	}
}

impl Default for RGBA {
	fn default() -> Self {
		RGBA::new(0, 0, 0, 0)
	}
}

impl fmt::Debug for RGBA {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{{ r: {}, g: {}, b: {}, a: {} }}",
			self.r(),
			self.g(),
			self.b(),
			self.a()
		)
	}
}

/// This defines an actual effect of a power. A power may have multiple
/// `AttribModTemplate`s. When a power is used, these `AttribModTemplate`s are
/// pared down to `AttribMod`s and attached to the targeted character.
#[derive(Debug, Default)]
pub struct AttribModTemplate {
	/// Array of byte offsets to the attribute in the `CharacterAttributes` struct.
	pub p_attrib: Vec<SpecialAttrib>,
	/// Byte offset to the structure in the `Character` to the `CharacterAttributes` to modify.
	pub off_aspect: u32,
	/// Determines when this attrib mod is applied during the lifecycle of a power.
	pub e_application_type: ModApplicationType,
	/// Determines if the duration or the magnitude is what is calculated.
	pub e_type: ModType,
	/// Who is the target of this particular modifier.
	pub e_target: ModTarget,
	/// Extra targeting info for this mod.
	pub p_target_info: Option<AttribModTargetInfo>,
	/// The name of the table to use for scaling the power by level. Tables are defined in the class.
	pub pch_table: Option<String>,
	/// How much to scale the basic value given by the class table for the given attribute.
	pub f_scale: f32,
	/// How long the effect lasts on the target. Booleans calculate this value, others use it directly.
	pub f_duration: ModDuration,
	/// Default for how much to change the attribute. Booleans use this value, others calculate it.
	pub f_magnitude: f32,
	/// An expression which calculates the duration of the `AttribMod`. If empty, the `f_duration` field is used instead.
	pub ppch_duration: Vec<String>,
	/// An expression which calculates the magnitude of the `AttribMod`.
	/// If empty, the fMagnitude field is used instead. Only used for `kModType_Expression`.
	pub ppch_magnitude: Vec<String>,
	/// How long to wait in seconds before applying the attrib modifier for the first time. Stacks with the delay from the parent `EffectGroup`.
	pub f_delay: f32,
	/// The attrib modifier is applied every `f_period` seconds.
	pub f_period: f32,
	/// If less than 1.0, the chance for an individual tick to apply.
	pub f_tick_chance: f32,
	/// An expression which describes the conditions under which this template is applied. Delayed Requires are checked "just in time" (right before
	/// the mod ticks for the first time) and cancel the mod if the expression evaluates to false.
	pub ppch_delayed_requires: Vec<String>,
	/// Determines how identical `AttribMod`s from the same power but from different casters are handled.
	pub e_caster_stack: CasterStackType,
	/// Determines how multiple `AttribMod`s that pass the `CasterStackType` check are handled.
	pub e_stack: StackType,
	/// Used for `kStackType_StackThenIgnore`.
	/// Determines how many times the `AttribMod` should stack before it is ignored.
	pub i_stack_limit: i32,
	/// If this is not zero, we compare this instead of stacking by the template.
	pub i_stack_key: i32,
	/// A list of `PowerEvent`s which will cancel this `AttribMod` outright.
	pub pi_cancel_events: Vec<PowerEvent>,
	/// An earray of events to check against to determine if this `AttribMod` is allowed to go off. This doesn't reject the `AttribMod` entirely
	/// (like the Requires does). If the time passes after the event and the `AttribMod` still has time left on it, it will work.
	pub pp_suppress: Vec<SuppressPair>,
	/// If specified, a power must specifically allow this `BoostType` in order for this mod to apply as part of a Boost, even if the Boost itself applies.
	/// This is for Boosts with mixed BoostTypes such as Hami-Os where Damage boosts can be slotted into Damage Resist powers for exploitative gain.
	pub boost_mod_allowed: SpecialAttrib,
	/// Boolean flags for this attribmod
	pub i_flags: AttribModFlag,
	pub p_messages: Option<AttribModMessages>,
	pub p_fx: Option<AttribModFX>,
	pub p_params: Option<AttribModParam>,
}

impl AttribModTemplate {
	pub fn new() -> Self {
		Default::default()
	}
}

/// An effect group is a group of AttribMod templates that are always applied together.
#[derive(Debug, Default)]
pub struct EffectGroup {
	/// Effect tags (for chance mods, etc)
	pub ppch_tags: Vec<String>,
	/// The chance that this attrib modifier will be applied to the target. (1.0 == 100%)
	pub f_chance: f32,
	/// If set, this will cause the chance to activate to be calculated automatically
	/// to result in the approximate number of procs per minute.
	pub f_procs_per_minute: f32,
	/// How long to wait before applying the attrib modifier for the first time.
	pub f_delay: f32,
	/// If set, these values define the inner and outer radii of a spherical shell that limits the targets affected by this
	/// effect group. An inner radius of 0 indicates a plain sphere. Setting both to 0 limits it to the main target
	/// only.
	pub f_radius_inner: f32,
	pub f_radius_outer: f32,
	/// An expression which describes the conditions under which the templates in the effect group may be applied. If there
	/// is no expression, then the templates are always applied.
	pub ppch_requires: Vec<String>,
	/// Boolean flags for this effect group.
	pub i_flags: EffectGroupFlag,
	/// AttribMod templates to be applied by this effect group.
	pub pp_templates: Vec<AttribModTemplate>,
	/// Child effect groups.
	pub pp_effects: Vec<EffectGroup>,
	/// Flags created at bin time based upon what special combat eval parameters need to be pushed for evaluation.
	pub i_eval_flags: u32,
}

impl EffectGroup {
	pub fn new() -> Self {
		Default::default()
	}
}

#[derive(Debug, Default)]
pub struct PowerFX {
	/// What .pfx file this was loaded from.
	pub pch_source_file: Option<String>,
	/// This is the attack anim used when the character is already in the stance. Set once. A.k.a. `cast_anim`.
	pub pi_attack_bits: Vec<i32>,
	/// The block reaction. Set once.
	pub pi_block_bits: Vec<i32>,
	/// The wind up. Set once.
	pub pi_wind_up_bits: Vec<i32>,
	/// The hit reaction. Set once. A.k.a. `hit_anim`.
	pub pi_hit_bits: Vec<i32>,
	/// This is used for the death animation if a specific one should be used instead of the default.
	/// Set always? A.k.a. `deathanimbits`.
	pub pi_death_bits: Vec<i32>,
	/// This is an optional pre-animation that happens when you first click on a power button.
	/// Set once. A.k.a. `AttachedAnim`.
	pub pi_activation_bits: Vec<i32>,
	/// This is an optional post-animation that happens when a toggle power is shut off. Set once.
	pub pi_deactivation_bits: Vec<i32>,
	/// This is the attack anim used when the character is entering the stance for the first time. Set once.
	pub pi_initial_attack_bits: Vec<i32>,
	/// Sets the given bits for the lifetime of an `AttribMod`.
	pub pi_continuing_bits: Vec<i32>,
	/// Sets the given bits while an `AttribMod` is alive and the `attrCur` for the modified attribute is greater than zero.
	pub pi_conditional_bits: Vec<i32>,
	/// The effect that happens immediately as the power button on the UI is selected. A.k.a. `AttachedFxName`.
	pub pch_activation_fx: Option<String>,
	/// The effect that happens when a toggle power is shut off.
	pub pch_deactivation_fx: Option<String>,
	/// The main effect fx filename used when the character is already in a stance. A.k.a. `TravellingProjectileEffect`.
	pub pch_attack_fx: Option<String>,
	/// Chaining from PrevTarget (or main target) to Target.
	pub pch_secondary_attack_fx: Option<String>,
	/// Hit reaction fx filename. A.k.a. `AttachedToVictimFxName`.
	pub pch_hit_fx: Option<String>,
	/// The effect that is played during wind up.
	pub pch_wind_up_fx: Option<String>,
	/// Block reaction fx filename.
	pub pch_block_fx: Option<String>,
	/// Death fx filename.
	pub pch_death_fx: Option<String>,
	/// The main effect fx filename used when the character is entering the stance for the first time.
	pub pch_initial_attack_fx: Option<String>,
	/// If non-NULL, plays maintained FX for the lifetime of an `AttribMod`. When it times out, the FX is killed.
	pub ppch_continuing_fx: Vec<String>,
	/// If non-NULL, plays maintained FX while an `AttribMod` is alive and the `attrCur` for the modified attribute is greater than zero.
	pub ppch_conditional_fx: Vec<String>,
	/// Sets the "mode" (combat, weapon, shotgun..) the player is in. These bits are always set until the power is deselected.
	/// Set always. A.k.a. `SeqBits`.
	pub pi_mode_bits: Vec<i32>,
	/// This is the time it takes for an attack to hit an enemy for the `AttackBits` animation.
	/// The default is set to happen on frame 15, so if it's different than that the time is put here.
	/// 0 means use the default of 15.
	pub i_frames_before_hit: i32,
	/// This is the time it takes for the secondary attack to hit.
	pub i_frames_before_secondary_hit: i32,
	/// If true, the hit animation is delayed according to how far away the victim is from the attacker. This is true for missle powers
	/// which have slow missles (like a fireball), and false for melee and fast missile powers (like guns).
	pub b_delayed_hit: bool,
	/// This is the time it takes for the attack animation to complete. 0 means use the default of 35.
	pub i_frames_attack: i32,
	/// This is the time it takes for an attack to hit an enemy for the `InitialAttackBits` animation.
	/// The default is set to happen on frame 15, so if it's different than that the time is put here.
	/// 0 means use the default of 15.
	pub i_initial_frames_before_hit: i32,
	/// Time to wait before playing the initial attack fx. Provided so they can use the same fx for `AttackFx` and `InitialAttackFx`.
	pub i_initial_attack_fx_frame_delay: i32,
	/// How fast the projectile moves when it leaves the entity.
	pub f_projectile_speed: f32,
	/// How fast the projectile moves from main target to secondary targets (or next chain jump)
	pub f_secondary_projectile_speed: f32,
	/// This is the time to wait before starting the block animation for the `InitialAttackBits` animation.
	/// The default is set to happen on frame 0, so if it's different than that the time is put here.
	pub i_initial_frames_before_block: i32,
	/// If not empty, ignores a mismatch between the `TimeToActivate` and `FramesAttack`.
	pub pch_ignore_attack_time_errors: Option<String>,
	/// This is the time to wait before starting the block animation. The default is set to happen on frame 0, so if it's different
	/// than that the time is put here.
	pub i_frames_before_block: i32,
	/// If true, then the FX for this power are "important" and shouldn't be suppressed for performance enhancement or by user choice.
	pub b_important: bool,
	/// Added i26p5. FXImportant (maybe?)
	pub b_fx_important: bool,
	/// The tint to use for non-customized powers. This lets artists reuse tintable assets.
	pub rgba_default_tint_primary: RGBA,
	/// The tint to use for non-customized powers. This lets artists reuse tintable assets.
	pub rgba_default_tint_secondary: RGBA,
	/// Added i26p5. HideOriginal (maybe?)
	pub b_hide_original: bool,
}

impl PowerFX {
	pub fn new() -> Self {
		Default::default()
	}

	/// Converts time expressed in frames into seconds.
	pub fn frames_as_seconds(frames: i32) -> f32 {
		const FRAME_TIME: f32 = 1.0 / 30.0; // 30fps
		frames as f32 * FRAME_TIME
	}
}

#[derive(Debug, Default)]
pub struct CustomPowerFX {
	/// Shown in the customization menu.
	pub pch_display_name: Option<String>,
	/// Use these settings if the player's costume has this token.
	pub pch_token: Option<String>,
	/// Alternate themes that can match this customfx if the token itself doesn't match.
	pub ppch_alt_themes: Vec<String>,
	/// `CustomPowerFX` with the same category should be considered exclusive of one another.  In the customization menu, the player will see
	/// a list for each category.
	pub pch_category: Option<String>,
	pub p_fx: Option<PowerFX>,
	pub pch_palette_name: Option<String>,
}

impl CustomPowerFX {
	pub fn new() -> Self {
		Default::default()
	}
}

/// The basic definition of a power. This struct contains all the attributes of a power which are shared by all entities in the game. Character-specific
/// differences (such as number of boosts, level, etc.) are kept in struct Power.
#[derive(Debug, Default)]
pub struct BasePower {
	/// Internal name of the power.
	pub pch_name: Option<String>,
	/// Full name, including source category and set.
	pub pch_full_name: Option<NameKey>,
	/// If this power was duplicated, the original full name.
	pub pch_source_name: Option<String>,
	/// The source file of the power.
	pub source_file: Option<String>,
	/// Which power system this power is associatd with. (Powers or Skills, for now)
	pub e_system: PowerSystem,
	/// If true, this power is given away automatically if the player is allowed to have it. (Must pass ppchRequires and level
	/// requirements.)
	pub b_auto_issue: bool,
	/// If true, this autoissue power keeps track of the actual level it was purchased at, instead of the default behavior of setting
	/// its level to its available level.
	pub b_auto_issue_save_level: bool,
	/// If true, this power doesn't count towards the player's current count of powers.
	pub b_free: bool,
	/// UI string
	pub pch_display_name: Option<String>,
	/// UI string
	pub pch_display_help: Option<String>,
	/// UI string
	pub pch_display_short_help: Option<String>,
	/// UI string
	pub pch_display_target_help: Option<String>,
	/// UI string
	pub pch_display_target_short_help: Option<String>,
	/// Message displayed as chat when the power is executed.
	pub pch_display_attacker_attack: Option<String>,
	/// Floater text when the power is executed.
	pub pch_display_attacker_attack_floater: Option<String>,
	/// Message displayed to the attacker when he hits with this power.
	pub pch_display_attacker_hit: Option<String>,
	/// Message displayed to the victim when he gets hits with this power.
	pub pch_display_victim_hit: Option<String>,
	/// Message displayed to the victim to confirm the power.
	pub pch_display_confirm: Option<String>,
	/// Message to float when this power is given as a reward.
	pub pch_display_float_rewarded: Option<String>,
	/// Message to float when this power is the defensive cause of a missed attack, and the attribmod does not specify its own float.
	pub pch_display_defense_float: Option<String>,
	/// Name of icon which represents this power.
	pub pch_icon_name: Option<String>,
	/// The type of power: auto, click, or toggle.
	pub e_type: PowerType,
	/// For temporary powers, the number of this power which they are allowed to have in their inventory. For other power types,
	/// this value is unused.
	pub i_num_allowed: i32,
	/// The list of attack groups this power is part of. Characters have defenses against each group individually.
	pub pe_attack_types: Vec<SpecialAttrib>,
	/// This requires statement is checked to see if the player is allowed to buy the power or have it autoissued.
	/// The requirements expression for this power. If empty, the power's only requirement is that the character is high enough level.
	/// Otherwise, this is a postfix expression (each element being an operand or operator) evaluated to determine if the power is
	/// available to the player.
	pub ppch_buy_requires: Vec<String>,
	/// This requires statement is checked to see if the player is allowed to activate the power. Ignored for Accesslevel.
	pub ppch_activate_requires: Vec<String>,
	/// The requirements expression for slotting this boost. If empty, there are no requirements for this boost.
	pub ppch_slot_requires: Vec<String>,
	/// The requirements expression for what this power targets. If empty, there are no requirements for the targets of this power.
	pub ppch_target_requires: Vec<String>,
	/// The requirements expression for when this power can be granted through the reward system.
	/// If empty, there are no requirements for when this power can be granted.
	pub ppch_reward_requires: Vec<String>,
	/// The requirements expression to determine if this power can be listed in the AuctionHouse.
	/// If empty, there are no requirements needed to be listed in the AuctionHouse.
	pub ppch_auction_requires: Vec<String>,
	/// The power that will be granted if the `ppch_reward_requires` is present and not met.
	/// If empty, then no power will be granted if the `ppch_reward_requires` is present and not met.
	pub pch_reward_fallback: Option<String>,
	/// Probability. Chance to hit.
	pub f_accuracy: f32,
	/// Requires the attacker to be on the the ground to succeed.
	pub b_near_ground: bool,
	/// Requires the target to be on the the ground to succeed.
	pub b_target_near_ground: bool,
	/// Determines whether the power can be used only while alive, only while dead, or while either dead or alive.
	pub e_death_castable_setting: DeathCastableSetting,
	/// Allows the power to be cast by the character while held.
	pub b_cast_through_hold: bool,
	/// Allows the power to be cast by the character while asleep.
	pub b_cast_through_sleep: bool,
	/// Allows the power to be cast by the character while stunned/disoriented.
	pub b_cast_through_stun: bool,
	/// Allows the power to be cast by the character while terrorized/frightened.
	pub b_cast_through_terrorize: bool,
	/// Allows the power (if a toggle) to remain active while the character is held.
	pub b_toggle_ignore_hold: bool,
	/// Allows the power (if a toggle) to remain active while the character is asleep.
	pub b_toggle_ignore_sleep: bool,
	/// Allows the power (if a toggle) to remain active while the character is stunned/disoriented.
	pub b_toggle_ignore_stun: bool,
	/// If true, this power will work regardless of when the power was bought. This is used for certain boostsets and accolades.
	pub b_ignore_level_bought: bool,
	/// Allows the power to ignore the untouchable aspect of the target.
	pub b_shoot_through_untouchable: bool,
	/// Specifies that this power is only interrupted by attribmods that would also cancel sleep, rather than all foe attribmods.
	pub b_interrupt_like_sleep: bool,
	/// Specifies when and if the AI is told about attacks with this power.
	pub e_ai_report: AIReport,
	/// Coverage of effect: target, cone, sphere
	pub e_effect_area: EffectArea,
	/// Maximum number of targets allowed to be hit by the power, used to limit AoE attacks. If more than MaxTargets could be
	/// hit, the ones farthest from the target point are rejected. This is only used when `e_effect_area` is Sphere or Cone.
	pub i_max_targets_hit: i32,
	/// Radius of effect around target, in feet.
	pub f_radius: f32,
	/// Spherical radians of the cone, centered around a ray connecting the attacker to the target.
	pub f_arc: f32,
	pub f_unknown: f32,
	/// For the chain effect area, add an optional delay between each jump.
	pub f_chain_delay: f32,
	/// If set, this expression is evaluated for each chain target beyond the first.
	/// It should evaluate to a number, which is stored as @ChainEff and used as the AttribMod's Effectiveness. This
	/// affects the total value that is applied by the power.
	pub ppch_chain_eff: Vec<String>,
	/// Which jumps the chain should create a new fork after. The same jump may be listed more than once to have more than one extra fork.
	pub pi_chain_fork: Vec<i32>,
	/// Used to define a cuboid volume positioned relative to the target, aligned to the basic x/y/z axes.
	pub vec_box_offset: Vec3,
	/// Used to define a cuboid volume positioned relative to the target, aligned to the basic x/y/z axes.
	pub vec_box_size: Vec3,
	/// Max distance to target in feet.
	pub f_range: f32,
	/// Max distance to secondary target in feet.
	pub f_range_secondary: f32,
	/// How long it takes in seconds to do the whole attack (including wind up, time it takes to strike the target (or launch), and
	/// the follow through.
	pub f_time_to_activate: f32,
	/// The time in seconds after a power is used that the power can be used again.
	pub f_recharge_time: f32,
	/// Time in seconds between automatic activations of the power.
	pub f_activate_period: f32,
	/// Cost of activation in endurance units.
	pub f_endurance_cost: f32,
	/// Cost of activation in insight units.
	pub f_insight_cost: f32,
	/// If non-zero, each player affected by the power must confirm that they want to be hit. The player has the given number of
	/// seconds to confirm. If they do nothing, the power is cancelled. Endurance is NOT given back to the caster if cancelled.
	pub i_time_to_confirm: i32,
	/// If true, a confirmation dialog will display for self-targeted powers.
	/// The default behavior is only other-targeted powers will use a confirmation window.
	pub b_self_confirm: bool,
	/// If the target of this power fails this requires statement, they will not receive a confirmation dialog for this power.
	/// The power will go off without the target's consent.
	pub ppch_confirm_requires: Vec<String>,
	/// If true, the power is removed from the character's power inventory if it reaches the usage limit.
	pub b_destroy_on_limit: bool,
	/// If true, this power will extend existing powers when granted multiple times.
	pub b_stacking_usage: bool,
	/// The number of times the power can be used.
	pub i_num_charges: i32,
	/// The max number of charges that `i_num_charges` can be extended to.
	pub i_max_num_charges: i32,
	/// The number of seconds which the power can be "on" overall. Used on toggle powers only.
	pub f_usage_time: f32,
	/// The max number of seconds that `f_usage_time` can be extended to.
	pub f_max_usage_time: f32,
	/// The number of seconds the power will function.
	pub f_lifetime: f32,
	/// The max number of seconds that `f_lifetime` can be extended to.
	pub f_max_lifetime: f32,
	/// The number of in-game seconds the power will function.
	pub f_lifetime_in_game: f32,
	/// The max number of seconds that `f_lifetime_in_game` can be extended to.
	pub f_max_lifetime_in_game: f32,
	/// The period of time in seconds, starting at the beginning of the attack, where the attack can be interrupted.
	pub f_interrupt_time: f32,
	/// Specifies what kind of visibility is required between the caster and target.
	pub e_target_visibility: TargetVisibility,
	/// What things can be targetted by this power.
	pub e_target_type: TargetType,
	/// What things can be targetted by this power.
	pub e_target_type_secondary: TargetType,
	/// Entity types of things which are always affected.
	pub p_auto_hit: Vec<TargetType>,
	/// Entoty types of things which are affected.
	pub p_affected: Vec<TargetType>,
	/// If true, can target, affect, and auto-hit things that are in a different vision phase.
	pub b_targets_through_vision_phase: bool,
	/// List of boost types allowed by the power
	pub pe_boosts_allowed: Vec<SpecialAttrib>,
	/// List of power groups this power belongs to. Only one of the powers in a power group can be on at a time. Using another
	/// power from the same group shuts off any other powers which are on.
	pub pe_group_membership: Vec<SpecialAttrib>,
	/// If any modes are listed, the character must be in one of the modes to activate the power. If the character exits a mode which is
	/// required by the power, it is shut off.
	pub pe_modes_required: Vec<SpecialAttrib>,
	/// If the character goes into any of the modes listed here, the power is shut off (if it's a toggle or auto) and the
	/// character will be unable to execute it.
	pub pe_modes_disallowed: Vec<SpecialAttrib>,
	/// List of AI groups this power belongs to. Determines how a particular power is to be used.
	pub ppch_ai_groups: Vec<String>,
	/// Unknown string array next to `pp_redirect`.
	pub ppch_unknown: Vec<String>,
	/// List of redirections for this power.
	pub pp_redirect: Vec<PowerRedirect>,
	/// Effects of this power.
	/// Stored as `Rc` because I need to make references to these for effects pulled in by `pp_redirect`.
	pub pp_effects: Vec<Rc<EffectGroup>>,
	/// Ignore all `AttribMod` strength modifiers when calculating the final strength for the power.
	pub b_ignore_strength: bool,
	/// If true, then the buff icon is shown for this power, otherwise it is not.
	pub b_show_buff_icon: bool,
	/// Determines whether the power is shown in the power inventory or not.
	pub e_show_in_inventory: ShowPowerSetting,
	/// If true, then the power is shown in the power management (i.e. the enhancement) and enh slot assignment screens, otherwise
	/// it is not.
	pub b_show_in_manage: bool,
	/// If true, then the power is shown in the power tab of the Player Info window. Also, if this power is in the Inherent powerset,
	/// it will show up in the contact dialog notifications that occur when new powers are automatically granted upon training.
	pub b_show_in_info: bool,
	///  If true AND this power is a temp power, it may be deleted by the player.
	pub b_deletable: bool,
	/// If this is true, the power may be traded to another player.
	pub b_tradeable: bool,
	/// Maximum number of boosts (aka enhancements) which can be on this power. This includes even free boosts given.
	pub i_max_boosts: i32,
	/// If true, then the power will never be saved to the database. This means that it will vanish on mapmoves, logout, and
	/// disconnects. Useful for temporary powers which you never want to leave a zone.
	pub b_do_not_save: bool,
	/// For Boosts, if this is true, then the boost's level relative to the character level does not impact its effectiveness.
	pub b_boost_ignore_effectiveness: bool,
	/// For Boosts, if this is true, then the boost is always counted as part of the set, even if exemplared below the level of the boost.
	pub b_boost_always_count_for_set: bool,
	/// For Boosts, if this is true, then the boost can be combined.
	pub b_boost_combinable: bool,
	/// For Inspirations and Boosts, if this is true, then the boost can be traded.
	pub b_boost_tradeable: bool,
	/// For Inspirations and Boosts, if this is true, then the boost can only be traded to other characters on the same account.
	pub b_boost_account_bound: bool,
	/// For Boosts, if this is true, then the boost can be combined with enhancement boosters.
	pub b_boost_boostable: bool,
	/// For Boosts, if this is true, then the boost uses the player's level rather than the boost's level.
	pub b_boost_use_player_level: bool,
	/// For Boosts, if set, the boost may be combined with an enhancement catalyst to create the specified enhancement.
	pub pch_boost_catalyst_conversion: Option<String>,
	/// For Boosts & Inspirations, if set, the item will not display on the auction house if the item is not published on the store.
	pub pch_store_product: Option<String>,
	/// For Boosts, the lowest level of boost that a character can slot that requires an invention license.
	pub i_boost_invention_license_required_level: i32,
	/// For Boosts, the lowest level a character can be to slot this boost.
	pub i_min_slot_level: i32,
	/// For Boosts, the highest level a character can be to slot this boost.
	pub i_max_slot_level: i32,
	/// For Boosts the use player level, the highest level that will be used to calc boost power
	pub i_max_boost_level: i32,
	/// List of variables which can be referenced by attrib mods on this power. These variables refer to values stored for each instance
	/// of the base power (in struct `Power`). The major purpose of these vars is for the Invention system, where they will be used to
	/// make Boosts.
	/// i26p5: Changed from its own struct to a simple array of `SpecialAttrib`.
	pub pp_vars: Vec<SpecialAttrib>,
	/// If this power is a toggle power, specifies how/when it responds to a `kDropToggle` `AttribMod`. The default is sometimes.
	pub e_toggle_droppable: ToggleDroppable,
	/// Whether this power applies all, none, or some of the non-template boosts that are slotted in it.
	pub e_proc_allowed: ProcAllowed,
	/// A list of character attributes whose strength cannot be modified. This can be used to make a Range buff not affect a power, for
	/// example.
	pub p_strengths_disallowed: Vec<SpecialAttrib>,
	/// True if the power is an AoE but you only want procs to go off once (on the main target) instead of on all targets.
	pub b_use_non_boost_templates_on_main_target: bool,
	/// If true, only the main target is animated and has FX put on him. Otherwise, everyone in the effect area will have the
	/// hit and block bits/fx applied.
	pub b_main_target_only: bool,
	/// The power will be highlighted in the UI when this is true.
	pub ppch_highlight_eval: Vec<String>,
	/// This icon will be shown when `ppch_highlight_eval` is true.
	pub pch_highlight_icon: Option<String>,
	/// A ring of this color will be shown around the power's icon when `ppch_highlight_eval` is true.
	pub rgba_highlight_ring: RGBA,
	/// If not zero, add this length of time to the combat travel power suppression.
	pub f_travel_suppression: f32,
	///	Default this to 1.0f.
	pub f_preference_multiplier: f32,
	/// Default this to false.
	pub b_dont_set_stance: bool,
	/// Values for gauging the "worth" of a power, used only in AE.
	pub f_point_val: f32,
	/// Values for gauging the "worth" of a power, used only in AE.
	pub f_point_multiplier: f32,
	pub pch_chain_into_power_name: Option<String>,
	pub b_instance_locked: bool,
	pub b_is_environment_hit: bool,
	pub b_shuffle_target_list: bool,
	pub i_force_level_bought: i32,
	/// This power is revoked and then granted again to any player that currently owns it when:
	/// * An Active Player reward token changes state for the team's current Active Player,
	/// * The team's current Active Player changes,
	/// * or the player joins or leaves a team.
	///
	/// The initial purpose for this flag is to refresh vision phase powers automatically.
	pub b_refreshes_on_active_player_change: bool,
	pub b_cancelable: bool,
	pub b_ignore_toggle_max_distance: bool,
	pub i_server_tray_priority: i32,
	pub ppch_server_tray_requires: Vec<String>,
	/// If true, attrib mods applied by this power will be cleared whenever entering a map that uses `eRAT_ClearAbusiveBuffs`.
	pub b_abusive_buff: bool,
	pub e_position_center: ModTarget,
	pub f_position_distance: f32,
	pub f_position_height: f32,
	pub f_position_yaw: f32,
	pub b_face_target: bool,
	/// Cache of attributes that can be modified by this power.
	pub pe_attrib_cache: Vec<SpecialAttrib>,
	/// Describes the visual effects of this power.
	pub p_fx: Option<PowerFX>,
	/// Per-costume selectable overrides for `fx`.
	pub pp_custom_fx: Vec<CustomPowerFX>,
	/// Added i26p5. Expression that determines the number of max target hits.
	pub ppch_max_targets_expr: Vec<String>,
	/// Added i26p5. Not sure what these are, appears to be either "1" or "3" and related to chain powers.
	pub pi_unknown: Vec<i32>,

	// Non-data fields.
	/// Whether or not to include this power in the output files.
	pub include_in_output: bool,
	/// Archetypes associated with this power.
	pub archetypes: Vec<Rc<Archetype>>,
	/// Have we resolved redirects on this power already?
	pub redirects_resolved: bool,
	/// Computed set of enhancement sets allowed.
	pub enhancement_set_categories_allowed: HashSet<String>,
}

impl BasePower {
	pub fn new() -> Self {
		Default::default()
	}
}

/// Describes a power category as containing either primary or secondary sets.
#[derive(Copy, Clone, Debug)]
pub enum PrimarySecondary {
	Primary,
	Secondary,
	None,
}

impl Default for PrimarySecondary {
	fn default() -> Self {
		PrimarySecondary::None
	}
}

#[derive(Debug, Default)]
pub struct PowerCategory {
	/// Filename this definition came from.
	pub pch_source_file: Option<String>,
	/// Internal name.
	pub pch_name: Option<NameKey>,
	/// Name the user sees.
	pub pch_display_name: Option<String>,
	/// Used for UI.
	pub pch_display_help: Option<String>,
	/// Used for UI.
	pub pch_display_short_help: Option<String>,
	/// The names of power sets in this category.
	pub ppch_power_set_names: Vec<NameKey>,
	/// List of power sets which make up this category.
	pub pp_power_sets: Vec<Rc<BasePowerSet>>,
	/// Archetypes associated with this category.
	pub archetypes: Vec<Rc<Archetype>>,
	/// For power categories tied to a specific archetype, this indicates whether it
	/// is a primary or secondary power pick.
	pub pri_sec: PrimarySecondary,

	// Non-data fields.
	/// Whether or not to include this power category in the output files.
	pub include_in_output: bool,
	/// If true, this category should be listed in the root JSON.
	pub top_level: bool,
}

impl PowerCategory {
	pub fn new() -> Self {
		Default::default()
	}
}

#[derive(Debug, Default)]
pub struct AttribName {
	pub pch_name: Option<String>,
	pub pch_display_name: Option<String>,
	pub pch_icon_name: Option<String>,
}

impl AttribName {
	pub fn new() -> Self {
		Default::default()
	}
}

#[derive(Debug, Default)]
pub struct AttribNames {
	pub pp_defense: Vec<AttribName>,
	pub pp_damage: Vec<AttribName>,
	pub pp_boost: Vec<AttribName>,
	pub pp_group: Vec<AttribName>,
	pub pp_mode: Vec<AttribName>,
	pub pp_elusivity: Vec<AttribName>,
	pub pp_stack_key: Vec<AttribName>,
	/// Not in the original struct but gives us a convenient place to hold onto them.
	pub attr_names: HashMap<usize, Option<String>>,
}

impl AttribNames {
	pub fn new() -> Self {
		Default::default()
	}
}

/// Custom struct for holding all of the parsed data.
#[derive(Debug)]
pub struct PowersDictionary {
	/// Contains the full hierarchy of power categories -> power sets -> powers.
	pub power_categories: Vec<Rc<PowerCategory>>,
	/// All of the archetype data.
	pub archetypes: Keyed<Archetype>,
	/// Character attribute names, mostly used for naming damage, defense, elusivity.
	pub attrib_names: AttribNames,
}
