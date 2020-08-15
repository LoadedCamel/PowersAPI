# Powers JSON Data Dictionary

[Return to power sets](powersets.md) <br> [Return to root](index.md)

## Powers

Each [power](https://paragonwiki.com/wiki/Power) represents an ability that can be utilized by a character. Powers are typically part of a [power set](https://paragonwiki.com/wiki/Power_Set) but can also be granted through the [incarnate system](https://paragonwiki.com/wiki/Incarnate_System) and other means.

| Field | Type | Description |
| --- | --- | --- |
| `name` | key | The internal name of the power. |
| `display_name` | string | A human-readable name for the power. |
| `icon` | URL | The power's UI icon. |
| `display_help` | string | A description of the power and its effects. |
| `display_short_help` | string | A short description of the power, typically containing the specific effects in an abbreviated form. |
| `display_info` | object | A [display info](#display-info) object containing a set of human readable information about the power's characteristics. |
| `requires` | expression | This expression must evaluate to true before the character can purchase or activate this power. |
| `attack_types` | arrary | An array of enum values that represent the type of defenses that will be checked on the target as part of this power's to hit roll. <br> `Melee_Def` <br> `Ranged_Def` <br> `AoE_Def` <br> `Smashing_Def` <br> `Lethal_Def` <br> `Energy_Def` <br> `Negative_Energy_Def` <br> `Fire_Def` <br> `Cold_Def` <br> `Psionic_Def` |
| `enhancements_allowed` | array | An array of strings containing human-readable info on what types of enhancements can be slotted into the power. |
| `enhancement_set_categories_allowed` | array | An array of strings containing human-readable info on which categories of enhancement sets can be slotted into the power. |
| `available_at_level` | int | The earliest level that this power can be purchased by the character. |
| `auto_issue` | bool | If `true`, this power will be given to the character for free when they acquire the power set (doesn't take a power pick). |
| `power_type` | enum | What type of power is this? <br> `Click` - A manually activated power. (Side note: Many temporary powers that are granted by other powers default to `Click` even though you can't technically click them.) <br> `Toggle` - A power that can be toggled on and off and pulses an effect periodically. <br> `Auto` - A power that's always on and doesn't have an activation cost. |
| `accuracy` | float | The power's base accuracy. This gets multiplied into the character's accuracy during hit checks. |
| `effect_area` | object | An [effect area](#effect-area) object that describes what the power can target. |
| `target_type_tags` | array | What are valid targets for the primary effect of this power? <br> See [target type tags](#target-type-tags) below. |
| `target_type_secondary_tags` | array | What are valid targets for the secondary effects of this power? <br> See [target type tags](#target-type-tags) below. |
| `display_target_type` | string | A human-readable string that describes `target_type`. |
| `display_target_type_secondary` | string | A human-readable string that describes `target_type_secondary`. |
| `target_auto_hit_tags` | array | What types of targets does this power auto-hit? (i.e. bypasses accuracy checks) <br> See [target type tags](#target-type-tags) below.|
| `display_target_auto_hit` | string | A human-readable string that describes `target_auto_hit_tags`. |
| `requires_line_of_sight` | bool | If `true`, the character must have line of sight to the target when activating the power. |
| `modes_required` | array | If present, these are the "modes" the character must be in to activate this power. Mostly used by the Kheldian's different forms. |
| `modes_disallowed` | array | If present, these are the "modes" the character cannot be in to activate this power. Most often you will see `Disable_All` in this field, which unsurprisingly means when all of the player's powers have been disabled globally. |
| `status_interaction` | object | A [status interaction](#status-interaction) object that describes how this power interacts with different status effects. |
| `activate` | object | An [activation](#activation) object that describes the activation characteristics of this power. |
| `usage` | object | A [usage](#usage) object that describes how much the power can be used before it is removed from the character. Most often used by limited-use temp powers. |
| `effect_groups` | array | An array of [effect groups](effectgroups.md) that describe the specific effects created by this power when it is activated, such as dealing damage, summoning pets, etc. |
| `redirects` | array | An array of [redirects](#redirects) that point to other powers. If present, evaluate these to take the place of this power when activated. |

## Display Info

This is a set of convenience fields for a power that mimics how global aspects of the power display in the information window in the game UI. Information on the specific effects are found in the `scaled_effects` array.

All of these fields are intended to be human-readable representations but should not be relied on for matching and may change in future revisions. The field names are the labels and are also subject to change. The below table is provided as an example.

| Field | Type | Description |
| --- | --- | --- |
| Target Type | string | What the power can target. |
| Activation Time | string | How long it takes to activate the power. |
| Recharge Time | string | How long it takes before the power recharges. | Effect Area | string | The power's area of effect, such as "Single Target" or "Cone". |
| Power Range | string | The power's range. |
| Power Type | string | Whether the power is "Click", "Toggle", or "Auto". |
| Endurance Cost | string | How much endurance the power consumes when activated. |
| Attack Types | string | The defenses that are checked on the target when making a to-hit check. |
| Available Level | string | The earliest level at which the character can purchase this power. |
| Accuracy | string | The power's base accuracy. |
| Aggro Type | string | If the power doesn't aggro enemies, it will be noted here. |

## Effect Area

This set of fields describes what kind of targets the power can affect as well as how many and the ranges involved.

| Field | Type | Description |
| --- | --- | --- |
| `area` | enum | What is the "area" of the power's effect? <br> `SingleTarget` - The power only affects the current character's focused target. <br> `Cone` - The power affects all targets in an arc originating from the character. <br> `AoE` - The power affects all targets in a sphere originating from the character, the character's focused target, or a point. <br> `Location` - The power affects a specific point (used mostly by teleports). <br> `Chain` - The power hits the character's focused target and then bounces to additional nearby targets. <br> `Self` - The power only affects the character that uses it. |
| `max_targets_hit` | int | If the power can affect more than one target, this is the maximum number of targets. |
| `max_targets_expression` | expression | This is an expression evaluated to determine the actual maximum number of targets. If this is present, `max_targets_hit` represents an absolute maximum, but the result of this expression could be lower. |
| `radius_feet` | float | The distance (in feet) from the point of origin that a `Cone` or `AoE` power will hit targets. |
| `jump_distance_feet` | float | The maximum distance (in feet) from the previous target that a `Chain` power will jump to the next target. |
| `arc_degrees` | float | The arc in degrees of a `Cone` power's target area. |
| `chain_delay_time` | time | The time in seconds before a `Chain` power jumps from one target to the next. |
| `range_feet` | float | The distance (in feet) that the power can reach from the player. Note for `AoE` and `Location` powers, if this is non-zero it represents the farthest distance the point of origin can be placed. |
| `range_feet_secondary` | float | Same as `range_feet` for the power's secondary effect. An example of where this is used is a power like [Recall Friend](https://paragonwiki.com/wiki/Teleportation#Recall_Friend) - the primary range represents the distance to the person you're trying to teleport and the secondary range is where you can place the point to teleport them to. |

## Status Interaction

This object describes how a power interacts with various status effects.

| Field | Type | Description |
| --- | --- | --- |
| `cast_through` | array | An array of enum values describing which status effects do not prevent the character from activating this power. <br> `Hold` <br> `Sleep` <br> `Stun` <br> `Terrorize` |
| `toggle_ignores` | array | An array of enum values describing which status effects will not toggle off this power. <br> `Hold` <br> `Sleep` <br> `Stun` |

## Activation

This object describes how and when a power is activated.

| Field | Type | Description |
| --- | --- | --- |
| `cast_time` | time | The time in seconds it takes to activate the power. |
| `animation_time` | time | The time in seconds it takes for the power's animation to complete. |
| `animation_time_before_hit` | time | The time in seconds it takes for the power to hit/apply to the target (attack animation). |
| `recharge_time` | time | After activating, this is the time in seconds before the power can be activated again. |
| `interrupt_time` | time | This is the window in seconds during `cast_time` in which the power can be interrupted. |
| `auto_cast_interval` | time | If present, this is the time in seconds that the power will automatically be re-activated (used by toggles to provide a continuous effect). |
| `endurance_cost` | float | When activated, the power will deduct this much endurance from the character's endurance pool. The power cannot be activated if they do not have sufficient endurance. |

## Usage

This object describes how much a power can be used before it is deactivated or removed from the character.

| Field | Type | Description |
| --- | --- | --- |
| `remove_on_limit` | bool | If `true`, the power is removed from the character when limits are reached. |
| `extend_on_additional_grant` | bool | If `true`, if the character receives another copy of this power, it's usage time will be extended rather than replaced. |
| `charges` | int | The initial number of activations the power has. |
| `max_charges_on_extend` | int | If extended, the maximum number of activations the power is allowed to have. |
| `toggle_usage_time` | time | The time in seconds the power can be toggled on before it automatically toggles off. |
| `toggle_max_usage_time_on_extend` | time | If extended, the maximum amount of time the power can be toggled on. |
| `lifetime` | time | The absolute time in seconds the character can own the power before it is removed. |
| `max_lifetime_on_extend` | time | If extended, the maximum amount of time the character can own the power. |
| `in_game_lifetime` | time | The time in seconds while logged in the character can own the power before it is removed. |
| `max_in_game_lifetime_on_extend` | time | If extended, the maximum amount of time in-game the character can own the power. |

## Target Type Tags

The target type tags are entirely a construction of this data set to try and make sense of the target types expressed by the game. Internally, there are almost 40 different values for this one field that represent different possible sets of targets. I broke this down into a system of tags that should be simpler to understand.

Where these are present, there will be an array of zero or more enum values that describe possible valid targets. For a target to be valid, it must match at least one of the tags in each of three groupings. If a tag from a group isn't present, then it isn't considered for matching.

### Group 1

| Enum Value | Description |
| --- | --- |
| `Self` | The character that activated the power. |
| `Player` | Any player. |
| `NPC` | Any non-player character. |
| `Pet` | Any minion owned by another character (Mastermind summons, lore pets, etc.). |
| `Owner` | If the character that activated the power has an owner, it must target that owner. |
| `Root_Owner` | Same as `Owner`, but it follows the chain all the way to the top (i.e. owner of a pet of a pet). |

### Group 2

| Enum Value | Description |
| --- | --- |
| `Friend` | Any friendly target. |
| `Foe` | Any enemy target. |
| `Owned` | The target must be "owned" (typically a pet or pseudopet) by the character activating the power. |

### Group 3

| Enum Value | Description |
| --- | --- |
| `Alive` | A target that is currently alive. |
| `Dead` | A target that is currently dead. |

### Miscellaneous

These remaining tags aren't considered part of a group and expand the scope of potential matches.

| Enum Value | Description |
| --- | --- |
| `Team` | The power can target teammates. |
| `League` | The power can target leaguemates. |
| `Location` | The power targets a specific location chosen by the character that activated it. |
| `Teleport` | The power is a teleport (special handling). |
| `Position` | The power is position-based (relative to the character that activated the power). |

## Redirects

Some powers don't have their own effects, instead utilizing a redirected power to take its place when activated. On activation, the requires expressions are evaluated, looking for the first suitable power to redirect to. If none are suitable, the fallback is used.

| Field | Type | Description |
| --- | --- | --- |
| `name` | key | The internal name of the power to redirect to. |
| `fallback` | bool | If `true`, this is the fallback power to use if no other suitable power is found in the set (i.e. all `requires` expressions evaluate to `false`). |
| `requires` | expression | This expression must evaluate to `true` for the power named by this redirect to take the place of the parent power. |
| `url` | url | A URL pointing to the [power set](powersets.md) where the power referenced by `name` can be found. |
