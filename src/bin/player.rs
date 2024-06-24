#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::path::PathBuf;

use clap::Parser;

use cirquil::headless::{HeadlessArgs, run_player_headless};
use cirquil::player::run_player_gui;

/// Cirquil circuit simulator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CirquilArgs {
    /// Start application in headless mode
    #[arg(long)]
    headless: bool,

    /// Circuit to load
    #[arg(long, required_if_eq("headless", "true"))]
    circuit: Option<PathBuf>,

    /// Workbench to load
    #[arg(long, required_if_eq("headless", "true"))]
    workbench: Option<PathBuf>,

    /// Replay file to save
    #[arg(long, requires = "headless")]
    replay: Option<PathBuf>,

    /// Trace file to save
    #[arg(long, requires = "headless")]
    trace: Option<PathBuf>,

    /// How many cycles to simulate
    #[arg(long, requires = "headless", required_if_eq("headless", "true"))]
    cycles: Option<usize>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CirquilArgs::parse();

    if args.headless {
        run_player_headless(HeadlessArgs {
            circuit_path: args.circuit.unwrap(),
            workbench_path: args.workbench.unwrap(),
            cycles: args.cycles.unwrap(),
            trace_path: args.trace,
            replay_path: args.replay,
        })?;
    } else {
        run_player_gui(args.circuit, args.workbench)?;
    }

    Ok(())
}
