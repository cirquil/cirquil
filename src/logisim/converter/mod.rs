use std::collections::HashMap;

use crate::core::canvas::location::Location;
use crate::core::simulation::components::subcircuit::Subcircuit;
use crate::logisim::converter::circuit::collect_circuits;
use crate::logisim::converter::component::convert_logisim_component;
use crate::logisim::parser::project::LogisimProject;
use crate::serde::project::{ProjectFile, SavedComponent, SavedWire};

pub mod component;
pub mod circuit;

pub fn convert_logisim_project(logisim_project: LogisimProject) -> ProjectFile {
    let mut project_file = ProjectFile {
        top_circuit: logisim_project.top_circuit.name.clone(),
        circuits: HashMap::new(),
    };

    collect_circuits(&mut project_file, &logisim_project);

    for circuit in logisim_project.circuits {
        let name = circuit.name;

        let wires: Vec<SavedWire> = circuit
            .wires
            .into_iter()
            .map(|x| SavedWire {
                start: Location::from(x.from),
                end: Location::from(x.to),
            })
            .collect();

        let components: Vec<SavedComponent> = circuit
            .components
            .into_iter()
            .map(|x| {
                let component = if x.lib.is_some() {
                    convert_logisim_component(&x)
                } else {
                    let subcircuit = project_file.circuits.get(x.name.as_str()).unwrap();

                    Subcircuit::from_saved_circuit(subcircuit, x.name.as_str())
                };

                SavedComponent {
                    location: Location::from(x.loc),
                    component,
                }
            })
            .collect();

        let project_circuit = project_file.circuits.get_mut(name.as_str()).unwrap();

        project_circuit.components = components;
        project_circuit.wires = wires;
    }

    project_file
}