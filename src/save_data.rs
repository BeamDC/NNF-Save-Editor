use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::Path;
use crate::game_data::{Item, Perk};
use crate::parser::SaveDataParser;

#[derive(Debug)]
pub enum DataError {
    ParseError(String),
}

impl Display for DataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}

impl Error for DataError {}


/// self-explanatory
#[derive(Default)]
pub struct SaveData {
    pub path: String,
    /// every [`Perk`] the player has
    pub perks: Vec<Perk>,
    /// every [`Item`] the player has
    pub items: Vec<Item>,
}

impl SaveData {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, DataError> {
        let bytes = fs::read_to_string(path).unwrap();
        let raw = bytes.as_str();
        SaveDataParser::parse(raw)
    }
}