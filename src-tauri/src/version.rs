/*-------------
/version.rs

This file is for grabbing version and build information to return to frontend.

Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use std::env;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    target: String,
    version: String,
    description: String,
    rust_version: String,
    authors: Vec<String>,
    #[serde(flatten)]
    dependencies: CargoToml,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum DependencyValue {
    String(String),
    Object {
        version: String,
        features: Vec<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoToml {
    dependencies: HashMap<String, DependencyValue>,
}

#[tauri::command]
pub fn package_version() -> Package {
    let cargo_toml_raw = include_str!("../Cargo.toml");
    let mut cargo_toml: CargoToml = toml::from_str(cargo_toml_raw).expect("Cargo.toml is not valid TOML");

    // Filter down the results to only the dependencies we care about
    let remove_dependencies = ["serde", "serde_json","toml"];
    for item in remove_dependencies {
        cargo_toml.dependencies.remove(item);
    }

    Package {
        target: env::consts::OS.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        rust_version: env!("CARGO_PKG_RUST_VERSION").to_string(),
        authors: env!("CARGO_PKG_AUTHORS").split(':').map(|s| s.to_string()).collect(),
        dependencies: cargo_toml,
    }
}