mod structs;

use crate::structs::config::{OutputStyleConfig, PowersConfig};
use crate::structs::{
    Archetype, AttribNames, BasePowerSet, Keyed, PowerCategory, PowersDictionary,
};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::rc::Rc;
use structs::*;

/// Default name for the .json files.
const JSON_FILE: &'static str = "index.json";

/// Begins the process of writing the entire powers dictionary to disk as .json files.
///
/// # Arguments:
///
/// * `powers_dict` - A `PowersDictionary` containing a hierarchy of categories, power sets, and powers.
/// * `config` - Configuration information.
///
/// # Returns:
///
/// Nothing if the operation was successful. Otherwise, an `io::Error` containing the error information.
///
/// # Notes:
///
/// The data is written as a hierarchy of individual .json files stored in folders. The reason for the layout
/// is to facilitate access from a web server. For example, you can get to the Tanker version of Super Strength
/// using a URL that looks like this:
///
/// `http://myserver/powers/tanker-melee/super-strength/`
pub fn write_powers_dictionary(
    powers_dict: PowersDictionary,
    config: &PowersConfig,
) -> io::Result<()> {
    // setup the output directory
    let output_path = Path::new(&config.output_path);
    fs::create_dir_all(&output_path)?;
    if output_path.read_dir()?.count() > 0 {
        print!(
            "WARNING! The output path {} is not empty. Overwrite? (y/n)",
            output_path.display()
        );
        io::stdout().flush()?;
        //TODO: better input handling
        for c in io::stdin().lock().bytes() {
            match c? {
                b'y' | b'Y' => break,
                b'n' | b'N' => return Err(Error::from(ErrorKind::Interrupted)),
                _ => (),
            }
        }
        println!();
    }

    // write the root file
    write_root(&powers_dict.power_categories, config)?;

    // write archetypes
    write_archetypes(&powers_dict.archetypes, config)?;

    // write all of the categories
    for category in &powers_dict.power_categories {
        if !category.include_in_output {
            continue;
        }
        write_power_category(category, config)?;

        if let Some(pcat_name) = category.pch_name.as_ref() {
            // write the category's power sets
            for set in &category.pp_power_sets {
                if set.include_in_output {
                    write_power_set(
                        Some(pcat_name.get_string()),
                        set,
                        &powers_dict.attrib_names,
                        config,
                    )?;
                }
            }
        }
    }

    Ok(())
}

/// Writes the root .json file.
fn write_root(power_categories: &Vec<Rc<PowerCategory>>, config: &PowersConfig) -> io::Result<()> {
    let output_file = config.join_to_output_path(JSON_FILE);
    println!("Writing: {} ...", output_file.display());
    let mut f = fs::File::create(output_file)?;
    let root = RootOutput::from_power_categories(power_categories, config);
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, &root)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, &root)?,
    }
    Ok(())
}

/// Writes the archetypes .json file.
fn write_archetypes(archetypes: &Keyed<Archetype>, config: &PowersConfig) -> io::Result<()> {
    let output_path = config.join_to_output_path("archetypes");
    fs::create_dir_all(&output_path)?;
    let output_file = output_path.join(JSON_FILE);
    println!("Writing: {} ...", output_file.display());
    let mut f = fs::File::create(output_file)?;
    let ats = ArchetypesOutput::from_archetypes(archetypes, config);
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, &ats)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, &ats)?,
    }
    Ok(())
}

/// Writes all of the power category .json files to individual directories.
fn write_power_category(power_category: &PowerCategory, config: &PowersConfig) -> io::Result<()> {
    if let Some(category_name) = &power_category.pch_name {
        let output_path = config.join_to_output_path(&make_file_name(category_name.get()));
        fs::create_dir_all(&output_path)?;
        let output_file = output_path.join(JSON_FILE);
        println!("Writing: {} ...", output_file.display());
        let mut f = fs::File::create(output_file)?;

        let pcat = PowerCategoryOutput::from_power_category(power_category, config);
        match config.output_style {
            OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, &pcat)?,
            OutputStyleConfig::Compact => serde_json::to_writer(&mut f, &pcat)?,
        }
    }
    Ok(())
}

/// Writes all of the power set .json files to individual directories beneath the power categories.
fn write_power_set(
    category_name: Option<&String>,
    power_set: &BasePowerSet,
    attrib_names: &AttribNames,
    config: &PowersConfig,
) -> io::Result<()> {
    let output_path = config
        .join_to_output_path(&make_file_name_opt(category_name))
        .join(&make_file_name_opt(power_set.pch_name.as_ref()));
    fs::create_dir_all(&output_path)?;
    let output_file = output_path.join(JSON_FILE);
    println!("\tWriting: {} ...", output_file.display());
    let mut f = fs::File::create(output_file)?;

    let pset = PowerSetOutput::from_base_power_set(
        power_set,
        attrib_names,
        config,
    );
    match config.output_style {
        OutputStyleConfig::Pretty => serde_json::to_writer_pretty(&mut f, &pset)?,
        OutputStyleConfig::Compact => serde_json::to_writer(&mut f, &pset)?,
    }

    Ok(())
}

/// Takes a string of arbitrary data and attempts to create a representation suitable for use
/// as a file name.
fn make_file_name_opt(string: Option<&String>) -> String {
    if let Some(s) = string {
        make_file_name(&s[..])
    } else {
        String::from("")
    }
}

/// Takes a string of arbitrary data and attempts to create a representation suitable for use
/// as a file name.
fn make_file_name(string: &str) -> String {
    let mut s = String::new();
    for c in string.chars() {
        if c.is_alphanumeric() {
            for c in c.to_lowercase() {
                s.push(c);
            }
        } else if c.is_whitespace() || c == '_' || c == '-' || c == '.' {
            s.push('-');
        }
    }
    s
}
