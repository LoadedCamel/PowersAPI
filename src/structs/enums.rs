#![allow(dead_code)]
#![allow(non_camel_case_types)]
use super::{NameKey, VillainDef};
use num_enum::TryFromPrimitive;
use std::rc::Rc;

macro_rules! default_new {
    ($type:ty) => {
        impl $type {
            pub fn new() -> Self {
                Default::default()
            }
        }
    };
}

macro_rules! default_val {
    ($type:ty, $typeval:ident) => {
        default_new!($type);
        impl Default for $type {
            fn default() -> Self {
                <$type>::$typeval
            }
        }
    };
}

/// A very large number of seconds which is essentially forever. This is used as a flag; anything larger than or
/// equal to this value will be handled specially.
const ATTRIBMOD_DURATION_FOREVER: f32 = 99999.0;

/// Which power system to use for advancement, level lookup, etc.
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum PowerSystem {
    kPowerSystem_Powers = 0,
}
default_val!(PowerSystem, kPowerSystem_Powers);

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum ShowPowerSetting {
    /// If on a powerset that the player owns, do not show this powerset or any powers in it (no matter what settings the powers have).
    /// If on a power, don't show it (if the powerset allows the power to control its display).
    kShowPowerSetting_Never = 0, // old false.
    /// If on a powerset that the player owns, show this powerset, but then allow the power to determine whether it is shown.
    /// If on a power, show it (if the powerset allows the power to control its display).
    kShowPowerSetting_Default = 1, // old true.
    /// If on a powerset that the player owns, show this powerset and all powers in it (no matter what settings the powers have).
    /// If on a power, show it (if the powerset allows the power to control its display).
    kShowPowerSetting_Always,
    /// If on a powerset that the player owns, display it, and display all powers in the powerset only if they are usable (no matter what settings the powers have).
    /// If on a power, display it only if it is usable (if the powerset allows the power to control its display).
    kShowPowerSetting_IfUsable,
    /// If on a powerset that the player owns, display it, and display all powers in the powerset only if they are owned by the player (no matter what settings the powers have).
    /// If on a power, display it only if it is owned by the player (if the powerset allows the power to control its display).
    kShowPowerSetting_IfOwned,
}
default_val!(ShowPowerSetting, kShowPowerSetting_Never);

/// Defines if the power is auto, toggle, or click power.
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum PowerType {
    /// Click powers only activate when the user has activated them.
    kPowerType_Click,
    /// Auto powers activate repeatedly.
    kPowerType_Auto,
    /// Toggle powers activate repeatedly when the character turns them on until they shut them off (or the character runs out of endurance).
    kPowerType_Toggle,
    /// Boosts apply to powers only.
    kPowerType_Boost,
    kPowerType_Inspiration,
    kPowerType_GlobalBoost,
}
default_val!(PowerType, kPowerType_Click);

impl PowerType {
    pub fn get_string(&self) -> &'static str {
        match self {
            PowerType::kPowerType_Auto => "Auto",
            PowerType::kPowerType_Boost => "Enhancement",
            PowerType::kPowerType_Click => "Click",
            PowerType::kPowerType_GlobalBoost => "Global Enhancement",
            PowerType::kPowerType_Inspiration => "Inspiration",
            PowerType::kPowerType_Toggle => "Toggle",
        }
    }
}

// see ESpecialAttrib in Common/entity/character_attribs.h
#[derive(Debug)]
pub enum SpecialAttrib {
    kSpecialAttrib_Character(i32),
    kSpecialAttrib_Translucency,
    kSpecialAttrib_EntCreate,
    kSpecialAttrib_ClearDamagers,
    kSpecialAttrib_SilentKill,
    kSpecialAttrib_XPDebtProtection,
    kSpecialAttrib_SetMode,
    kSpecialAttrib_SetCostume,
    kSpecialAttrib_Glide,
    kSpecialAttrib_Null,
    kSpecialAttrib_Avoid,
    kSpecialAttrib_Reward,
    kSpecialAttrib_XPDebt,
    kSpecialAttrib_DropToggles,
    kSpecialAttrib_GrantPower,
    kSpecialAttrib_RevokePower,
    kSpecialAttrib_UnsetMode,
    kSpecialAttrib_GlobalChanceMod,
    kSpecialAttrib_PowerChanceMod,
    kSpecialAttrib_GrantBoostedPower,
    kSpecialAttrib_ViewAttrib,
    kSpecialAttrib_RewardSource,
    kSpecialAttrib_RewardSourceTeam,
    kSpecialAttrib_ClearFog,
    kSpecialAttrib_CombatPhase,
    kSpecialAttrib_CombatModShift,
    kSpecialAttrib_RechargePower,
    kSpecialAttrib_VisionPhase,
    kSpecialAttrib_NinjaRun,
    kSpecialAttrib_Walk,
    kSpecialAttrib_BeastRun,
    kSpecialAttrib_SteamJump,
    kSpecialAttrib_DesignerStatus,
    kSpecialAttrib_ExclusiveVisionPhase,
    kSpecialAttrib_HoverBoard,
    kSpecialAttrib_SetSZEValue,
    kSpecialAttrib_AddBehavior,
    kSpecialAttrib_MagicCarpet,
    kSpecialAttrib_TokenAdd,
    kSpecialAttrib_TokenSet,
    kSpecialAttrib_TokenClear,
    kSpecialAttrib_LuaExec,
    kSpecialAttrib_ForceMove,
    kSpecialAttrib_ParkourRun,
    kSpecialAttrib_CancelMods,
    kSpecialAttrib_ExecutePower,
    kSpecialAttrib_PowerRedirect,
    kSpecialAttrib_UNSET,
}
default_val!(SpecialAttrib, kSpecialAttrib_UNSET);

