use std::fmt::Display;
use std::path::Path;
use eframe::App;
use rfd::FileDialog;
use crate::save_data::SaveData;

#[derive(Default)]
pub enum FileState {
    Success,
    #[default]
    None,
    Error(String),
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileState::Success => write!(f, "file successfully parsed"),
            FileState::Error(e) => write!(f, "Error: {}", e),
            FileState::None => write!(f, "No file provided"),
        }
    }
}

#[derive(Default)]
pub struct Gui {
    file_state: FileState,
    data: SaveData,
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // file selection
            if ui.button("select save file").clicked() {
                let file = FileDialog::new()
                    .add_filter("save files", &["save"])
                    .set_directory("/")
                    .pick_file();

                match file {
                    Some(p) => self.update_data(p),
                    None => self.file_state = FileState::Error("File path not found".to_owned())
                }
            }
            ui.label(self.file_state.to_string());
            ui.label(format!("{:?}", self.data.perks));
            ui.label(format!("{:?}", self.data.items));
        });
    }
}

impl Gui {
    fn update_data<P: AsRef<Path>>(&mut self, path: P) {
        let data = SaveData::from_path(path);
        match data {
            Ok(sd) => {
                self.data = sd;
                self.file_state = FileState::Success;
            }
            Err(e) => {
                self.file_state = FileState::Error(e.to_string());
            }
        }
}

    pub fn run() -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "NNF Save Editor",
            options,
            Box::new(|_cc| Ok(Box::new(Gui::default()))),
        )
    }
}