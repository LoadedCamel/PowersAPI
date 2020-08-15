use std::collections::HashMap;
use std::str;

/// Describes an individual entry in a `MessageStore`. When keyed to a descriptive string, this
/// can be used to map that key to readable text.
#[derive(Debug)]
pub struct TextMessage {
	/// The index into `MessageStore.messages` the message.
	message_index: usize,
	/// The index into `MessageStore.messages` for help text.
	help_index: usize,
	/// Any variables used in the message. (Note: the type is always the value +1.)
	vars: Option<Vec<u32>>,
}

impl TextMessage {
	/// Creates a new `TextMessage`.
	pub fn new(message_index: usize, help_index: usize, vars: Option<Vec<u32>>) -> Self {
		TextMessage {
			message_index,
			help_index,
			vars,
		}
	}
}

/// Represents a .bin store of client messages, used for mapping P-strings into readable text.
#[derive(Debug, Default)]
pub struct MessageStore {
	/// The table of message strings.
	pub messages: Vec<String>,
	/// The table of variables and variable types.
	pub variables: Vec<String>,
	/// The map of message IDs to message strings.
	pub message_ids: HashMap<String, TextMessage>,
}

impl MessageStore {
	/// Creates a new `MessageStore`.
	pub fn new() -> Self {
		Default::default()
	}

	/// Gets the message identified by `key`.
	/// 
	/// Arguments:
	/// 
	/// * `key` - A key to look up in the message store, such as "P1260685325".
	/// 
	/// Returns:
	/// 
	/// A reference to the localized text matching the given `key`, if it is valid.
	pub fn get_message(&self, key: &str) -> Option<&String> {
		if let Some(t) = self.message_ids.get(key) {
			return self.messages.get(t.message_index);
		}
		None
	}

	/// Gets the number of message ID entries in the store.
	/// 
	/// Returns:
	/// 
	/// A `usize`.
	pub fn len_ids(&self) -> usize {
		self.message_ids.len()
	}
}

/// Describes a struct that can be converted into a message from a `MessageStore`.
pub trait IntoMessage {
    /// Convert the current struct into a message from `message_store`.
    /// 
    /// # Arguments:
    /// 
    /// * `message_store` - A `MessageStore`.
    /// 
    /// # Returns:
    /// 
    /// If successful, a corresponding `String` from `message_store`.
    /// Otherwise, None.
    fn into_message(&self, message_store: &MessageStore) -> Option<String>;
}

impl IntoMessage for Option<&str> {
    /// Converts an `Option<String>` into a message from a `MessageStore`. This is a convenience
    /// function that makes it easy to swap stored strings into messages while parsing.
    /// 
    /// # Arguments:
    /// 
    /// * `message_store` - A `MessageStore`.
    /// 
    /// # Returns:
    /// 
    /// If successful, a corresponding `String` from `message_store`.
    /// Otherwise, this preserves the original string (so fields that aren't messages just pass through).
    fn into_message(&self, message_store: &MessageStore) -> Option<String> {
        if let Some(s) = self.as_ref() {
            if let Some(m) = message_store.get_message(s) {
                return Some(m.clone());
            } else {
				return Some(String::from(*s));
			}
		}
		None
    }
}

/// Represents a pool of strings from a .bin file. A string pool is a series of NUL-terminated
/// strings that are referenced by offset from struct fields in the .bin.
#[derive(Debug)]
pub struct StringPool(Vec<u8>);

impl StringPool {
	/// Create a new string pool, initialized with the bytes in `pool`.
	///
	/// # Arguments
	///
	/// * `pool` - A vector of bytes populated with the string pool data.
	///
	/// # Returns
	///
	/// A `StringPool`.
	pub fn new(pool: Vec<u8>) -> Self {
		debug_assert!(pool.len() == 0 || *pool.first().unwrap() == 0u8, "StringPool should start with NUL");
		debug_assert!(pool.len() == 0 || *pool.last().unwrap() == 0u8, "StringPool should end with NUL");
		StringPool(pool)
	}

	/// Attempts to get the string at offset `offset` in the pool, by reading until it encounters a NUL
	/// terminator.
	///
	/// # Arguments
	///
	/// * `offset` - The offset into the string pool where reading should start.
	///
	/// # Returns
	/// 
	/// If successful, a slice representing the string at `offset`. Will return `None` if the string
	/// is empty or could not be parsed into a valid UTF-8 string.
	pub fn get_string(&self, offset: usize) -> Option<&str> {
		if offset > 0 && offset < self.0.len() {
			if let Some(end) = self.0[offset..].iter().enumerate().find(|(_, b)| **b == 0) {
				if let Ok(s) = str::from_utf8(&self.0[offset..offset + end.0]) {
					return Some(s);
				}
			}
		}
		None
	}
}
