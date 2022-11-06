use crate::game_state;
use std::fmt::format;
use std::fs;
use std::path::PathBuf;
use std::{fs::File, path::Path};

use std::io::prelude::*;

const DEFAULT_FILE: &str = "save.yaml";
const DEFAULT_FOLDER: &str = ".slay-the-spire";
const METADATA_FILE: &str = ".metadata.yaml";

pub(crate) struct Loader {
    folder_name: String,
    save_file_name: String,
}

impl Loader {
    pub(crate) fn new() -> Self {
        let folder = Path::new(DEFAULT_FOLDER);

        let save_file_name;
        let metadata_path = folder.join(METADATA_FILE);
        if !metadata_path.exists() {
            fs::create_dir_all(folder).unwrap();
            let mut metadata_file = File::options()
                .write(true)
                .truncate(true)
                .open(&metadata_path)
                .unwrap();
            write!(metadata_file, "{}", DEFAULT_FILE.to_string()).unwrap();
        }

        save_file_name = fs::read_to_string(&metadata_path).unwrap();

        Self {
            folder_name: DEFAULT_FOLDER.to_string(),
            save_file_name,
        }
    }

    pub(crate) fn with_file(self, save_name: String) -> Self {
        let save_file_name = format!("{}.yaml", save_name);
        let folder = Path::new(DEFAULT_FOLDER);
        let metadata_path = folder.join(METADATA_FILE);

        let mut output = File::options()
            .write(true)
            .truncate(true)
            .open(&metadata_path)
            .unwrap();
        write!(output, "{}", save_file_name).unwrap();

        Self {
            folder_name: self.folder_name,
            save_file_name,
        }
    }

    pub(crate) fn load(&self) -> game_state::GameState {
        let folder = Path::new(&self.folder_name);
        let file_name = folder.join(&self.save_file_name);

        if !file_name.exists() {
            fs::create_dir_all(Path::new(&self.folder_name)).unwrap();
            File::create(&file_name).unwrap();
        }

        if fs::read_to_string(&file_name).unwrap().is_empty() {
            return game_state::GameState::new();
        }

        let file = File::open(&file_name).unwrap();
        serde_yaml::from_reader(file).expect("Could not read values.")
    }

    pub(crate) fn save(&self, game_state: &game_state::GameState) {
        let folder = Path::new(&self.folder_name);
        let file_path = folder.join(&self.save_file_name);

        if !file_path.exists() {
            fs::create_dir_all(folder).unwrap();
            File::create(&file_path).unwrap();
        }

        let file = File::options()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .unwrap();
        serde_yaml::to_writer(file, game_state).unwrap();
    }

    pub(crate) fn delete(&self) {
        let folder = Path::new(&self.folder_name);
        let file_name = folder.join(&self.save_file_name);

        if file_name.exists() {
            fs::remove_file(&file_name).unwrap();
        }
    }
}