impl SpecialAttrib {
    pub fn from_i32(val: i32) -> Self {
        match val {
            460 => SpecialAttrib::kSpecialAttrib_Translucency, /* sizeof(CharacterAttributes) */
            461 => SpecialAttrib::kSpecialAttrib_EntCreate,
            462 => SpecialAttrib::kSpecialAttrib_ClearDamagers,
            463 => SpecialAttrib::kSpecialAttrib_SilentKill,
            464 => SpecialAttrib::kSpecialAttrib_XPDebtProtection,
            465 => SpecialAttrib::kSpecialAttrib_SetMode,
            466 => SpecialAttrib::kSpecialAttrib_SetCostume,
            467 => SpecialAttrib::kSpecialAttrib_Glide,
            468 => SpecialAttrib::kSpecialAttrib_Null,
            469 => SpecialAttrib::kSpecialAttrib_Avoid,
            470 => SpecialAttrib::kSpecialAttrib_Reward,
            471 => SpecialAttrib::kSpecialAttrib_XPDebt,
            472 => SpecialAttrib::kSpecialAttrib_DropToggles,
            473 => SpecialAttrib::kSpecialAttrib_GrantPower,
            474 => SpecialAttrib::kSpecialAttrib_RevokePower,
            475 => SpecialAttrib::kSpecialAttrib_UnsetMode,
            476 => SpecialAttrib::kSpecialAttrib_GlobalChanceMod,
            477 => SpecialAttrib::kSpecialAttrib_PowerChanceMod,
            478 => SpecialAttrib::kSpecialAttrib_GrantBoostedPower,
            479 => SpecialAttrib::kSpecialAttrib_ViewAttrib,
            480 => SpecialAttrib::kSpecialAttrib_RewardSource,
            481 => SpecialAttrib::kSpecialAttrib_RewardSourceTeam,
            482 => SpecialAttrib::kSpecialAttrib_ClearFog,
            483 => SpecialAttrib::kSpecialAttrib_CombatPhase,
            484 => SpecialAttrib::kSpecialAttrib_CombatModShift,
            485 => SpecialAttrib::kSpecialAttrib_RechargePower,
            486 => SpecialAttrib::kSpecialAttrib_VisionPhase,
            487 => SpecialAttrib::kSpecialAttrib_NinjaRun,
            488 => SpecialAttrib::kSpecialAttrib_Walk,
            489 => SpecialAttrib::kSpecialAttrib_BeastRun,
            490 => SpecialAttrib::kSpecialAttrib_SteamJump,
            491 => SpecialAttrib::kSpecialAttrib_DesignerStatus,
            492 => SpecialAttrib::kSpecialAttrib_ExclusiveVisionPhase,
            493 => SpecialAttrib::kSpecialAttrib_HoverBoard,
            494 => SpecialAttrib::kSpecialAttrib_SetSZEValue,
            495 => SpecialAttrib::kSpecialAttrib_AddBehavior,
            496 => SpecialAttrib::kSpecialAttrib_MagicCarpet,
            497 => SpecialAttrib::kSpecialAttrib_TokenAdd,
            498 => SpecialAttrib::kSpecialAttrib_TokenSet,
            499 => SpecialAttrib::kSpecialAttrib_TokenClear,
            500 => SpecialAttrib::kSpecialAttrib_LuaExec,
            501 => SpecialAttrib::kSpecialAttrib_ForceMove,
            502 => SpecialAttrib::kSpecialAttrib_ParkourRun,
            503 => SpecialAttrib::kSpecialAttrib_CancelMods,
            504 => SpecialAttrib::kSpecialAttrib_ExecutePower,
            1460 => SpecialAttrib::kSpecialAttrib_PowerRedirect,
            _ => SpecialAttrib::kSpecialAttrib_Character(val),
        }
    }

