#![allow(non_upper_case_globals)]

bitflags! {
    #[derive(Default)]
    pub struct EffectGroupFlag: u32 {
        /// If true, this effect group is ignored while on PVP maps.
        const PVEOnly = 1;
        /// If true, this effect group is ignored while on PVE maps.
        const PVPOnly = 1 << 1;
        /// Fallback effect groups are normally ignored. If no non-fallback effect
        /// groups within the same collection are eligible to be applied
        const Fallback = 1 << 2;
        /// If true, ensure that the Chance is always evaluated consistently for all LinkedChance effect groups from the same power activation.
        /// DEPRECATED: Will be removed in a future version.
        const LinkedChance = 1 << 3;
    }
}

/// Used below to map values of attrib mod flags back to their human-readable names.
const EFFECT_GROUP_FLAGS_TO_STRINGS: &'static [(EffectGroupFlag, &'static str)] = &[
    (EffectGroupFlag::PVEOnly, "PVEOnly"),
    (EffectGroupFlag::PVPOnly, "PVPOnly"),
    (EffectGroupFlag::Fallback, "Fallback"),
    (EffectGroupFlag::LinkedChance, "LinkedChance"),
];

impl EffectGroupFlag {
    /// Converts an `EffectGroupFlag` value to human-readable strings for each bit.
    ///
    /// # Returns
    /// A `Vec<String>` containing zero or more values based on the current `AttribModFlag`.
    pub fn get_strings(&self) -> Vec<&'static str> {
        let mut strings = Vec::new();
        for (a, s) in EFFECT_GROUP_FLAGS_TO_STRINGS {
            if self.contains(*a) {
                strings.push(*s);
            }
        }
        strings
    }
}

bitflags! {
    #[derive(Default)]
    pub struct AttribModFlag: u64 {
        /// If set, hides floaters (damage and healing numbers, for example) over the affected's head.
        /// If specified, `pch_display_float` is always shown, even if this is set.
        const NoFloaters = 1;
        /// Determines whether this attribmod ignores diminishing returns on boosts
        /// (aka Enhancement Diversification). Only used if aspect is Strength.
        const BoostIgnoreDiminishing = 1 << 1;
        /// If set and the test governed by `f_tick_chance` fails, the attrib mod will be removed.
        const CancelOnMiss = 1 << 2;
        /// If set, only applies the attrib modifier if the target is on the ground.
        const NearGround = 1 << 3;
        /// If set, the attacker's strength is not used to modify the scale of the effects.
        const IgnoreStrength = 1 << 4;
        /// If set, the target's resistance is not used to modify the scale of the applied effects.
        const IgnoreResistance = 1 << 5;
        /// If set, the level difference between the source and the target
        /// is not used to modify the effect's magnitude and/or duration.
        const IgnoreCombatMods = 1 << 6;
        /// If set, forces resistance to apply to something other than the default (based on the
        /// attrib mod type).
        const ResistMagnitude = 1 << 7;
        /// If set, forces resistance to apply to something other than the default (based on the
        /// attrib mod type).
        const ResistDuration = 1 << 8;
        /// If set, forces combat mod to apply to something other than the default (based on the
        /// attrib mod type).
        const CombatModMagnitude = 1 << 9;
        /// If set, forces combat mod to apply to something other than the default (based on the
        /// attrib mod type).
        const CombatModDuration = 1 << 10;
        /// If true, this is supposed to be a boost template.  This is used
        /// in boosts and powers used as set bonuses, which can include both
        /// boost templates and additional normal templates.
        const Boost = 1 << 11;
        /// If true and `pch_display_float`, `pch_display_attacker_hit` or `pch_display_victim_hit`
        /// is specified, it will only display if the attribute evaluates to a non-zero value.
        const HideZero = 1 << 12;
        /// If true, do not clean out this attrib mod when the entity dies.
        const KeepThroughDeath = 1 << 13;
        /// If true, delay any evaluations associated with this attrib mod until the last possible moment in mod_process.
        /// This means you will have to store all the Eval stashes until that time comes. Note that this can cause
        /// desynchronization with other members of the same effect group.
        const DelayEval = 1 << 14;
        /// Do not add the FramesBeforeHit delay from the power to this attrib mod.
        const NoHitDelay = 1 << 15;
        /// Do not add the projectile distance delay from the power to this attribmod.
        const NoProjectileDelay = 1 << 16;
        /// When using StackKey stacking, also compare the Aspect/Attribute in addition to the key.
        const StackByAttribAndKey = 1 << 17;
        /// Apply stacking rules per power instance rather than by the template. Implies individual caster stacking.
        const StackExactPower = 1 << 18;
        /// Designer laziness flag.
        const IgnoreSuppressErrors = 1 << 19;
        /// Valid for: EntCreate
        /// If true, if the pet times out or is otherise destroyed by the server (as opposed to being defeated) then the entity is
        /// vanished as opposed to going through the usual DieNow code. (Only for powers which spawn entities.)
        const VanishEntOnTimeout = 1 << 32;
        /// Valid for: CombatModShift
        /// Causes this mod shift not to be added to the total reported to the client.
        const DoNotDisplayShift = 1 << 33;
        /// Valid for: TokenAdd, TokenSet
        /// Don't update the token timer.
        const NoTokenTime = 1 << 34;
        /// Valid for: RevokePower
        /// Revokes all copies of the power, ignoring Count.
        const RevokeAll = 1 << 35;
        /// Valid for: EntCreate
        /// If true, do not apply custom tinting to the spawned pet's costume.
        const DoNotTintCostume = 1 << 36;
        /// Valid for: ExecutePower, EntCreate
        /// Copy enhancements to the resulting power(s) if they are accepted by its allowed types.
        const CopyBoosts = 1 << 37;
        /// Valid for: EntCreate
        /// Copy strength buff mods from the creator of this entity.
        const CopyCreatorMods = 1 << 38;
        /// Valid for: EntCreate
        /// Suppresses FX on mods copied from creator. Only has an effect if CopyCreatorMods is also set.
        const NoCreatorModFX = 1 << 39;
        /// Valid for: EntCreate
        /// Ignores `pch_villain_def` and `pch_class`, creates a generic entity the same class as its creator.
        /// Implies NoCreatorModFX.
        const PseudoPet = 1 << 40;
        /// Valid for: EntCreate
        /// Forces the summoned entity to show up in a player's pet window.
        const PetVisible = 1 << 41;
        /// Valid for: EntCreate
        /// Forces the summoned entity to be commandable like a mastermind pet.
        const PetCommandable = 1 << 42;
    }
}

