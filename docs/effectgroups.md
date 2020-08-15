# Powers JSON Data Dictionary

[Return to powers](powers.md) <br> [Return to root](index.md)

## Effect Groups

Each [power](https://paragonwiki.com/wiki/Power) has a set of effect groups that describe what the power actually does when a character activates it.

| Field | Type | Description |
| --- | --- | --- |
| `pve_or_pvp` | enum | Some effect groups only apply when used in either PVE or PVP. If this is present, this will indicate which one. If not present, the effect group is active in both PVE and PVP. <br> `PVE` <br> `PVP` |
| `tags` | array | This is an array of enums that describe different aspects of the effect group. See below for descriptions: <br> `FieryEmbrace` - This effect group only applies if [Fiery Embrace](https://paragonwiki.com/wiki/Fiery_Aura#Fiery_Embrace) is active. <br> `Critical` - This effect group represents extra damage from a critical hit. <br> `Domination` - This effect group only applies if a Dominator's Domination inherent is active. <br> `Scourge` - This effect group represents extra damage from a Corrupter's Scourge inherent. <br> `Containment` - This effect group represents extra damage from a Controller's Containment inherent. <br> `DualPistolsLethalMode`, `DualPistolsFireMode`, `DualPistolsColdMode`, `DualPistolsToxicMode` - This effect group only applies if the corresponding [Swap Ammo toggle](https://paragonwiki.com/wiki/Dual_Pistols#Swap_Ammo) is active. |
| `visible_in_info_window` | bool | If `true`, this effect is visible in the power info window in the game UI. |
| `chance_percent` | percent | This represents the chance that the effect group actually activates when the power is activated. |
| `procs_per_minute` | float | Not used by powers. |
| `after_delay_seconds` | time | If present, this is a number of seconds that must pass after the power is activated before this effect group is applied to the target. |
| `requires` | expression | An expression that must be satisfied in order for the effect group to actually take effect. |
| `flags` | array | An array of enums of additional info about the effect group. I think these are outdated and may not actually be used anymore. <br> `PVEOnly` - Effect group is ignored on PVP maps. <br> `PVPOnly` - Effect group is ignored on PVE maps. <br> `Fallback` - Fallback effect groups are usually ignored. (Not sure when they're not?) <br> `LinkedChance` - Deprecated. |
| `effects` | array | An array of [effects](#effects) that describe the specific effects applied by this effect group. |
| `child_effect_groups` | array | Effect groups can have additional effect groups that trigger when activated. |

## Effects

Effects are individual effects applied to a target of a power when the containing effect group is applied. These are also referred to as "attribute modifiers" or "attribmods" internally.

| Field | Type | Description |
| --- | --- | --- |
| `attributes` | array | If the effect applies modifications to specific character attributes on the target, they will be listed here. See [attributes](#attributes) for values. |
| `applies_to` | enum | Attribute modifiers can be applied in a few different ways. Describing how these are used is a bit arcane because the game logic is very specific to this value combined with the specific character attribute. <br> `CurrentValue` - The current value of the modifier. <br> `Strength` - The "strength" of the modifier, i.e. a buff above the baseline. <br> `Resistance` - Damage or status resistance (a percentage). <br> `CurrentModifier` - The current modifier, such as status effects applied. <br> `AbsoluteValue` - Applies a simple add/subtract to the current value. <br> `Special` - Meaning depends on the specific attribute being modified. |
| `application_type` | enum | How and when the effect is applied. <br> `Immediate` - Once, immediately. <br> `OnTick` - Continuously while the power is active (re-applied based on the auto-cast interval or `continuous_apply_seconds` if present). <br> `OnActivate` - When the power is turned on. <br> `OnDeactivate` - When the power is turned off. <br> `OnExpire` - When the power is turned off. <br> `OnEnable` - When the power becomes able to be turned on. <br> `OnDisable` - When the power becomes no longer able to be turned on. |
| `tick_chance_percent` | percent | If the `application_type` is `OnTick`, this is the chance that the effect is applied on each tick. If not specified, assumed to be `100.0`. |
| `magnitude` | float | The strength of a particular effect, if the scaling is based on duration. Most powers that have a "magnitude" as described in game terms actually have the magnitude in the scaled effect. |
| `magnitude_expression` | expression | If this is present, this must be evaluated to calculate the value of `magnitude`. |
| `duration` | enum | A general description of the effect's duration. <br> `InSeconds` - Check `duration_seconds` or `duration_expression` for the explicit duration. <br> `UntilKilled` - The effect will last until the target or the caster are killed. <br> `UntilShutOff` - The effect will last until the power is deactivated. |
| `duration_seconds` | time | If `duration` is `InSeconds`, this is the time in seconds that the effect will last. |
| `duration_expression` | expression | If this is present, this must be evaluated to calculate the value of `duration_seconds`. |
| `after_delay_seconds` | time | If present, this is a number of seconds that must pass after the effect group is activated before this effect is applied to the target. |
| `continuous_apply_seconds` | time | If present, the effect is applied again every time this amount of time has passed. |
| `ticks` | int | If present, the number of "ticks" applied over time. This is a funciton of `duration_seconds` divided by `continuous_apply_seconds` and is provided for convenience. |
| `flags` | array | A set of enum values that describe miscellaneous aspects of this effect. See [effect flags](#effect-flags) below. |
| `parameter` | object | Some effects can have an additional [parameter](#parameters) that provides additional information about the effect. |
| `stacking` | object | If the effect can stack, this object will be present. See [stacking](#stacking) below. |
| `scaled` | array | An effect generated by different archetypes will have different specific values for damage, resistance, etc. This array will have one [scaled effect](#scaled-effects) object per archetype that can use the power. <br> **Note:** Where possible, I've tried to narrow this group down to ATs that can actually use the power. In some cases, however, that wasn't possible to determine programmatically, and you'll see data for every AT even if it's not available to some of them. |

### Attributes

These are the possible attributes that can be modified by an effect.

* `Smashing_Dmg`, `Lethal_Dmg`, `Fire_Dmg`, `Cold_Dmg`, `Energy_Dmg`, `Negative_Energy_Dmg`, `Psionic_Dmg`, `Toxic_Dmg`
* `Melee_Def`, `Ranged_Def`, `Area_Def`, `Smashing_Def`, `Lethal_Def`, `Fire_Def`, `Cold_Def`, `Energy_Def`, `Negative_Energy_Def`, `Psionic_Def`, `Defense`
* `Melee_Elusivity`, `Ranged_Elusivity`, `Area_Elusivity`, `Smashing_Elusivity`, `Lethal_Elusivity`, `Fire_Elusivity`, `Cold_Elusivity`, `Energy_Elusivity`, `Negative_Energy_Elusivity`, `Psionic_Elusivity`, `ElusivityBase`
* `HitPoints`, `Absorb`
* `Endurance`
* `Insight` (deprecated)
* `Rage`
* `ToHit`, `Accuracy`
* `RunningSpeed`, `FlyingSpeed`, `SwimmingSpeed`, `JumpingSpeed`, `JumpHeight`
* `MovementControl`, `MovementFriction`
* `Stealth` (not what it sounds like, at some point it was co-opted into Domination)
* `StealthRadius_PVE`, `StealthRadius_PVP`, `PerceptionRadius`
* `Regeneration`, `Recovery`, `InsightRecovery`
* `ThreatLevel`, `Taunt`, `Placate`
* `Confused`, `Afraid`, `Terrorized`, `Held`, `Immobilized`, `Stunned`, `Sleep`
* `Fly`, `Jump Pack`, `Teleport`
* `Untouchable`, `Intangible`, `OnlyAffectsSelf`
* `ExperienceGain`, `InfluenceGain`, `PrestigeGain`
* `Evade` (deprecated)
* `Knockup`, `Knockback`, `Repel`
* `Radius`, `Arc`, `Range`
* `TimeToActivate`, `RechargeTime`, `InterruptTime`
* `EnduranceDiscount`
* `InsightDiscount` (deprecated)
* `Meter`

### Effect Flags

| Value | Description |
| --- | --- |
| `NoFloaters` | Suppresses floating damage/healing numbers (but not explicit float text). |
| `BoostIgnoreDiminishing` | Ignores diminishing returns from enhancements (a.k.a. enhancement diversification). Only used if `applies_to` is `Strength`. |
| `CancelOnMiss` | If the check against `tick_chance_percent` fails, the effect is ended. |
| `NearGround` | The target must be on the ground for this effect to be applied. |
| `IgnoreStrength` | The effect does not scale based on the attacker's strength. |
| `IgnoreResitance` | The effect does not scale based on the target's resistances. |
| `IgnoreLevelDifference` | The effect does not scale based on level difference between attacker and target. |
| `ResistMagnitude` | The effect's resistance applies to magnitude rather than the default for the attribute modifier. |
| `ResistDuration`  | The effect's resistance applies to duration rather than the default for the attribute modifier. |
| `CombatModMagnitude` | The effect applies to magnitude rather than the default for the attribute modifier. |
| `CombatModDuration` | The effect applies to duration rather than the default for the attribute modifier. |
| `Boost` | This is an enhancement template or a set bonus granted by enhancements (not used by powers). |
| `HideZero` | The float text will only display if the result of applying this effect is a non-zero adjustment. |
| `KeepThroughDeath` | The effect will not be removed when the target dies. |
| `DelayEval` | Delays evaluation of any expressions until the last possible moment before the effect is applied. |
| `NoHitDelay` | Used by the animation / visual effects. |
| `NoProjectileDelay` | Used by the animation / visual effects. |
| `StackByAttribAndKey` | When making comparisons for stacking, also take the specific attribute into account as well as the key. |
| `StackExactPower` | When stacking, compare the specific power rather than the template. Implies stacking by individual caster. |
| `IgnoreSuppressErrors` | Used for testing. |
| `VanishEntOnTimeout` | If a summoned entity times out, it vanishes rather than dying. |
| `DoNotDisplayShift` | Causes a level shift to not be reported by the game client. |
| `NoTokenTime` | Don't update the token timer when a token is added or removed from the target. |
| `RevokeAll` | Revokes all copies of the power from the target, regardless of how many they have. |
| `DoNotTintCostume` | Do not apply power tinting to a pet's costume. |
| `CopyBoosts` | Copy enhancements to a created power or pet if they apply. |
| `CopyCreatorMods` | Copy the owner's strength buff values to a pet. |
| `NoCreatorModFX` | If `CopyCreatorMods` is used, this will suppress copying the visual FX of the copied buffs. |
| `PseudoPet` | Creates a generic entity the same class as its creator (i.e. doesn't use a specific villain definition). |
| `PetVisible` | Forces a created entity to show up in the owner's pet window. |
| `PetCommandable` | Forces a created entity to be commandable like a mastermind pet. |

## Parameters

An effect can have one of these paramaters attached to it to describe additional information needed to generate the effect. For example, an effect that summons a pet will have an `EntCreate` parameter that specifies what pet to create.

The parameter will be one of the folllowing objects.

### Costume (`costume`)

The effect applies a costume to the target.

| Field | Type | Description |
| --- | --- | --- |
| `costume_name` | string | An internal costume name to apply. |

### Effect (`effect`)

Applies a visual effect to the target.

| Field | Type | Description |
| --- | --- | --- |
| `tags` | array | An array of visual FX tags.

### CreateEntity (`create_entity`)

Creates an entity ("pet"). This could be a true pet such as a mastermind summon, or it could be a pseudo-pet used to generate effect, like Archery's Rain of Arrows.

| Field | Type | Description |
| --- | --- | --- |
| `name` | key | The unique name of the villain definition for the entity. |
| `display_name` | string | The name of the entity as it's displayed in the game client. |
| `powers` | array | An array of objects that point to the granted powers. See [power reference](#power-reference) below. |
| `power_refs` | array | If for some reason the specific powers can't be loaded, this will contain the names of the powers as specified by the villain definition. |
| `redirects` | array | An arry of objects that point to _redirected_ powers. These replace the effect of summoning a pet if present. See [power reference](#power-reference) below. |

### Phase (`phase`)

Adjusts the "phase" of the target, which modifies what the target can see and/or interact with. I'm not 100% sure how this is used.

| Field | Type | Description |
| --- | --- | --- |
| `exlcusive_vision_phase` | int | |
| `combat_phases` | array | |
| `vision_phases` | array | |

### Power (`power`)

Grants a power to the target or executes a power.

| Field | Type | Description |
| --- | --- | --- |
| `count` | int | If a granted power that gives more than 1 stack, this will be the number granted. |
| `powers` | array | An array of objects that point to the granted/executed powers. See [power reference](#power-reference) below. |

### Reward (`reward`)

Grants a reward to the target.

| Field | Type | Description |
| --- | --- | --- |
| `rewards` | array | An array of reward names. |

### Teleport (`teleport`)

Causes the target to be teleported to a specific destination.

| Field | Type | Description |
| --- | --- | --- |
| `destination` | string | The name of the location to teleport to. |

### Token (`token`)

Grants a token to the target.

| Field | Type | Description |
| --- | --- | --- |
| `tokens` | array | An array of token names. |

### Behavior (`behavior`)

Applies AI behaviors to the target.

| Field | Type | Description |
| --- | --- | --- |
| `behaviors` | array | An array of behavior names. |

### ScriptValue (`script_value`)

Adjusts the values of script variables.

| Field | Type | Description |
| --- | --- | --- |
| `values` | array | An array of key/value pairs to adjust. |

## Power Reference

References a power elsewhere in the data set.

| Field | Type | Destruction |
| --- | --- | --- |
| `name` | key | The internal name of the power. |
| `url` | url | A URL pointing to the [power set](powersets.md) where the referenced power can be found. |

## Stacking

Describes how the effect stacks, if at all.

| Field | Type | Description |
| --- | --- | --- |
| `behavior` | enum | The specific stacking behavior. <br> `Stack` - Allow multiple. <br> `Extend` - Update the parameters and extend the existing effect. <br> `Replace` - Update the parameters and replace the existing effect. <br> `Overlap` - Update the parameters, but don't extend the existing effect. <br> `StackToLimit` - Allow multiple up to `limit` times (see below). <br> `Refresh` - Update the duration of all similar effects, then add a new copy. <br> `RefreshToLimit` - As `Refresh`, but if below `limit` (see below), also add a new copy. <br> `Maximize` - If the new effect has a greater magnitude, replace the current effect.  <br> `Suppress` - Keep all copies, but only apply the highest magnitude. |
| `by_caster` | bool | If `true`, then each caster can apply their own stacks to a target. Otherwise, effects placed by any caster are treated as the same for stacking. |
| `limit` | int | If `behavior` is `StackToLimit` or `RefreshToLimit`, this is the maximum number of times the effect will stack. |
| `key` | string | If this value is present, then stacking happens based on effects with this same `key` value, rather than the specific effect. |

## Scaled Effects

Describes the specific scaled values for an effect based on a particular archetype that generates that effect.

| Field | Type | Description |
| --- | --- | --- |
| `archetype` | string | The display name of the [archetype](archetypes.md) for which this scaled effect applies. |
| `average` | float | For `damage` / `healing`, this is a convenience field with the average amount applied (averages based on number of ticks and tick chance). |
| `per_activation` | float | For `damage` / `healing`, this is a convenience field with the average amount per activation. This is `average` divided by the total cast time (activation, wind up, follow through). |
| `per_cast_cycle` | float | For `damage` / `healing`, this is a convenience field with the average amount per cast cycle. This is `average` divided by the total cast time and recharge time. |
| `display_info` | array | This is an array of strings that attempts to replicate how the effect is described in the power info window in the game client. |
| `base_value` | float | The base value of the effect, provided for reference. |
| `scale` | float | The scale applied to `base_value`, provided for reference. |

**There will only be one of the following fields present.**

The final value of the below fields is a function of `base_value` mulitplied by `scale`.

| Field | Type | Description |
| --- | --- | --- |
| `damage` | float | An amount of damage applied to the target's hit points (subtracted). |
| `healing` | float | An amount of healing applied to the target's hit points (added). |
| `percent` | float | A scaled percentage (`0.0` to `100.0`) to apply to the attribute (multiplied). |
| `duration_seconds` | time | The duration of the effect in seconds (absolute). |
| `magnitude` | float | A magnitude of a status effect to be applied to the attribute (added/subtracted). |
| `distance` | float | A distance in feet to be applied to the attribute (added/subtracted). |
| `value` | float | A generic value for effects that aren't one of the above (added/subtracted). |