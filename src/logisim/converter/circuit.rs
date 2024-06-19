use crate::core::canvas::location::Location;
use crate::core::simulation::pin::Direction;
use crate::logisim::parser::circuit::LogisimCircuit;
use crate::logisim::parser::project::LogisimProject;
use crate::serde::project::{ProjectFile, SavedCircuit, SavedCircuitBounds, SavedCircuitPin};

pub fn collect_circuits(project_file: &mut ProjectFile, logisim_project: &LogisimProject) {
    for LogisimCircuit { name, appear, components, .. } in logisim_project.circuits.iter() {
        let Some(circuit_appearance) = appear else {
            project_file.circuits.insert(
                name.clone(),
                SavedCircuit {
                    components: vec![],
                    wires: vec![],
                    bounds: SavedCircuitBounds {
                        start: Location::new(0, 0),
                        end: Location::new(0, 0),
                    },
                    pins: vec![],
                },
            );

            continue;
        };

        let anchor = Location::new(
            (circuit_appearance.circ_anchor.x + (circuit_appearance.circ_anchor.width / 2)) as i16,
            (circuit_appearance.circ_anchor.y + (circuit_appearance.circ_anchor.height / 2)) as i16,
        );

        let bounds = SavedCircuitBounds {
            start: Location::new(
                circuit_appearance.rect.x as i16,
                circuit_appearance.rect.y as i16,
            ) - anchor,
            end: Location::new(
                (circuit_appearance.rect.x + circuit_appearance.rect.width) as i16,
                (circuit_appearance.rect.y + circuit_appearance.rect.height) as i16,
            ) - anchor,
        };

        let pins = circuit_appearance.circ_ports.iter()
            .map(|port| {
                let location = Location::new(
                    (port.x + (port.width / 2)) as i16,
                    (port.y + (port.height / 2)) as i16,
                ) - anchor;

                let pin_component = components.iter()
                    .find(|comp| comp.loc == port.pin_location)
                    .unwrap();

                SavedCircuitPin {
                    location,
                    label: pin_component.get_param("label").unwrap().to_string(),
                    bit_width: 1,
                    direction: {
                        if let Some("true") = pin_component.get_param("output") {
                            Direction::Output
                        } else {
                            Direction::Input
                        }
                    },
                }
            })
            .collect();

        project_file.circuits.insert(
            name.clone(),
            SavedCircuit {
                components: vec![],
                wires: vec![],
                bounds,
                pins,
            },
        );
    }
}