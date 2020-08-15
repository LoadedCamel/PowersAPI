use super::*;

/// We use this structure to load references to powers from data files.
#[derive(Debug, Default)]
pub struct PowerNameRef {
    pub power_category: Option<NameKey>,
    pub power_set: Option<NameKey>,
    pub power: Option<NameKey>,
    pub level: i32,
    /// Used for rewarding only: sets the level delta or the actual level depending on `fixed_level`.
    pub def_level: i32,
    /// Used for rewarding only: if `true`, removes the named power.
    pub remove: i32,
    /// Used for rewarding only: if `true`, `def_level` is a fixed reward level to give.
    pub fixed_level: i32,
    pub dont_set_stance: i32,
}

impl PowerNameRef {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct VillainLevelDef {
    /// What is the villain level is this definition for?
    pub level: i32,
    /// How much experience do I reward the player when defeated?
    pub experience: i32,
    /// What are the possible display names for the villain at this level?
    pub display_names: Vec<String>,
    /// Costume names, which references entries in NPC definitions.
    pub costumes: Vec<String>,
}

impl VillainLevelDef {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct PetCommandStrings {
    pub ppch_passive: Vec<String>,
    pub ppch_defensive: Vec<String>,
    pub ppch_aggressive: Vec<String>,
    pub ppch_attack_target: Vec<String>,
    pub ppch_attack_no_target: Vec<String>,
    pub ppch_stay_here: Vec<String>,
    pub ppch_use_power: Vec<String>,
    pub ppch_use_power_none: Vec<String>,
    pub ppch_follow_me: Vec<String>,
    pub ppch_goto_spot: Vec<String>,
    pub ppch_dismiss: Vec<String>,
}

impl PetCommandStrings {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Defines different villain (NPC) templates. This is used to look up the definition
/// of entities (such as pets and pseudopets) created by powers.
#[derive(Debug, Default)]
pub struct VillainDef {
    /// Internal name.  NPCs should be referenced by this name.
    pub name: Option<NameKey>,
    pub gender: Gender,
    /// What kind of stats should I have?
    pub character_class_name: Option<NameKey>,
    /// Villain description.
    pub description: Option<String>,
    /// An override for the villain group description.
    pub group_description: Option<String>,
    /// An override for the class display name.
    pub display_class_name: Option<String>,
    /// What `AIConfig` should I use?
    pub ai_config: Option<String>,
    /// Tags which can be checked in powers for special effects.
    pub power_tags: Vec<String>,
    /// In addition to the class reward, what other rewards do I give?
    pub additional_rewards: Vec<String>,
    /// For skill objects destroyed via HP, what rewards do I give?
    pub skill_hp_rewards: Vec<String>,
    /// For skill objects destroyed via Status, what rewards do I give?
    pub skill_status_rewards: Vec<String>,
    /// Am I a hero, villain, or monster? (Default is monster.)
    pub ally: Option<String>,
    /// What gang am I on? Any string will do.
    pub gang: Option<String>,
    /// Scale XP and influence given out by this value.
    pub reward_scale: f32,
    /// Villain should only appear in this version of the game.
    pub exclusion: VillainExclusion,
    /// Weapon to use in encounter inactive state when you carry a weapon (linked to animation).
    pub favorite_weapon: Option<String>,
    /// If `true`, this villain's attacks and defenses will ignore combat modifiers.
    pub ignore_combat_mods: bool,
    /// If `true`, when spawned as a pet this villain gets a copy of its creator's attribute modifiers.
    pub copy_creator_mods: bool,
    /// The maximum number of this villain that will be put in one spawn in non-mission maps (`-1` = no limit).
    pub spawn_limit: i32,
    /// The maximum number of this villain that will be put in one spawn in mission maps (`-1` = no limit, defaults to `spawn_limit`).
    pub spawn_limit_mission: i32,
    /// If `true`, this villain will not be reduced from an arch-villain to an elite boss.
    pub ignore_reduction: bool,
    /// If `true`, this villain can follow its creator across zone boundaries.
    pub can_zone: bool,
    /// Increment this badge stat on a kill of this villain.
    pub custom_badge_stat: Option<String>,
    pub flags: VillainDefFlags,
    /// What is my rank?
    pub rank: VillainRank,
    /// Which villain group do I belong to?
    pub group: i32,
    /// What are my possible powers?
    pub powers: Vec<PowerNameRef>,
    pub levels: Vec<VillainLevelDef>,
    /// If I am a pet, what power, if any, do I wait for my master to tell me to use? (Power must also be in my `powers` list.)
    pub special_pet_power: Option<String>,
    /// For pets, responses for when they are commanded.
    pub pet_command_strings: Vec<PetCommandStrings>,
    /// For pets, whether or not to display in pet window.
    pub pet_visibility: i32,
    /// For pets, can they be commanded?
    pub pet_commandability: i32,
    /// For debugging.
    pub file_name: Option<String>,
    /// File modification time.
    pub file_age: u32,
    /// Entry proccess time.
    pub process_age: u32,
}

impl VillainDef {
    pub fn new() -> Self {
        Default::default()
    }
}