    pub fn get_string(&self) -> &'static str {
        match self {
            SpecialAttrib::kSpecialAttrib_UNSET => "",
            SpecialAttrib::kSpecialAttrib_Character(_) => "Character Attribute",
            SpecialAttrib::kSpecialAttrib_Translucency => "Translucency",
            SpecialAttrib::kSpecialAttrib_EntCreate => "Create Entity",
            SpecialAttrib::kSpecialAttrib_ClearDamagers => "Clear Damagers",
            SpecialAttrib::kSpecialAttrib_SilentKill => "Silent Kill",
            SpecialAttrib::kSpecialAttrib_XPDebtProtection => "Debt Protection",
            SpecialAttrib::kSpecialAttrib_SetMode => "Set Mode",
            SpecialAttrib::kSpecialAttrib_SetCostume => "Set Costume",
            SpecialAttrib::kSpecialAttrib_Glide => "Glide",
            SpecialAttrib::kSpecialAttrib_Null => "Null",
            SpecialAttrib::kSpecialAttrib_Avoid => "Avoid",
            SpecialAttrib::kSpecialAttrib_Reward => "Reward",
            SpecialAttrib::kSpecialAttrib_XPDebt => "Debt",
            SpecialAttrib::kSpecialAttrib_DropToggles => "Drop Toggles",
            SpecialAttrib::kSpecialAttrib_GrantPower => "Grant Power",
            SpecialAttrib::kSpecialAttrib_RevokePower => "Revoke Power",
            SpecialAttrib::kSpecialAttrib_UnsetMode => "Unset Mode",
            SpecialAttrib::kSpecialAttrib_GlobalChanceMod => "Global Chance Mod",
            SpecialAttrib::kSpecialAttrib_PowerChanceMod => "Power Chance Mod",
            SpecialAttrib::kSpecialAttrib_GrantBoostedPower => "Grant Boosted Power",
            SpecialAttrib::kSpecialAttrib_ViewAttrib => "View Attributes",
            SpecialAttrib::kSpecialAttrib_RewardSource => "Reward Source",
            SpecialAttrib::kSpecialAttrib_RewardSourceTeam => "Reward Source Team",
            SpecialAttrib::kSpecialAttrib_ClearFog => "Clear Fog",
            SpecialAttrib::kSpecialAttrib_CombatPhase => "Combat Phase",
            SpecialAttrib::kSpecialAttrib_CombatModShift => "Level Shift",
            SpecialAttrib::kSpecialAttrib_RechargePower => "Recharge Power",
            SpecialAttrib::kSpecialAttrib_VisionPhase => "Vision Phase",
            SpecialAttrib::kSpecialAttrib_NinjaRun => "Ninja Run",
            SpecialAttrib::kSpecialAttrib_Walk => "Walk",
            SpecialAttrib::kSpecialAttrib_BeastRun => "Beast Run",
            SpecialAttrib::kSpecialAttrib_SteamJump => "Steam Jump",
            SpecialAttrib::kSpecialAttrib_DesignerStatus => "Designer Status",
            SpecialAttrib::kSpecialAttrib_ExclusiveVisionPhase => "Exclusive Vision Phase",
            SpecialAttrib::kSpecialAttrib_HoverBoard => "Hover Board",
            SpecialAttrib::kSpecialAttrib_SetSZEValue => "Set Script Value",
            SpecialAttrib::kSpecialAttrib_AddBehavior => "Add Behavior",
            SpecialAttrib::kSpecialAttrib_MagicCarpet => "Magic Carpet",
            SpecialAttrib::kSpecialAttrib_TokenAdd => "Add Token",
            SpecialAttrib::kSpecialAttrib_TokenSet => "Set Token",
            SpecialAttrib::kSpecialAttrib_TokenClear => "Clear Token",
            SpecialAttrib::kSpecialAttrib_LuaExec => "Execute Script",
            SpecialAttrib::kSpecialAttrib_ForceMove => "Force Move",
            SpecialAttrib::kSpecialAttrib_ParkourRun => "Parkour Run",
            SpecialAttrib::kSpecialAttrib_CancelMods => "Cancel Effects",
            SpecialAttrib::kSpecialAttrib_ExecutePower => "Execute Power",
            SpecialAttrib::kSpecialAttrib_PowerRedirect => "Redirect Power",
        }
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum DeathCastableSetting {
    kDeathCastableSetting_AliveOnly = 0, // old false.
    kDeathCastableSetting_DeadOnly = 1,  // old true.
    kDeathCastableSetting_DeadOrAlive = 2,
}
default_val!(DeathCastableSetting, kDeathCastableSetting_AliveOnly);

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum AIReport {
    /// Report on hit or miss.
    kAIReport_Always,
    /// Never report to AI.     
    kAIReport_Never,
    /// Report only when hit.      
    kAIReport_HitOnly,
    /// Report only when missed.    
    kAIReport_MissOnly,
}
default_val!(AIReport, kAIReport_Always);

/// The area effected by the power.
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum EffectArea {
    /// Any targeted entity
    kEffectArea_Character,
    /// A cone centered around the ray connecting the source to the target.
    kEffectArea_Cone,
    /// A sphere surrounding the target.
    kEffectArea_Sphere,
    /// A single spot on the ground.
    kEffectArea_Location,
    /// A linked chain of targets.
    kEffectArea_Chain,
    /// In the same volume as the caster.
    kEffectArea_Volume,
    /// In a volume named the same as the one the caster is in (not yet implemented).
    kEffectArea_NamedVolume,
    /// Everybody on the same map as the caster.
    kEffectArea_Map,
    /// In the same tray (room) as the caster.
    kEffectArea_Room,
    /// Capsules touch.
    kEffectArea_Touch,
    /// A box positioned relative to the target, oriented along the regular xyz axes.
    kEffectArea_Box,
}
default_val!(EffectArea, kEffectArea_Character);

impl EffectArea {
    /// Get a human readable string representing this `EffectArea`.
    pub fn get_string(&self) -> &'static str {
        match self {
            EffectArea::kEffectArea_Character => "SingleTarget",
            EffectArea::kEffectArea_Cone => "Cone",
            EffectArea::kEffectArea_Sphere => "AoE",
            EffectArea::kEffectArea_Location => "Location",
            EffectArea::kEffectArea_Chain => "Chain",
            EffectArea::kEffectArea_Volume => "Self",
            EffectArea::kEffectArea_NamedVolume => "(not implemented)",
            EffectArea::kEffectArea_Map => "Map",
            EffectArea::kEffectArea_Room => "Room",
            EffectArea::kEffectArea_Touch => "Touch",
            EffectArea::kEffectArea_Box => "Box",
        }
    }
}

