use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;

use colored::Colorize;

use crate::core::simulation::probe::CanvasProbe;
use crate::core::simulation::trace::Trace;
use crate::core::simulation::value::Value;
use crate::headless::files::{load_circuit, load_probes};
use crate::player::replay::ReplayManager;
use crate::serde::csv::{save_csv, TimeSeries, TimeSeriesRecord};
use crate::serde::fs::serialize_to_file;
use crate::serde::replay::ReplayFile;

mod files;

#[derive(Debug)]
pub struct HeadlessArgs {
    pub circuit_path: PathBuf,
    pub workbench_path: PathBuf,
    pub cycles: usize,
    pub replay_path: Option<PathBuf>,
    pub trace_path: Option<PathBuf>,
}

pub fn run_player_headless(args: HeadlessArgs) -> Result<(), Box<dyn Error>> {
    let (top_circuit_idx, circuits) = load_circuit(args.circuit_path)?;
    let (top_circuit, _) = circuits.instantiated_circuits.get(top_circuit_idx).unwrap();

    let probes = load_probes(args.workbench_path, &circuits)?;

    let mut replay_manager = ReplayManager::default();
    let mut trace = Trace::default();

    let probes: Vec<(CanvasProbe, usize)> = probes.into_iter()
        .map(|probe| (probe, trace.add_row()))
        .collect();

    let timer = Instant::now();

    for _ in 0..args.cycles {
        top_circuit.tick();
        top_circuit.propagate_ticked();

        if args.trace_path.is_some() {
            let mut trace_sample = vec![];

            for (CanvasProbe { probe, .. }, trace_idx) in probes.iter() {
                let (current_circuit, _) = circuits.instantiated_circuits.get(probe.circuit).unwrap();
                let value = current_circuit.wires.get(probe.wire).unwrap().value.get();

                trace_sample.push((*trace_idx, value));
            }

            trace.add_sample(trace_sample);
        }

        if args.replay_path.is_some() {
            replay_manager.push_frame(
                circuits.instantiated_circuits.iter()
                    .map(|(circuit, idx)| ((*circuit).as_ref().clone(), *idx))
                    .collect()
            );
        }
    }

    println!("{} {} {} {:?}", "Successfully simulated".green(), args.cycles, "cycles in".green(), timer.elapsed());

    if args.trace_path.is_some() {
        let mut records = vec![];

        for i in 0..trace.recorded_samples {
            let record: TimeSeriesRecord = probes.iter()
                .map(|(_, idx)| *idx)
                .map(|idx| {
                    if let Some(value) = trace.traces[idx][i as usize] {
                        value.get_defined_value()
                    } else {
                        Value::default().get_defined_value()
                    }
                })
                .collect();

            records.push(record);
        }

        let time_series = TimeSeries {
            names: probes.iter().map(|(probe, _)| probe.probe.name.clone()).collect(),
            records,
        };

        save_csv(args.trace_path.as_ref().unwrap(), time_series);

        println!("{} {}", "Successfully saved trace file:".green(), &args.trace_path.as_ref().unwrap().display());
    }

    if args.replay_path.is_some() {
        let replay_file = ReplayFile {
            top_circuit: top_circuit_idx,
            states: replay_manager.replay.clone(),
            canvas_circuits: circuits.canvas_circuits.clone(),
            simulation_tree: circuits.simulation_tree.clone(),
            by_uuid: circuits.by_uuid.clone(),
            parents: circuits.parents.clone(),
        };

        serialize_to_file(&replay_file, args.replay_path.as_ref().unwrap())?;

        println!("{} {}", "Successfully saved replay file:".green(), &args.replay_path.as_ref().unwrap().display());
    }

    Ok(())
}
