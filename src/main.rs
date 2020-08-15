#![feature(get_mut_unchecked)]

#[macro_use]
extern crate bitflags;
extern crate chrono;
extern crate md5;
extern crate num_enum;
extern crate serde;
extern crate serde_json;
extern crate toml;

mod bin_parse;
mod load;
mod output;
mod structs;

use bin_parse::{ParseError, ParseErrorKind};
use std::borrow::Cow;
use std::env;
use std::ffi::OsString;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process;
use std::time::Instant;
use structs::config::PowersConfig;

/// Default name for the config file.
const CONFIG_FILE: &'static str = "PowersConfig.toml";

/// Program entry point.
fn main() {
    // get path to configuration
    let config_path = get_config_path();

    // load configuration
    let config = PowersConfig::load(&config_path).unwrap_or_else(|e| {
        println!(
            "Unable to load {}. {}",
            config_path.display(),
            get_io_error(&e)
        );
        process::exit(1);
    });
    println!("Configuration loaded.");

    // parse the powers dictionary
    let powers_dict = load::load_powers_dictionary(&config).unwrap_or_else(|context| {
        println!("{} {}.", context.message, get_error(&context.error));
        process::exit(1);
    });
    println!("Powers dictionary loaded.");

    // write output files
    let begin_time = Instant::now();
    if let Err(e) = output::write_powers_dictionary(powers_dict, &config) {
        println!("Unable to write ouput files! {}", get_io_error(&e));
        process::exit(1);
    }
    let elapsed = Instant::now().duration_since(begin_time);
    println!("Files written in {} seconds.", elapsed.as_secs());
}

/// Optionally read path to config file from command line. Otherwise use
/// `CONFIG_FILE` in the current directory.
fn get_config_path() -> PathBuf {
    let mut config_path: Option<OsString> = None;
    for arg in env::args_os().skip(1) {
        if config_path.is_none() {
            config_path = Some(arg);
        } else {
            println!("Too many command line arguments.");
            process::exit(1);
        }
    }
    if let Some(config_path) = &config_path {
        let mut path = PathBuf::from(config_path);
        if path.is_dir() {
            path.push(CONFIG_FILE);
        }
        path
    } else {
        PathBuf::from(CONFIG_FILE)
    }
}

/// Converts a `ParseError` into a human-readable string.
fn get_error(error: &ParseError) -> Cow<'static, str> {
    match error.kind() {
        ParseErrorKind::MissingCrypticSig => {
            Cow::Borrowed("Missing Cryptic signature (is this a real bin?)")
        }
        ParseErrorKind::StringConversion => {
            Cow::Borrowed("Could not convert string (corrupted bin?)")
        }
        ParseErrorKind::WrongFileType => {
            Cow::Borrowed("Wrong file type encountered (did you copy the wrong bin?)")
        }
        ParseErrorKind::ReadError => {
            let io_error = error.get_io_error_ref().unwrap();
            get_io_error(io_error)
        }
        ParseErrorKind::SizeMismatch {
            expected_bytes,
            read_bytes,
        } => Cow::Owned(format!(
            "Expected {} bytes in struct, but read {} bytes",
            expected_bytes, read_bytes
        )),
        ParseErrorKind::MissingNameKey => {
            Cow::Borrowed("Current object has no name key (corrupted bin?")
        }
    }
}

/// Converts a `std::io::Error` into a human-readable string.
fn get_io_error(error: &Error) -> Cow<'static, str> {
    match error.kind() {
        ErrorKind::NotFound => Cow::Borrowed("No such file or directory"),
        ErrorKind::PermissionDenied => Cow::Borrowed("Access denied"),
        ErrorKind::AlreadyExists => Cow::Borrowed("File already exists"),
        ErrorKind::TimedOut => Cow::Borrowed("Operation timed out"),
        ErrorKind::UnexpectedEof => Cow::Borrowed("Unexpected end of file"),
        ErrorKind::Interrupted => Cow::Borrowed("Operation interrupted"),
        ErrorKind::Other => {
            if let Some(e) = error.get_ref() {
                Cow::Owned(format!("Other error: {}", e))
            } else {
                Cow::Borrowed("Unknown error")
            }
        }
        _ => Cow::Owned(format!("Other error {:?}", error.kind())),
    }
}
