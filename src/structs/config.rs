use super::NameKey;
use chrono::{DateTime, Local};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use toml;

/// Configuration information for image assets.
#[derive(Debug, Deserialize)]
pub struct AssetsConfig {
    /// Set the base URL for images.
    pub base_asset_url: String,
    /// Set the normalized extension for images (filenames extracted from bins will be replaced with this extension).
    pub ext: String,
    /// A format string specifying the URL format for archetypes.
    pub archetype_icon_format: String,
    /// A format string specifying the URL format for powers.
    pub powers_icon_format: String,
}

/// Configuration information for JSON output style.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputStyleConfig {
    /// Inserts whitespace so that JSON is human-readable.
    Pretty,
    /// Saves space by removing unnecessary whitespace.
    Compact,
}

impl Default for OutputStyleConfig {
    fn default() -> Self {
        OutputStyleConfig::Compact
    }
}

/// Configuration information for the current run.
#[derive(Debug, Deserialize)]
pub struct PowersConfig {
    /// Issue that the data were extracted from.
    pub issue: String,
    /// Arbitrary string identifying the source of the data.
    pub source: String,
    /// Local date/time that the data were extracted. (Set at runtime.)
    #[serde(skip)]
    pub extract_date: Option<DateTime<Local>>,
    /// JSON output style.
    #[serde(default)]
    pub output_style: OutputStyleConfig,
    /// Determines the security level used for power calculations.
    pub at_level: i32,
    /// Set the base URL for generated JSON assets.
    pub base_json_url: Option<String>,
    /// For future use.
    pub assets: Option<AssetsConfig>,
    /// Where to find the extracted .bin files.
    pub input_path: String,
    /// Where the JSON files will be written.
    pub output_path: String,
    /// List of power categories to use as a filter. If empty, nothing will be filtered.
    pub power_categories: Vec<NameKey>,
    /// List of power categories to assign to all archetypes. Used to heal up some
    /// troublesome spots like epic pools and incarnate powers.
    pub global_categories: Vec<NameKey>,
    /// List of power set partial name matches to filter. Used to get rid of some
    /// power sets we don't want that are part of included power categories.
    pub filter_powersets: Vec<NameKey>,
}

impl PowersConfig {
    /// Parses a .toml file to create a `PowersConfig`.
    ///
    /// # Arguments:
    ///
    /// * `path` - The path to the .toml file to parse.
    ///
    /// # Returns:
    ///
    /// If successful, a `PowersConfig`. Otherwise, a `std::io::Error`.
    pub fn load(path: &Path) -> Result<PowersConfig> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        let _ = file.read_to_string(&mut buf)?;
        let mut config: PowersConfig =
            toml::from_str(&buf).map_err(|e| Error::new(ErrorKind::Other, e))?;
        config.extract_date = Some(Local::now());
        assert!(
            config.at_level > 0 && config.at_level < 51,
            "at_level must be between 1 and 50 (inclusive)"
        );
        Ok(config)
    }

    /// Joins a subpath to the `input_path`.
    ///
    /// # Arguments:
    ///
    /// * `path` - A subpath to append to the `input_path`.
    ///
    /// # Returns:
    ///
    /// A `PathBuf`.
    pub fn join_to_input_path(&self, path: &str) -> PathBuf {
        Path::new(&self.input_path).join(path)
    }

    /// Joins a subpath to the `output_path`.
    ///
    /// # Arguments:
    ///
    /// * `path` - A subpath to append to the `output_path`.
    ///
    /// # Returns:
    ///
    /// A `PathBuf`.
    pub fn join_to_output_path(&self, path: &str) -> PathBuf {
        Path::new(&self.output_path).join(path)
    }
}