/// Defines what kind of visibility is required between the caster and
/// the target for successful execution of the power.
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum TargetVisibility {
    /// The caster must have direct line of sight to the target.
    kTargetVisibility_LineOfSight,
    /// No visibility is required (or checked)
    kTargetVisibility_None,
}
default_val!(TargetVisibility, kTargetVisibility_LineOfSight);

/// The thing which can be targetted. Used to specify which kinds of entities are affected, auto-hit, etc. by a power.
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum TargetType {
    kTargetType_None,
    /// The caster, dead or alive.
    kTargetType_Caster,
    /// Any living players except the caster.              
    kTargetType_Player,
    /// Any living player on Team Hero except the caster.              
    kTargetType_PlayerHero,
    /// Any living player on Team Villain except the caster.          
    kTargetType_PlayerVillain,
    /// Any dead players including the caster.       
    kTargetType_DeadPlayer,
    /// Any dead players on the same side as the caster except the caster and skill checks.          
    kTargetType_DeadPlayerFriend,
    /// Any dead players on the same side as the caster except the caster and skill checks.    
    kTargetType_DeadPlayerFoe,
    /// Any living teammate and their pets except the caster.
    kTargetType_Teammate,
    /// Any dead teammate and all teammate's dead pets except the caster.           
    kTargetType_DeadTeammate,
    /// All teammates and their pets, dead or alive except the caster.        
    kTargetType_DeadOrAliveTeammate,
    /// Any living critter on team evil except the caster and skill checks.
    kTargetType_Villain,
    /// Any dead critter on team evil including the caster and skill checks.             
    kTargetType_DeadVillain,
    /// Any living NPC but villains and skill checks.         
    kTargetType_NPC,
    /// Anyone dead or alive on the same side as the caster except the caster and skill checks.
    kTargetType_DeadOrAliveFriend,
    /// Anyone dead on the same side as the caster except the caster and skill checks.   
    kTargetType_DeadFriend,
    /// Anyone alive on the same side as the caster except the caster and skill checks.          
    kTargetType_Friend,
    /// Anyone dead or alive on a different side from the caster except the caster and skill checks.
    kTargetType_DeadOrAliveFoe,
    /// Anyone dead on a different side from the caster except the caster and skill checks.      
    kTargetType_DeadFoe,
    /// Anyone alive on a different side from the caster except the caster and skill checks.   
    kTargetType_Foe,
    /// A specific location.
    kTargetType_Location,
    /// Any living entity which isn't dead.            
    kTargetType_Any,
    /// Added i26p5. Any dead entity.
    kTargetType_DeadAny,
    /// Added i26p5. Any dead or living entity.
    kTargetType_DeadOrAliveAny,
    /// A specific location with constraints for teleporting.                 
    kTargetType_Teleport,
    /// Any target where the source is the owner.
    kTargetType_DeadOrAliveMyPet,
    /// The target is dead and is the source is the owner.    
    kTargetType_DeadMyPet,
    /// The target is alive and is the source is the owner.           
    kTargetType_MyPet,
    /// The target is the owner of the caster (this goes all the way back up to the original owner).
    kTargetType_MyOwner,
    /// The target is the creator of the caster (this goes only one level up to the entity that created ent).
    kTargetType_MyCreator,
    ///	The target is alive and the source is the creator.
    kTargetType_MyCreation,
    /// The target is dead and the source is the creator
    kTargetType_DeadMyCreation,
    ///	Any target where the source is the creator.
    kTargetType_DeadOrAliveMyCreation,
    /// Any living leaguemate and their pets except the caster.
    kTargetType_Leaguemate,
    /// Any dead leaguemate and all leaguemate's dead pets except the caster.
    kTargetType_DeadLeaguemate,
    /// All leaguemates and their pets, dead or alive except the caster.
    kTargetType_DeadOrAliveLeaguemate,
    /// A position relative to an entity specified by the designer.
    kTargetType_Position,
}
default_val!(TargetType, kTargetType_None);

