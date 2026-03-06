use anyhow::Result;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub name: String,
    pub folder: String,
    pub extensions: Option<Vec<String>>,
    pub subrules: Option<Vec<Rule>>,
}

pub fn get_config_path(config_arg: &String) -> Option<PathBuf> {
    let path = PathBuf::from(config_arg);

    if !path.exists() {
        println!("Config file not found: {:?}", path);
        return None;
    }

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext != "yml" && ext != "yaml" {
        println!("ERROR: Config file must be .yml or .yaml");
        return None;
    }

    Some(path)
}

pub fn load_config() -> Result<Option<Config>> {
    todo!();
}
