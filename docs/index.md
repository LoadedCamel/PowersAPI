# Powers JSON Data Dictionary

All output is represented as [UTF-8](https://en.wikipedia.org/wiki/UTF-8) encoded [JSON](https://www.json.org/) data. The following documents describe the structure and purpose of each field.

Since JSON is not a strict serialization format, fields may not appear in the exact order presented here and may be omitted if unspecified.

In this dictionary:

* [Data Types](#data-types) - referenced throughout the dictionary
* [Root](#root) - description of `/index.json`
* [Archetypes](archetypes.md) - description of `/archetypes/index.json`
* [Power Categories](powercats.md) - description of `/(power category name)/index.json`
* [Power Sets](powersets.md) - description of  `/(power category name)/(power set name)/index.json`
* [Powers](powers.md) - contained in power sets
* [Effect Groups](effectgroups.md) - contained in powers

## Data Types

The following data types are used in this data dictionary.

| Type | Definition |
| --- | --- |
| string | A sequence of characters, UTF-8 encoded. |
| int | An integer (whole number). |
| float | A floating point number (i.e. includes fractional values). |
| bool | The literal value `true` or `false`. |
| percent | Same as float, but represents a percent chance from `0.0` to `100.0`, used by the game to check if some random effect takes place. |
| time | Same as float, but indicates a duration in seconds. |
| URL | Same as string, but speficially represents a [URL](https://en.wikipedia.org/wiki/URL) that points to some other resource, typically another JSON file with more information. This could be on a remote server (http/https) or a relative pointer to a local file. | 
| key | Same as string, but represents a unique identifier for a particular object. Generally most objects will have a field called `name` that is its own key, but may contain additional fields that reference the keys of other objects. |
| enum | Same as string, but limited to a few specific values that can be relied upon to be consistent. The description will identify the possible values. |
| object | A complex object with additional subfields. The description will contain a link to the definition. |
| array | A sequence of zero or more values. The description will contain a link to the definition if these are complex objects or otherwise describe the contents if they are simple values. |
| expression | A string that is a representation of a complex expression to be evaluated by the game client/server. Describing this field is a bit out of scope as it is endemic to the game's internals, but the data is included for reference. Where possible, typical expressions (such as features like [Scourge](https://paragonwiki.com/wiki/Scourge) or [critical hits](https://paragonwiki.com/wiki/Inherent_Powers#Critical_Hit)) have been represented in other fields for easier processing. |

## Root

This is the root data set that can be found in the topmost `index.json` file.

| Field | Type | Description |
| --- | --- | --- |
| `issue` | string | The [issue](https://paragonwiki.com/wiki/Issues) (game version) of the extracted data, e.g. "i26p5". |
| `source` | string | The source server, e.g. "homecoming". |
| `extract_date` | string | The date/time that the data was extracted, in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. |
| `archetypes` | URL | The location of the [archetypes](archetypes.md) data. |
| `power_categories` | array | An array of [power categories](#power-category), which are the top level groupings of power sets. |

## Power Category

Each power category is an arbitrary grouping of power sets. Some of them represents the selections of power sets available as primary/secondary options for an [archetype](https://paragonwiki.com/wiki/Archetypes), while others cover large categories of power sets such as all of the [incarnate powers](https://paragonwiki.com/wiki/Incarnate_System).

| Field | Type | Description |
| --- | --- | --- |
| `name` | key | The internal name of the power category. |
| `display_name` | string | A human-readable name for the category. Since categories aren't normally exposed to the game UI, it's not guaranteed this will be meaningful. |
| `archetype` | object | If this category is only intended to be used by one archetype, an [archetype summary](#archetype-summary) will be in this field. |
| `url` | URL | The location of the full [power category](powercats.md) data for this category. |

## Archetype Summary

A shorter form of data about archetypes, similar to that found in the [archetypes](archetypes.md) data.

| Field | Type | Description |
| --- | --- | --- |
| `name` | key | The internal name of the archetype. |
| `display_name` | string | A human-readable name for the archetype. |
| `icon` | URL | The archetype UI icon. |
| `primary_or_secondary` | enum | If a power category is tied to a specific archetype, this will indicate whether it contains power sets intended for the primary or secondary selections. <br> `Primary` - Contains primary power sets. <br> `Secondary` - Contains secondary power sets. |