impl TargetType {
    pub fn get_strings(&self) -> Vec<&'static str> {
        let mut tt_tags = Vec::new();
        macro_rules! tags {
            ($($tag:literal),+) => {
               { $( tt_tags.push($tag); )+ }
            }
        }
        match self {
            TargetType::kTargetType_None => (),
            TargetType::kTargetType_Caster => tags!("Self", "Alive", "Dead"),
            TargetType::kTargetType_Player => tags!("Player", "Alive"),
            TargetType::kTargetType_PlayerHero => tags!("Player", "Alive", "Hero"),
            TargetType::kTargetType_PlayerVillain => tags!("Player", "Alive", "Villain"),
            TargetType::kTargetType_DeadPlayer => tags!("Player", "Dead"),
            TargetType::kTargetType_DeadPlayerFoe => tags!("Player", "Dead", "Foe"),
            TargetType::kTargetType_DeadPlayerFriend => tags!("Player", "Dead", "Friend"),
            TargetType::kTargetType_Teammate => tags!("Player", "Alive", "Team"),
            TargetType::kTargetType_DeadTeammate => tags!("Player", "Dead", "Team"),
            TargetType::kTargetType_DeadOrAliveTeammate => tags!("Player", "Alive", "Dead", "Team"),
            // possible point of confusion: player villains are CoV characters, NPC villains are any hostile mob
            TargetType::kTargetType_Villain => tags!("NPC", "Alive", "Foe"),
            TargetType::kTargetType_DeadVillain => tags!("NPC", "Dead", "Foe"),
            TargetType::kTargetType_NPC => tags!("NPC", "Friend"),
            TargetType::kTargetType_DeadOrAliveFriend => tags!("Alive", "Dead", "Friend"),
            TargetType::kTargetType_DeadFriend => tags!("Dead", "Friend"),
            TargetType::kTargetType_Friend => tags!("Friend"),
            TargetType::kTargetType_DeadOrAliveFoe => tags!("Alive", "Dead", "Foe"),
            TargetType::kTargetType_DeadFoe => tags!("Dead", "Foe"),
            TargetType::kTargetType_Foe => tags!("Foe"),
            TargetType::kTargetType_Location => tags!("Location"),
            TargetType::kTargetType_Any => tags!("Alive"),
            // changed: i26p5, DeadOrAliveAny can be seen in the UI for Defibrillate, DeadAny is an educated guess
            TargetType::kTargetType_DeadAny => tags!("Dead"),
            TargetType::kTargetType_DeadOrAliveAny => tags!("Dead", "Alive"),
            TargetType::kTargetType_Teleport => tags!("Location", "Teleport"),
            TargetType::kTargetType_DeadOrAliveMyPet => tags!("Pet", "Owned", "Dead", "Alive"),
            TargetType::kTargetType_DeadMyPet => tags!("Pet", "Owned", "Dead"),
            TargetType::kTargetType_MyPet => tags!("Pet", "Owned"),
            // another point of confusion: owner vs creator (root vs 1 level up)
            TargetType::kTargetType_MyOwner => tags!("Root_Owner"),
            TargetType::kTargetType_MyCreator => tags!("Owner"),
            TargetType::kTargetType_MyCreation => tags!("Owned", "Alive"),
            TargetType::kTargetType_DeadMyCreation => tags!("Owned", "Dead"),
            TargetType::kTargetType_DeadOrAliveMyCreation => tags!("Owned", "Alive", "Dead"),
            TargetType::kTargetType_Leaguemate => tags!("Player", "Alive", "Team", "League"),
            TargetType::kTargetType_DeadLeaguemate => tags!("Player", "Dead", "Team", "League"),
            TargetType::kTargetType_DeadOrAliveLeaguemate => {
                tags!("Player", "Alive", "Dead", "Team", "League")
            }
            TargetType::kTargetType_Position => tags!("Position"),
        }
        tt_tags
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum ModApplicationType {
    /// While the power is running.
    kModApplicationType_OnTick,
    /// When the power is turned on.
    kModApplicationType_OnActivate,
    /// When the power is turned off.
    kModApplicationType_OnDeactivate,
    /// A limited version of onDeactivate.
    kModApplicationType_OnExpire,
    /// When the power becomes able to be turned on.
    kModApplicationType_OnEnable,
    /// When the power becomes no longer able to be turned on
    kModApplicationType_OnDisable,
}
default_val!(ModApplicationType, kModApplicationType_OnTick);

impl ModApplicationType {
    pub fn get_string(&self) -> &'static str {
        match self {
            ModApplicationType::kModApplicationType_OnTick => "OnTick",
            ModApplicationType::kModApplicationType_OnActivate => "OnActivate",
            ModApplicationType::kModApplicationType_OnDeactivate => "OnDeactive",
            ModApplicationType::kModApplicationType_OnExpire => "OnExpire",
            ModApplicationType::kModApplicationType_OnEnable => "OnEnable",
            ModApplicationType::kModApplicationType_OnDisable => "OnDisable",
        }
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum ModTarget {
    kModTarget_Caster,
    kModTarget_CastersOwnerAndAllPets,
    kModTarget_Focus,
    kModTarget_FocusOwnerAndAllPets,
    kModTarget_Affected,
    kModTarget_AffectedsOwnerAndAllPets,
    kModTarget_Marker,
}
default_val!(ModTarget, kModTarget_Affected);

impl ModTarget {
    pub fn get_string(&self) -> &'static str {
        match self {
            ModTarget::kModTarget_Caster => "Self",
            ModTarget::kModTarget_CastersOwnerAndAllPets => "SelfAndPets",
            ModTarget::kModTarget_Focus => "Target",
            ModTarget::kModTarget_FocusOwnerAndAllPets => "TargetAndPets",
            ModTarget::kModTarget_Affected => "Affected",
            ModTarget::kModTarget_AffectedsOwnerAndAllPets => "AffectedAndPets",
            ModTarget::kModTarget_Marker => "Marker",
        }
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum ModType {
    kModType_Duration,
    kModType_Magnitude,
    kModType_Constant,
    kModType_Expression,
    kModType_SkillMagnitude,
}
default_val!(ModType, kModType_Magnitude);

impl ModType {
    pub fn get_string(&self) -> &'static str {
        match self {
            ModType::kModType_Duration => "Duration",
            ModType::kModType_Magnitude => "Magnitude",
            ModType::kModType_Constant => "Constant",
            ModType::kModType_Expression => "Expression",
            ModType::kModType_SkillMagnitude => "SkillMagnitude",
        }
    }
}

#[derive(Debug)]
pub enum ModDuration {
    InSeconds(f32),
    kModDuration_Instant,
    kModDuration_UntilKilled,
    kModDuration_UntilShutOff,
}

impl ModDuration {
    pub fn from_f32(val: f32) -> Self {
        if val == -1.0 {
            ModDuration::kModDuration_Instant
        } else if val >= ATTRIBMOD_DURATION_FOREVER {
            // UntilKilled and UntilShutoff have the same value
            ModDuration::kModDuration_UntilKilled
        } else {
            ModDuration::new(val)
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            ModDuration::InSeconds(s) => *s,
            ModDuration::kModDuration_Instant => -1.0,
            ModDuration::kModDuration_UntilKilled => ATTRIBMOD_DURATION_FOREVER,
            ModDuration::kModDuration_UntilShutOff => ATTRIBMOD_DURATION_FOREVER,
        }
    }

