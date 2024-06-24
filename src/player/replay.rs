use std::path::{Path, PathBuf};

use crate::core::compiler::project::InstantiatedCircuits;
use crate::serde::fs::serialize_to_file;
use crate::serde::replay::ReplayFile;

#[derive(Debug, Default)]
pub struct ReplayManager {
    replay: Vec<InstantiatedCircuits>,
}

impl ReplayManager {
    pub fn push_frame(&mut self, frame: InstantiatedCircuits) {
        self.replay.push(frame);
    }

    pub fn clear(&mut self) {
        self.replay.clear();
    }

    pub fn save_replay<P>(&self, path: P)
        where P: AsRef<Path>
    {
        let replay_file = ReplayFile { states: self.replay.clone() };

        serialize_to_file(&replay_file, path).unwrap();
    }
}

pub fn show_load_replay_file_dialogue() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Cirquil Replay", vec!["crp"].as_slice())
        .pick_file()
}

pub fn show_save_replay_file_dialogue() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Cirquil Replay", vec!["crp"].as_slice())
        .save_file()
}