use super::namekey::NameKey;

/// A list of boosts (enhancements) included in a `BoostSet`.
#[derive(Debug, Default)]
pub struct BoostList {
    pub ppch_boosts: Vec<NameKey>,
}

impl BoostList {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Bonus granted by a `BoostSet`.
#[derive(Debug, Default)]
pub struct BoostSetBonus {
    /// The display name of the bonus.
    pub pch_display_name: Option<String>,
    /// The number of distinct boosts required to activate this bonus.
    pub i_min_boosts: i32,
    /// The max number of distinct boosts required to keep this bonus. 0 means no max.
    pub i_max_boosts: i32,
    /// A power eval statement that may limit the availability of this bonus. If empty, it is not evaluated.
    pub ppch_requires: Vec<String>,
    /// The auto powers granted by this bonus.
    pub ppch_auto_powers: Vec<NameKey>,
    /// The general power granted by this bonus (may be a mix of boost attribs and additional attribs).
    pub pch_bonus_power: Option<NameKey>,
}

impl BoostSetBonus {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Structure for boost (enhancement) sets.
#[derive(Debug, Default)]
pub struct BoostSet {
    /// The internal name of the set.
    pub pch_name: Option<NameKey>,
    /// The display name of the set.
    pub pch_display_name: Option<String>,
    /// The display name of the set group.
    pub pch_group_name: Option<String>,
    /// The names of the conversion groups this boost set belongs to.
    pub ppch_conversion_groups: Vec<String>,
    /// The powers that can use this set.
    pub ppch_powers: Vec<NameKey>,
    /// The boosts that make up this set.
    pub pp_boost_lists: Vec<BoostList>,
    /// The bonuses granted by this set.
    pub pp_bonuses: Vec<BoostSetBonus>,
    /// Minimum level for this boost set.
    pub i_min_level: i32,
    /// Maximum level for this boost set.
    pub i_max_level: i32,
    /// Product code that must be available for this boost set to be a valid conversion.
    pub pch_store_product: Option<String>,
}

impl BoostSet {
    pub fn new() -> Self {
        Default::default()
    }
}
