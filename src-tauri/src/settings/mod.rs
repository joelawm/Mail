/*-------------
/settings/mod.rs

Contains a struct of potential settings so we can load previous states and settings from the user computer.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use serde::Deserialize;
use std::{fs::File, io::prelude::*};

mod init;

// Global Static Variables
lazy_static! {
	pub static ref CONFIG: Settings = load_settings();
}

#[derive(Deserialize)]
pub struct Settings {
	pub client: Vec<ClientSettings>,
	pub dark_mode: bool,
}

#[derive(Deserialize)]
pub struct ClientSettings {
	pub email: String,
	pub mailboxes: Vec<(String, String)>,
	pub imported: bool,
}

/// Load the settings from the skutl_lib config file without AWS.
pub fn load_settings() -> Settings {
	let doc = create_toml_config();

	toml::from_str::<Settings>(&doc.to_string()).unwrap_or_else(|e| panic!("Settings Toml file is missing data. - {}", e))
}

/// Grab the contents of the config file.
/// 
/// # Arguments
/// * `profile_arg` - The profile name to load from the config file.
fn create_toml_config() -> toml_edit::DocumentMut {
	// Load the differently depending on build
	let mut file: File = match File::open("settings.toml") {
		Ok(f) => f,
		Err(_) => write_toml_config(),
	};

	// Load the contents
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap_or_else(|e| {
		panic!("API - Can't open File! - {}", e);
	});

	// Parse the content string into a Toml Document
	contents.parse::<toml_edit::DocumentMut>().unwrap_or_else(|e| {
		panic!("The settings.toml File was malformed. Please check it for issues. - {}", e);
	})
}

/// Creates and writes the settings.toml file with the default settings.
fn write_toml_config() -> File {
	// Load the differently depending on build
	let mut file: File = File::create("settings.toml").unwrap_or_else(|e| {
		panic!("API - Can't open File! - {}", e);
	});

	// Load the contents
	let contents = init::init_toml();

	file.write_all(contents.as_bytes()).unwrap_or_else(|e| {
		panic!("API - Can't open File! - {}", e);
	});
	file
}