/// Used below to map values of attrib mod flags back to their human-readable names.
const ATTRIB_MOD_FLAGS_TO_STRINGS: &'static [(AttribModFlag, &'static str)] = &[
    (AttribModFlag::NoFloaters, "NoFloaters"),
    (
        AttribModFlag::BoostIgnoreDiminishing,
        "BoostIgnoreDiminishing",
    ),
    (AttribModFlag::CancelOnMiss, "CancelOnMiss"),
    (AttribModFlag::NearGround, "NearGround"),
    (AttribModFlag::IgnoreStrength, "IgnoreStrength"),
    (AttribModFlag::IgnoreResistance, "IgnoreResistance"),
    (AttribModFlag::IgnoreCombatMods, "IgnoreLevelDifference"),
    (AttribModFlag::ResistMagnitude, "ResistMagnitude"),
    (AttribModFlag::ResistDuration, "ResistDuration"),
    (AttribModFlag::CombatModMagnitude, "CombatModMagnitude"),
    (AttribModFlag::CombatModDuration, "CombatModDuration"),
    (AttribModFlag::Boost, "Boost"),
    (AttribModFlag::HideZero, "HideZero"),
    (AttribModFlag::KeepThroughDeath, "KeepThroughDeath"),
    (AttribModFlag::DelayEval, "DelayEval"),
    (AttribModFlag::NoHitDelay, "NoHitDelay"),
    (AttribModFlag::NoProjectileDelay, "NoProjectileDelay"),
    (AttribModFlag::StackByAttribAndKey, "StackByAttribAndKey"),
    (AttribModFlag::StackExactPower, "StackExactPower"),
    (AttribModFlag::IgnoreSuppressErrors, "IgnoreSupressErrors"),
    (AttribModFlag::VanishEntOnTimeout, "VanishEntOnTimeout"),
    (AttribModFlag::DoNotDisplayShift, "DoNotDisplayShift"),
    (AttribModFlag::NoTokenTime, "NoTokenTime"),
    (AttribModFlag::RevokeAll, "RevokeAll"),
    (AttribModFlag::DoNotTintCostume, "DoNotTintCostume"),
    (AttribModFlag::CopyBoosts, "CopyBoosts"),
    (AttribModFlag::CopyCreatorMods, "CopyCreatorMods"),
    (AttribModFlag::NoCreatorModFX, "NoCreatorModFX"),
    (AttribModFlag::PseudoPet, "PseudoPet"),
    (AttribModFlag::PetVisible, "PetVisible"),
    (AttribModFlag::PetCommandable, "PetCommandable"),
];

impl AttribModFlag {
    /// Converts an `AttribModFlag` value to human-readable strings for each bit.
    ///
    /// # Returns
    /// A `Vec<String>` containing zero or more values based on the current `AttribModFlag`.
    pub fn get_strings(&self) -> Vec<&'static str> {
        let mut strings = Vec::new();
        for (a, s) in ATTRIB_MOD_FLAGS_TO_STRINGS {
            if self.contains(*a) {
                strings.push(*s);
            }
        }
        strings
    }
}

bitflags! {
    #[derive(Default)]
    pub struct VillainExclusion: u32
    {
        /// Allow in all games.
        const VE_NONE = 0;
        /// Allow in _CoH_ only.
        const VE_COH = 1;
        /// Allow in _CoV_ only.
        const VE_COV = 1 << 1;
        /// ???
        const VE_MA = 1 << 2;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct VillainDefFlags: u32 {
        /// Don't count a badge stat for the villain group when defeated.
        const VILLAINDEF_NOGROUPBADGESTAT = 1;
        /// Don't count a badge stat for the villain rank when defeated.
        const VILLAINDEF_NORANKBADGESTAT = 1 << 2;
        /// Don't count a badge stat for the villain name when defeated.
        const VILLAINDEF_NONAMEBADGESTAT = 1 << 3;
        const VILLAINDEF_NOGENERICBADGESTAT = Self::VILLAINDEF_NOGROUPBADGESTAT.bits | Self::VILLAINDEF_NORANKBADGESTAT.bits | Self::VILLAINDEF_NONAMEBADGESTAT.bits;
    }
}