use std::path::{Path, PathBuf};

use crate::core::simulation::circuit::{Circuit, CircuitIdx};
use crate::player::circuit::CircuitManager;
use crate::player::CirquilPlayerApp;
use crate::player::osc::Oscilloscope;
use crate::serde::fs::{deserialize_from_file, serialize_to_file};
use crate::serde::replay::ReplayFile;

#[derive(Debug, Default)]
pub struct ReplayManager {
    pub replay: Vec<Vec<(Circuit, CircuitIdx)>>,
}

impl ReplayManager {
    pub fn push_frame(&mut self, frame: Vec<(Circuit, CircuitIdx)>) {
        self.replay.push(frame);
    }

    pub fn clear(&mut self) {
        self.replay.clear();
    }
}

impl CirquilPlayerApp {
    pub fn save_replay<P>(&self, path: P)
        where P: AsRef<Path>
    {
        let circuits = self.circuit_manager.get_circuits();

        let replay_file = ReplayFile {
            top_circuit: self.top_circuit,
            states: self.replay_manager.replay.clone(),
            canvas_circuits: circuits.canvas_circuits.clone(),
            simulation_tree: circuits.simulation_tree.clone(),
            by_uuid: circuits.by_uuid.clone(),
            parents: circuits.parents.clone(),
        };

        serialize_to_file(&replay_file, path).unwrap();
    }

    pub fn load_replay<P>(&mut self, path: P)
        where P: AsRef<Path>
    {
        let replay_file: ReplayFile = deserialize_from_file(path).unwrap();

        self.top_circuit = replay_file.top_circuit;
        self.current_circuit = replay_file.top_circuit;
        self.circuit_manager = CircuitManager::create_replay(replay_file);
        self.probes = vec![];
        self.probe_max_id = 0;
        
        self.osc = Oscilloscope::default();
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
