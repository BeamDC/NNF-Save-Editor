mod save_data;
mod gui;
mod game_data;
mod parser;

use eframe::egui;
use crate::gui::Gui;

fn main() -> Result<(), eframe::Error> {
    Gui::run()
}