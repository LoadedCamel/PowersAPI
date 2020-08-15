use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};

const WILDCARD: &'static str = "*";
const SEPARATOR: char = '.';

/// A `NameKey` represents a string that is used when matching
/// various structs in the data set. The important distinction from plain
/// strings is that name keys are case-insensitive so they have to be
/// handled in a special way.
#[derive(Clone)]
pub struct NameKey(String);

impl NameKey {
    /// Creates a new `NameKey` from `key`.
    pub fn new<T>(key: T) -> Self
    where
        T: Into<String>,
    {
        NameKey(key.into())
    }

    /// Gets the string slice representation of this `NameKey`.
    pub fn get(&self) -> &str {
        &self.0[..]
    }

    /// Gets the string representation of this `NameKey`.
    pub fn get_string(&self) -> &String {
        &self.0
    }

    /// Tests whether or not this `NameKey` instance represents a "wildcard".
    /// This is used for matching one-to-many relationships during load.
    pub fn is_wildcard(&self) -> bool {
        self.0 == WILDCARD
    }

    /// Tests if `other` is contained in any part of this `NameKey`.
    pub fn partial_match(&self, other: &str) -> bool {
        self.0
            .to_ascii_lowercase()
            .contains(&other.to_ascii_lowercase())
    }

    /// Returns a collection of slices over the `NameKey`, based
    /// on seperating it using the default character (`.`).
    pub fn split(&self) -> Vec<&str>
    {
        self.0.split(SEPARATOR).collect()
    }
}

impl From<String> for NameKey {
    fn from(string: String) -> Self {
        NameKey::new(string)
    }
}

impl Eq for NameKey {}

impl PartialEq for NameKey {
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self
                .0
                .chars()
                .zip(other.0.chars())
                .all(|(s, o)| s.eq_ignore_ascii_case(&o))
    }
}

impl PartialEq<str> for NameKey {
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.
    fn eq(&self, other: &str) -> bool {
        self.0.len() == other.len()
            && self
                .0
                .chars()
                .zip(other.chars())
                .all(|(s, o)| s.eq_ignore_ascii_case(&o))
    }
}

impl Hash for NameKey {
    /// Feeds this value into the given `Hasher`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state)
    }
}

impl fmt::Display for NameKey {
    /// Formats the value using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl fmt::Debug for NameKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NameKey(\"{}\")", self.get())
    }
}

impl<'de> Deserialize<'de> for NameKey {
    fn deserialize<D>(deserializer: D) -> Result<NameKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Ok(NameKey::new(s))
    }
}

impl Serialize for NameKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.get())
    }
}
