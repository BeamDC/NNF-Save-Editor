use std::io::Read;
use chumsky::Parser;
use crate::game_data::{Item, Perk};
use crate::save_data::SaveData;

pub struct SaveDataParser {
    pub raw: String,
}

impl SaveDataParser {
    pub fn new(bytes: Vec<u8>) -> SaveDataParser {
        SaveDataParser {
            raw: String::from_utf8(bytes).unwrap(),
        }
    }

    /// build the parser for perks
    fn build_perk<'sd>() -> impl Parser<'sd, &'sd str, Vec<Perk>>{
        todo!()
    }

    /// build the parser for items
    fn build_item<'sd>() -> impl Parser<'sd, &'sd str, Vec<Item>>{
        todo!()
    }

    /// build the parser for the save data
    fn parse() -> SaveData {
        todo!()
    }
}