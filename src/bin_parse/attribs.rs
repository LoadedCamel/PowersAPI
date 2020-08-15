use super::*;
use crate::structs::{AttribName, AttribNames, CharacterAttributes};

/// Reads all of the attribute names in the current .bin file.
/// Refer to Common/entity/attrib_names.h TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for archetypes
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, an `AttribNames` struct.
/// Otherwise, a `ParseError` with the error information.
#[rustfmt::skip]
pub fn serialized_read_attribs<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<AttribNames>
where
    T: Read + Seek,
{
    let mut attrib_names = AttribNames::new();

    macro_rules! names_arr {
        ($($field:ident),+) => {
            $( bin_read_arr_fn(&mut attrib_names.$field, |re| read_attrib_name(re, strings, messages), reader)?; )+
        };
    }
    // data length
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;

    names_arr!(
        pp_damage,
        pp_defense,
        pp_boost,
        pp_group,
        pp_mode,
        pp_elusivity,
        pp_stack_key
    );

    // Not technically part of this file but we'll read them from the message store here.
    // If you're wondering where these came from, these are the same messages used by the
    // info box UI (see `Game/UI/uiPowerInfo.c` and `Common/entity/attrib_description.c`).
    for (offset, key) in vec![
        (CharacterAttributes::OFFSET_DMG_0, "AttrDamageType[0]"),
        (CharacterAttributes::OFFSET_DMG_1, "AttrDamageType[1]"),
        (CharacterAttributes::OFFSET_DMG_2, "AttrDamageType[2]"),
        (CharacterAttributes::OFFSET_DMG_3, "AttrDamageType[3]"),
        (CharacterAttributes::OFFSET_DMG_4, "AttrDamageType[4]"),
        (CharacterAttributes::OFFSET_DMG_5, "AttrDamageType[5]"),
        (CharacterAttributes::OFFSET_DMG_6, "AttrDamageType[6]"),
        (CharacterAttributes::OFFSET_DMG_7, "AttrDamageType[7]"),
        (CharacterAttributes::OFFSET_DMG_8, "AttrDamageType[8]"),
        (CharacterAttributes::OFFSET_DMG_9, "AttrDamageType[9]"),
        (CharacterAttributes::OFFSET_DMG_10, "AttrDamageType[10]"),
        (CharacterAttributes::OFFSET_DMG_11, "AttrDamageType[11]"),
        (CharacterAttributes::OFFSET_DMG_12, "AttrDamageType[12]"),
        (CharacterAttributes::OFFSET_DMG_13, "AttrDamageType[13]"),
        (CharacterAttributes::OFFSET_DMG_14, "AttrDamageType[14]"),
        (CharacterAttributes::OFFSET_DMG_15, "AttrDamageType[15]"),
        (CharacterAttributes::OFFSET_DMG_16, "AttrDamageType[16]"),
        (CharacterAttributes::OFFSET_DMG_17, "AttrDamageType[17]"),
        (CharacterAttributes::OFFSET_DMG_18, "AttrDamageType[18]"),
        (CharacterAttributes::OFFSET_DMG_19, "AttrDamageType[19]"),
        (CharacterAttributes::OFFSET_HIT_POINTS, "AttrHitPoints"),
        (CharacterAttributes::OFFSET_ABSORB, "AttrAbsorb"),
        (CharacterAttributes::OFFSET_ENDURANCE, "AttrEndurance"),
        (CharacterAttributes::OFFSET_INSIGHT, "AttrInsight"),
        (CharacterAttributes::OFFSET_RAGE, "AttrRage"),
        (CharacterAttributes::OFFSET_TOHIT, "AttrToHit"),
        (CharacterAttributes::OFFSET_DEF_0, "AttrDefenseType[0]"),
        (CharacterAttributes::OFFSET_DEF_1, "AttrDefenseType[1]"),
        (CharacterAttributes::OFFSET_DEF_2, "AttrDefenseType[2]"),
        (CharacterAttributes::OFFSET_DEF_3, "AttrDefenseType[3]"),
        (CharacterAttributes::OFFSET_DEF_4, "AttrDefenseType[4]"),
        (CharacterAttributes::OFFSET_DEF_5, "AttrDefenseType[5]"),
        (CharacterAttributes::OFFSET_DEF_6, "AttrDefenseType[6]"),
        (CharacterAttributes::OFFSET_DEF_7, "AttrDefenseType[7]"),
        (CharacterAttributes::OFFSET_DEF_8, "AttrDefenseType[8]"),
        (CharacterAttributes::OFFSET_DEF_9, "AttrDefenseType[9]"),
        (CharacterAttributes::OFFSET_DEF_10, "AttrDefenseType[10]"),
        (CharacterAttributes::OFFSET_DEF_11, "AttrDefenseType[11]"),
        (CharacterAttributes::OFFSET_DEF_12, "AttrDefenseType[12]"),
        (CharacterAttributes::OFFSET_DEF_13, "AttrDefenseType[13]"),
        (CharacterAttributes::OFFSET_DEF_14, "AttrDefenseType[14]"),
        (CharacterAttributes::OFFSET_DEF_15, "AttrDefenseType[15]"),
        (CharacterAttributes::OFFSET_DEF_16, "AttrDefenseType[16]"),
        (CharacterAttributes::OFFSET_DEF_17, "AttrDefenseType[17]"),
        (CharacterAttributes::OFFSET_DEF_18, "AttrDefenseType[18]"),
        (CharacterAttributes::OFFSET_DEF_19, "AttrDefenseType[19]"),
        (CharacterAttributes::OFFSET_DEFENSE, "AttrDefense"),
        (CharacterAttributes::OFFSET_RUNNING_SPEED, "AttrSpeedRunning"),
        (CharacterAttributes::OFFSET_FLYING_SPEED, "AttrSpeedFlying"),
        (CharacterAttributes::OFFSET_SWIMMING_SPEED, "AttrSpeedSwimming"),
        (CharacterAttributes::OFFSET_JUMPING_SPEED, "AttrSpeedJumping"),
        (CharacterAttributes::OFFSET_JUMP_HEIGHT, "AttrJumpHeight"),
        (CharacterAttributes::OFFSET_MOVEMENT_CONTROL, "AttrMovementControl"),
        (CharacterAttributes::OFFSET_MOVEMENT_FRICTION, "AttrMovementFriction"),
        (CharacterAttributes::OFFSET_STEALTH, "AttrStealth"),
        (CharacterAttributes::OFFSET_STEALTH_RADIUS_PVE, "AttrStealthRadius"),
        (CharacterAttributes::OFFSET_STEALTH_RADIUS_PVP, "AttrStealthRadiusPlayer"),
        (CharacterAttributes::OFFSET_PERCEPTION_RADIUS, "AttrPerceptionRadius"),
        (CharacterAttributes::OFFSET_REGENERATION, "AttrRegeneration"),
        (CharacterAttributes::OFFSET_RECOVERY, "AttrRecovery"),
        (CharacterAttributes::OFFSET_INSIGHT_RECOVERY, "AttrInsightRecovery"),
        (CharacterAttributes::OFFSET_THREAT_LEVEL, "AttrThreatLevel"),
        (CharacterAttributes::OFFSET_TAUNT, "AttrTaunt"),
        (CharacterAttributes::OFFSET_PLACATE, "AttrPlacate"),
        (CharacterAttributes::OFFSET_CONFUSED, "AttrConfused"),
        (CharacterAttributes::OFFSET_AFRAID, "AttrAfraid"),
        (CharacterAttributes::OFFSET_TERRORIZED, "AttrTerrorized"),
        (CharacterAttributes::OFFSET_HELD, "AttrHeld"),
        (CharacterAttributes::OFFSET_IMMOBILIZED, "AttrImmobilized"),
        (CharacterAttributes::OFFSET_STUNNED, "AttrStunned"),
        (CharacterAttributes::OFFSET_SLEEP, "AttrSleep"),
        (CharacterAttributes::OFFSET_FLY, "AttrFly"),
        (CharacterAttributes::OFFSET_JUMP_PACK, "AttrJumppack"),
        (CharacterAttributes::OFFSET_TELEPORT, "AttrTeleport"),
        (CharacterAttributes::OFFSET_UNTOUCHABLE, "AttrUntouchable"),
        (CharacterAttributes::OFFSET_INTANGIBLE, "AttrIntangible"),
        (CharacterAttributes::OFFSET_ONLY_AFFECTS_SELF, "AttrOnlyAffectsSelf"),
        (CharacterAttributes::OFFSET_EXPERIENCE_GAIN, "AttrExperienceGain"),
        (CharacterAttributes::OFFSET_INFLUENCE_GAIN, "AttrInfluenceGain"),
        (CharacterAttributes::OFFSET_PRESTIGE_GAIN, "AttrPrestigeGain"),
        (CharacterAttributes::OFFSET_EVADE, "AttrNullBool"),
        (CharacterAttributes::OFFSET_KNOCKUP, "AttrKnockup"),
        (CharacterAttributes::OFFSET_KNOCKBACK, "AttrKnock"), // sic
        (CharacterAttributes::OFFSET_REPEL, "AttrRepel"),
        (CharacterAttributes::OFFSET_ACCURACY, "AttrAccuracy"),
        (CharacterAttributes::OFFSET_RADIUS, "AttrRadius"),
        (CharacterAttributes::OFFSET_ARC, "AttrArc"),
        (CharacterAttributes::OFFSET_RANGE, "AttrRange"),
        (CharacterAttributes::OFFSET_TIME_TO_ACTIVATE, "AttrTimeToActivate"),
        (CharacterAttributes::OFFSET_RECHARGE_TIME, "AttrRechargeTime"),
        (CharacterAttributes::OFFSET_INTERRUPT_TIME, "AttrInterruptTime"),
        (CharacterAttributes::OFFSET_ENDURANCE_DISCOUNT, "AttrEnduranceDiscount"),
        // the client seems to be missing a string for elusivity (you can see it as "Attr(null)" in the game client)
        (CharacterAttributes::OFFSET_ELUSIVITY_0, "AttrDefenseType[0]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_1, "AttrDefenseType[1]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_2, "AttrDefenseType[2]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_3, "AttrDefenseType[3]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_4, "AttrDefenseType[4]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_5, "AttrDefenseType[5]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_6, "AttrDefenseType[6]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_7, "AttrDefenseType[7]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_8, "AttrDefenseType[8]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_9, "AttrDefenseType[9]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_10, "AttrDefenseType[10]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_11, "AttrDefenseType[11]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_12, "AttrDefenseType[12]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_13, "AttrDefenseType[13]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_14, "AttrDefenseType[14]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_15, "AttrDefenseType[15]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_16, "AttrDefenseType[16]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_17, "AttrDefenseType[17]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_18, "AttrDefenseType[18]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_19, "AttrDefenseType[19]"),
        (CharacterAttributes::OFFSET_ELUSIVITY_BASE, "AttrDefense"),
    ] {
        attrib_names.attr_names
            .insert(offset, messages.get_message(key).cloned());
    }

    verify_struct_length(attrib_names, expected_bytes, begin_pos, reader)
}

/// Reads an `AttribName` struct from a .bin file.
/// Refer to Common/entity/attrib_names.h TokenizerParseInfo structs.
///
/// # Arguments:
///
/// * `reader` - An open `Read` + `Seek`
/// * `strings` - The `StringPool` for attribute names
/// * `messages` - The global `MessageStore` containing client messages
///
/// # Returns:
///
/// If successful, a `AttribName`.
/// Otherwise, a `ParseError` with the error information.
fn read_attrib_name<T>(
    reader: &mut T,
    strings: &StringPool,
    messages: &MessageStore,
) -> ParseResult<AttribName>
where
    T: Read + Seek,
{
    let mut attrib_name = AttribName::new();
    let (expected_bytes, begin_pos) = read_struct_length(reader)?;
    attrib_name.pch_name = read_pool_string(reader, strings, messages)?;
    attrib_name.pch_display_name = read_pool_string(reader, strings, messages)?;
    attrib_name.pch_icon_name = read_pool_string(reader, strings, messages)?;
    verify_struct_length(attrib_name, expected_bytes, begin_pos, reader)
}