    pub fn get_string(&self) -> &'static str {
        match self {
            ModDuration::InSeconds(_) => "InSeconds",
            ModDuration::kModDuration_Instant => "Instant",
            ModDuration::kModDuration_UntilKilled => "UntilKilled",
            ModDuration::kModDuration_UntilShutOff => "UntilShutOff",
        }
    }
}

impl ModDuration {
    pub fn new(in_seconds: f32) -> Self {
        ModDuration::InSeconds(in_seconds)
    }
}

impl Default for ModDuration {
    fn default() -> Self {
        ModDuration::InSeconds(0.0)
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum CasterStackType {
    /// Stacking is handled for each caster individually.
    kCasterStackType_Individual,
    /// Stacking is handled for all casters collectively.
    kCasterStackType_Collective,
}
default_val!(CasterStackType, kCasterStackType_Individual);

/// Determines how multiple identical `AttribMod`s from the same power and caster are handled.
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum StackType {
    /// Stack up (allow multiples).
    kStackType_Stack,
    /// Ignore the new duplicate (do nothing).
    kStackType_Ignore,
    /// Update the parameters in and extend the existing `AttribMod`.
    kStackType_Extend,
    /// Update the parameters and replace the existing `AttribMod`.
    kStackType_Replace,
    /// Update the parameters in the existing `AttribMod`, don't extend the duration.
    kStackType_Overlap,
    /// Stack up to StackCount times (if count < StackCount, then stack, else ignore).
    kStackType_StackThenIgnore,
    /// Update the duration in all copies of the existing matching `AttribMod`s, then add a new copy on.
    kStackType_Refresh,
    /// If count < StackCount, then Refresh and add a new copy, else just Refresh.
    kStackType_RefreshToCount,
    /// If mag is greater Replace, else Ignore.
    kStackType_Maximize,
    /// Keep all, but suppress all but the greatest magnitude.
    kStackType_Suppress,
}
default_val!(StackType, kStackType_Replace);

impl StackType {
    pub fn get_string(&self) -> &'static str {
        match self {
            StackType::kStackType_Stack => "Stack",
            StackType::kStackType_Ignore => "Ignore",
            StackType::kStackType_Extend => "Extend",
            StackType::kStackType_Replace => "Replace",
            StackType::kStackType_Overlap => "Overlap",
            StackType::kStackType_StackThenIgnore => "StackToLimit",
            StackType::kStackType_Refresh => "Refresh",
            StackType::kStackType_RefreshToCount => "RefreshToLimit",
            StackType::kStackType_Maximize => "Maximize",
            StackType::kStackType_Suppress => "Suppress",
        }
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum PowerEvent {
    // Invoke-related events.
    kPowerEvent_Activate,
    kPowerEvent_ActivateAttackClick,
    kPowerEvent_Attacked,
    kPowerEvent_AttackedNoException,
    kPowerEvent_Helped,
    kPowerEvent_Hit,
    kPowerEvent_Miss,
    kPowerEvent_EndActivate,
    // Apply-related events.
    kPowerEvent_AttackedByOther,
    kPowerEvent_AttackedByOtherClick,
    kPowerEvent_HelpedByOther,
    kPowerEvent_HitByOther,
    kPowerEvent_HitByFriend,
    kPowerEvent_HitByFoe,
    kPowerEvent_MissByOther,
    kPowerEvent_MissByFriend,
    kPowerEvent_MissByFoe,
    // Damaged/healed events.
    kPowerEvent_Damaged,
    kPowerEvent_Healed,
    // Staus events.
    kPowerEvent_Stunned,
    kPowerEvent_Immobilized,
    kPowerEvent_Held,
    kPowerEvent_Sleep,
    kPowerEvent_Terrorized,
    kPowerEvent_Confused,
    kPowerEvent_Untouchable,
    kPowerEvent_Intangible,
    kPowerEvent_OnlyAffectsSelf,
    kPowerEvent_AnyStatus,
    // Misc.
    kPowerEvent_Knocked,
    kPowerEvent_Defeated,
    kPowerEvent_MissionObjectClick,
    kPowerEvent_Moved,
    kPowerEvent_Defiant,
}
default_val!(PowerEvent, kPowerEvent_Activate);

#[derive(Debug, Default)]
pub struct AttribModParam_Costume {
    pub pch_costume_name: Option<String>,
    pub i_priority: i32,
}
default_new!(AttribModParam_Costume);

#[derive(Debug, Default)]
pub struct AttribModParam_Reward {
    pub ppch_rewards: Vec<String>,
}
default_new!(AttribModParam_Reward);

#[derive(Debug, Default)]
pub struct AttribModParam_EntCreate {
    pub pch_entity_def: Option<NameKey>,
    pub pch_class: Option<String>,
    pub pch_costume_name: Option<String>,
    pub pch_display_name: Option<String>,
    pub pch_priority_list: Option<String>,
    pub pch_ai_config: Option<String>,
    // flattened from PowerSpec
    pub ppch_category_names: Vec<NameKey>,
    pub ppch_powerset_names: Vec<NameKey>,
    pub ppch_power_names: Vec<NameKey>,
    /// added i26p5
    pub redirects: Vec<NameKey>,
    /// reference to full Villain Def (not inline)
    pub villain_def: Option<Rc<VillainDef>>,
    /// reference to entity's powers (not inline)
    pub power_refs: Vec<NameKey>,
    /// have we resolved this already? (not inline)
    pub resolved: bool,
}
default_new!(AttribModParam_EntCreate);

#[derive(Debug, Default)]
pub struct AttribModParam_Power {
    pub i_count: i32,
    // flattened from PowerSpec
    pub ppch_category_names: Vec<NameKey>,
    pub ppch_powerset_names: Vec<NameKey>,
    pub ppch_power_names: Vec<NameKey>,
    /// have we resolved this already? not inline
    pub resolved: bool,
}
default_new!(AttribModParam_Power);

#[derive(Debug, Default)]
pub struct AttribModParam_Phase {
    pub pi_combat_phases: Vec<i32>,
    pub pi_vision_phases: Vec<i32>,
    pub i_exclusive_vision_phase: i32,
}
default_new!(AttribModParam_Phase);

#[derive(Debug, Default)]
pub struct AttribModParam_Teleport {
    pub pch_destination: Option<String>,
}
default_new!(AttribModParam_Teleport);

#[derive(Debug, Default)]
pub struct AttribModParam_Behavior {
    pub ppch_behaviors: Vec<String>,
}
default_new!(AttribModParam_Behavior);

#[derive(Debug, Default)]
pub struct AttribModParam_SZEValue {
    pub ppch_script_id: Vec<String>,
    pub ppch_script_value: Vec<String>,
}
default_new!(AttribModParam_SZEValue);

#[derive(Debug, Default)]
pub struct AttribModParam_Token {
    pub ppch_tokens: Vec<String>,
}
default_new!(AttribModParam_Token);

#[derive(Debug, Default)]
pub struct AttribModParam_EffectFilter {
    pub ppch_tags: Vec<String>,
    // flattened from PowerSpec
    pub ppch_category_names: Vec<String>,
    pub ppch_powerset_names: Vec<String>,
    pub ppch_power_names: Vec<String>,
}
default_new!(AttribModParam_EffectFilter);

/// Added i26p5. Chain related?
#[derive(Debug, Default)]
pub struct AttribModParam_Param11 {
    pub i_unknown_1: i32,
    pub i_unknown_2: i32,
    pub i_unknown_3: i32,
    pub f_unknown_4: f32,
    pub i_unknown_5: i32,
    pub i_unknown_6: i32,
    pub f_unknown_7: f32,
    pub f_unknown_8: f32,
    pub f_unknown_9: f32,
    pub f_unknown_10: f32,
}
default_new!(AttribModParam_Param11);

#[derive(Debug)]
pub enum AttribModParam {
    Costume(AttribModParam_Costume),
    Reward(AttribModParam_Reward),
    EntCreate(AttribModParam_EntCreate),
    Power(AttribModParam_Power),
    Phase(AttribModParam_Phase),
    Teleport(AttribModParam_Teleport),
    Behavior(AttribModParam_Behavior),
    SZEValue(AttribModParam_SZEValue),
    Token(AttribModParam_Token),
    EffectFilter(AttribModParam_EffectFilter),
    Param11(AttribModParam_Param11),
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum ToggleDroppable {
    kToggleDroppable_Sometimes,
    kToggleDroppable_Always,
    kToggleDroppable_First,
    kToggleDroppable_Last,
    kToggleDroppable_Never,
}
default_val!(ToggleDroppable, kToggleDroppable_Sometimes);

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum ProcAllowed {
    kProcAllowed_All,
    kProcAllowed_None,
    kProcAllowed_PowerOnly,
    kProcAllowed_GlobalOnly,
}
default_val!(ProcAllowed, kProcAllowed_All);

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum AttribType {
    kAttribType_Cur,
    kAttribType_Str,
    kAttribType_Res,
    kAttribType_Max,
    kAttribType_Mod,
    kAttribType_Abs,
    kAttribType_Special,
}
default_val!(AttribType, kAttribType_Cur);

impl AttribType {
    pub fn get_string(&self) -> &'static str {
        match self {
            AttribType::kAttribType_Cur => "CurrentValue",
            AttribType::kAttribType_Str => "Strength",
            AttribType::kAttribType_Res => "Resistance",
            AttribType::kAttribType_Max => "MaxValue",
            AttribType::kAttribType_Mod => "CurrentModifier",
            AttribType::kAttribType_Abs => "AbsoluteValue",
            AttribType::kAttribType_Special => "Special",
        }
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum AttribStyle {
    kAttribStyle_None,
    kAttribStyle_Percent,
    kAttribStyle_Magnitude,
    kAttribStyle_Distance,
    kAttribStyle_PercentMinus100,
    kAttribStyle_PerSecond,
    kAttribStyle_Speed,
    kAttribStyle_ResistanceDuration,
    kAttribStyle_Multiply,
    kAttribStyle_Integer,
    kAttribStyle_EnduranceReduction,
    kAttribStyle_InversePercent,
    kAttribStyle_ResistanceDistance,
}
default_val!(AttribStyle, kAttribStyle_None);

/// Rank of a villain. The "level" here is for conning purposes.
#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum VillainRank {
    VR_NONE,
    /// -1 level
    VR_SMALL,
    /// +0 level
    VR_MINION,
    /// +1 level
    VR_LIEUTENANT,
    /// +1 level
    VR_SNIPER,
    /// +2 level
    VR_BOSS,
    /// +3 level - Elite Boss
    VR_ELITE,
    /// +5 level
    VR_ARCHVILLAIN,
    /// +5 level
    VR_ARCHVILLAIN2,
    /// +100 level
    VR_BIGMONSTER,
    /// +1 level
    VR_PET,
    /// +1 level
    VR_DESTRUCTIBLE,
}
default_val!(VillainRank, VR_NONE);

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Gender {
    GENDER_UNDEFINED,
    GENDER_NEUTER,
    GENDER_MALE,
    GENDER_FEMALE,
}
default_val!(Gender, GENDER_UNDEFINED);
