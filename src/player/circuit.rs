use std::rc::Rc;

use crate::core::compiler::project::InstantiatedCircuits;
use crate::core::simulation::circuit::{Circuit, CircuitIdx};
use crate::serde::replay::ReplayFile;

pub type FrameIdx = usize;

#[derive(Debug)]
pub enum PlaybackType {
    Simulation,
    Replay(ReplayFile, FrameIdx),
}

impl PlaybackType {
    pub fn is_simulation(&self) -> bool {
        if let PlaybackType::Simulation = self {
            true
        } else {
            false
        }
    }

    pub fn is_replay(&self) -> bool {
        if let PlaybackType::Replay(_, _) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct CircuitManager {
    pub circuits: InstantiatedCircuits,
    pub playback_type: PlaybackType,
}

impl CircuitManager {
    pub fn get_circuits(&self) -> &InstantiatedCircuits {
        &self.circuits
    }

    pub fn set_frame(&mut self, frame: FrameIdx) {
        if let PlaybackType::Replay(replay_file, current_frame) = &mut self.playback_type {
            *current_frame = frame;

            self.circuits.instantiated_circuits = Self::wrap_in_rc(replay_file.states.get(frame).unwrap())
        } else {
            debug_assert!(false);
        }
    }

    pub fn get_frame(&self) -> FrameIdx {
        if let PlaybackType::Replay(_, current_frame) = &self.playback_type {
            *current_frame
        } else {
            debug_assert!(false);
            0
        }
    }

    pub fn get_total_frames(&self) -> usize {
        if let PlaybackType::Replay(replay_file, _) = &self.playback_type {
            replay_file.states.len()
        } else {
            debug_assert!(false);
            0
        }
    }

    pub fn create_simulation(circuits: InstantiatedCircuits) -> Self {
        Self {
            circuits,
            playback_type: PlaybackType::Simulation,
        }
    }

    pub fn create_replay(replay_file: ReplayFile) -> Self {
        Self {
            circuits: InstantiatedCircuits {
                canvas_circuits: replay_file.canvas_circuits.clone(),
                instantiated_circuits: Self::wrap_in_rc(replay_file.states.get(0).unwrap()),
                simulation_tree: replay_file.simulation_tree.clone(),
                by_uuid: replay_file.by_uuid.clone(),
                parents: replay_file.parents.clone(),
            },
            playback_type: PlaybackType::Replay(replay_file, 0),
        }
    }

    fn wrap_in_rc(state: &[(Circuit, CircuitIdx)]) -> Vec<(Rc<Circuit>, CircuitIdx)> {
        state.iter()
            .map(|(a, b)| (Rc::new(a.clone()), *b))
            .collect()
    }
}