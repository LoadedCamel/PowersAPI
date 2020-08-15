# Powers JSON Data Dictionary

[Return to root](index.md)

## Power Sets

Each [power set](https://paragonwiki.com/wiki/Power_Set) has some top-level data about the power set as well as a collection of individual [powers](https://paragonwiki.com/wiki/Power) that make up the set. Once a character has access to a power set (either granted automatically by [archetype](https://paragonwiki.com/wiki/Archetypes) or through choice such as with [power pools](https://paragonwiki.com/wiki/Power_Pools)), they can choose from the individual powers in the set as they levevl up.

| Field | Type | Description |
| --- | --- | --- |
| `issue` | string | The [issue](https://paragonwiki.com/wiki/Issues) (game version) of the extracted data, e.g. "i26p5". |
| `source` | string | The source server, e.g. "homecoming". |
| `extract_date` | string | The date/time that the data was extracted, in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. |
| `name` | key | The internal name of the power set. |
| `display_name` | string | A human-readable name for the power set. |
| `icon` | URL | The power set's UI icon. Note that power sets do not have unique icons; the icon is pulled from the first power in the set. |
| `specialize_at_level` | int | Some power sets can't be picked until the character reaches a certain level. If present, this the level the character can start picking powers from this set. |
| `specialize_requires` | expression | An expression that must evaluate to true for the player to have access to this power set when reaching `specialize_at_level`. |
| `show_in_inventory` | enum | How the power set is displayed in the character's "inventory" (character creation and level up). <br> `Always`, `Show` - Always shown. <br> `IfOwned` - If the character already owns a power from the set. <br> `IfUsable` - If the character can use one of the powers from the set. <br> `Never` - Always hidden. |
| `show_in_power_management` | bool | If true, the power set will show in the enhancement management screen. |
| `show_in_power_info` | bool | If true, the power set will show in the powers tab of the player info dialog. |
| `ordered_power_names` | array | An array of keys to the individual powers in the power set. This array is guaranteed to be sorted in the same way as the game's UI. (`powers` below will be in arbitrary order.) |
| `powers` | array | An array of [powers](powers.md) that are available in the power set. |