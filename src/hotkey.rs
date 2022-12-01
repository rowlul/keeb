use std::fs;

use anyhow::{Context, Result};
use rdev::{Button, Key};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hotkey {
    pub key: Key,
    pub button: Button,
    pub position: Position,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Outer {
    #[serde(default, rename = "hotkey")]
    pub hotkeys: Vec<Option<Hotkey>>,
}

impl Outer {
    pub fn from_path(path: &str) -> Result<Self> {
        return Ok(toml::from_str(
            &fs::read_to_string(&path).with_context(|| format!("Could not read file {}", &path))?,
        )
        .with_context(|| format!("Could not parse hotkey file"))?);
    }
}
