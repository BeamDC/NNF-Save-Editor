use crate::game_data::{Item, Perk};
use crate::save_data::{DataError, SaveData};

pub struct SaveDataParser;

impl<'sd> SaveDataParser {
    const PERKS: &'sd str = "\"A_Perks\":";
    const ITEMS: &'sd str = "\"A_Items\":";

    /// build the parser for perks
    pub fn get_perks(raw: &str) -> Result<Vec<Perk>, DataError> {
        let pos = match raw.find(Self::PERKS) {
            Some(pos) => pos,
            None => return Err(DataError::ParseError("could not find perks".to_owned()))
        } + Self::PERKS.len() + 1;
        let perk_end = raw[pos..].find("\"").unwrap() + pos;
        let perks = raw[pos..perk_end]
            .trim()
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| unsafe {
                let id = s.parse::<u8>().unwrap();
                println!("{}", id);
                Perk::from_id_unchecked(id)
            })
            .collect::<Vec<Perk>>();
        Ok(perks)
    }

    /// build the parser for items
    pub fn get_items(raw: &str) -> Result<Vec<Item>, DataError> {
        let pos = match raw.find(Self::ITEMS) {
            Some(pos) => pos,
            None => return Err(DataError::ParseError("could not find items".to_owned()))
        } + Self::ITEMS.len() + 1;
        let item_end = raw[pos..].find("\"").unwrap() + pos;
        let items = raw[pos..item_end]
            .trim()
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| unsafe {
                let id = s.parse::<i16>().unwrap();
                println!("{}", id);
                Item::from_id_unchecked(id)
            })
            .collect::<Vec<Item>>();
        Ok(items)
    }
    
    pub fn parse(raw: &str) -> Result<SaveData, DataError> {
        let perks = Self::get_perks(raw)?;
        let items = Self::get_items(raw)?;
        Ok(SaveData {
            path: "".to_string(),
            perks,
            items,
        })
    }
}