# Powers JSON Data Dictionary

[Return to root](index.md)

## Archetypes

This data set describes all of the possible [archetype](https://paragonwiki.com/wiki/Archetypes) selections that are referenced elsewhere in the powers data.

| Field | Type | Description |
| --- | --- | --- |
| `issue` | string | The [issue](https://paragonwiki.com/wiki/Issues) (game version) of the extracted data, e.g. "i26p5". |
| `source` | string | The source server, e.g. "homecoming". |
| `extract_date` | string | The date/time that the data was extracted, in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format. |
| `archetypes` | array | An array of [archetype objects](#archetype-object) |

## Archetype Object

Describes an individual archetype.

| Field | Type | Description |
| --- | --- | --- |
| `name` | key | The internal name of the archetype. |
| `display_name` | string | A human-readable name for the archetype. |
| `icon` | URL | The archetype UI icon. |
| `display_help` | string | A description of the archetype's role. |
| `display_short_help` | string | A shortened description of the archetype's role. |
| `allowed_origins` | array | An array of enum values that describe what origins are allowed for this archetype. <br> `Science` <br> `Magic` <br> `Natural` <br> `Mutation` <br> `Technology` |
| `restrictions` | array | An array of enum values that describe any restrictions for taking this archetype (historical, these aren't used since [Going Rogue](https://paragonwiki.com/wiki/Going_Rogue)). <br> `ArachnosSolider` - Must unlock villain epic archetypes <br> `Hero` -  City of Heroes only <br> `Kheldian` - Must unlock hero epic archetypes <br> `Villain` - City of Villains only |
| `level_up_respecs` | array | An array of ints indicating at which levels the character must [respec](https://paragonwiki.com/wiki/Power_Respecification). |
| `primary_category` | key | The name of the [power category](powercats.md) that contains the archetype's primary power sets. |
| `secondary_category` | key | The name of the [power category](powercats.md) that contains the archetype's secondary power sets. |