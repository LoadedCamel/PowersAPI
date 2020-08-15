# Powers JSON Data Dictionary

[Return to root](index.md)

## Power Categories

This data set describes all of the possible power categories that group known [power sets](https://paragonwiki.com/wiki/Power_Set).

| Field | Type | Description |
| --- | --- | --- |
| `issue` | string | The [issue](https://paragonwiki.com/wiki/Issues) (game version) of the extracted data, e.g. "i26p5". |
| `source` | string | The source server, e.g. "homecoming". |
| `extract_date` | string | The date/time that the data was extracted, in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. |
| `name` | key | The internal name of the power category. |
| `archetype` | object | If this category is only intended to be used by one archetype, an [archetype summary](index.md#archetype-summary) will be in this field. |
| `power_sets` | array | An array of [power sets](#power-set) contained in this category. |

## Power Set

Each power set is a group of [powers](https://paragonwiki.com/wiki/Power) that a character can choose from as they level up.

| Field | Type | Description |
| --- | --- | --- |
| `name` | key | The internal name of the power set. |
| `display_name` | string | A human-readable name for the power set. |
| `url` | URL | The location of the full [power set](powersets.md) data for this set. |