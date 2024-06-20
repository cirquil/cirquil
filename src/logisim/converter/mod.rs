use std::collections::HashMap;

use crate::core::canvas::location::Location;
use crate::logisim::converter::component::convert_logisim_component;
use crate::logisim::parser::project::LogisimProject;
use crate::serde::project::{ProjectFile, SavedCircuit, SavedComponent, SavedWire};

pub mod component;

pub fn convert_logisim_project(logisim_project: LogisimProject) -> ProjectFile {
    let mut project_file = ProjectFile {
        top_circuit: logisim_project.top_circuit.name,
        circuits: HashMap::new(),
    };

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
                SavedComponent {
                    location: Location::from(x.loc),
                    component: convert_logisim_component(&x),
                }
            })
            .collect();

        project_file
            .circuits
            .insert(name, SavedCircuit { components, wires });
    }

    project_file
}