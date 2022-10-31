use crate::game_state;
use std::fs;
use std::{fs::File, os::unix::prelude::FileExt, path::Path};

use std::io::prelude::*;

const default_file: &str = "save.yaml";
const default_folder: &str = ".slay-the-spire";

pub(crate) struct Loader {
    folder: String,
    file: String,
}

impl Loader {
    pub(crate) fn new() -> Self {
        Self {
            folder: default_folder.to_string(),
            file: default_file.to_string(),
        }
    }

    pub(crate) fn with_file(self, file: String) -> Self {
        Self {
            folder: self.folder,
            file,
        }
    }

    pub(crate) fn with_folder(self, folder: String) -> Self {
        Self {
            folder,
            file: self.file,
        }
    }

    pub(crate) fn load(&self) -> game_state::GameState {
        let file_name = Path::new(&self.folder).join(&self.file);

        if !file_name.exists() {
            fs::create_dir_all(&file_name).unwrap();
            File::create(&file_name).unwrap();
        }

        if fs::read_to_string(&file_name).unwrap().is_empty() {
            return game_state::GameState::new();
        }

        let file = File::open(&file_name).unwrap();
        serde_yaml::from_reader(file).expect("Could not read values.")
    }

    pub(crate) fn save(self, game_state: &game_state::GameState) {
        let file_name = Path::new(&self.folder).join(self.file);

        if !file_name.exists() {
            fs::create_dir_all(&file_name).unwrap();
            File::create(&file_name).unwrap();
        }

        let file = File::options().write(true).open(&file_name).unwrap();
        serde_yaml::to_writer(file, game_state).unwrap();
    }
}
