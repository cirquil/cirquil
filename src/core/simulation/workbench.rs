use uuid::Uuid;

use crate::core::compiler::project::InstantiatedCircuits;
use crate::core::simulation::component::ComponentModel;
use crate::core::simulation::components::subcircuit::Subcircuit;
use crate::core::simulation::pin::Direction;
use crate::core::simulation::probe::{CanvasProbe, Probe};
use crate::core::simulation::wire::WireIdx;
use crate::serde::workbench::{OscilloscopeConfig, OscilloscopeRow, ProbePin, SavedProbe, WorkbenchFile};
use crate::player::osc;


impl CanvasProbe {
    fn from_saved(saved: SavedProbe,
                  circuits: &InstantiatedCircuits)
                  -> Result<CanvasProbe, String> {
        let mut circ_instance = circuits.simulation_tree.get_idx();
        for uuid in saved.subcircuit_path.iter() {
            // Not tested
            let (circ, c_type) = &circuits.instantiated_circuits[circ_instance];
            let &comp_idx = circuits.by_uuid[*c_type]
                .get(uuid)
                .ok_or(format!("Probe {}: UUID {} from subcircuit path is invalid", saved.name, uuid))?;
            let component = &circ.components[comp_idx];
            match &component.model {
                ComponentModel::Subcircuit(subcircuit) => {
                    match subcircuit {
                        Subcircuit::Instantiated(_, sub_instance) => {
                            circ_instance = *sub_instance;
                        }
                        Subcircuit::NotInstantiated(_) => {
                            panic!("Component not instantiated when opening workbench");
                        }
                    }
                }
                _ => { return Err(format!("Probe {}: UUID {} from subcircuit path is not a subcircuit", saved.name, uuid)); }
            }
        }
        if saved.pins.is_empty() {
            return Err(format!("Probe {}: was attached to wire without output pins", saved.name));
        }
        let (circ, circ_type) = &circuits.instantiated_circuits[circ_instance];

        let mut wire: Option<WireIdx> = None;
        for p in saved.pins.iter() {
            let &comp_idx = circuits.by_uuid[*circ_type].get(&p.component)
                .ok_or(format!("Probe {}: UUID {} is invalid", saved.name, p.component))?;
            let pin = circ.components[comp_idx].get_pins().get(p.pin)
                .ok_or(format!("Probe {}: component {} doesn not have pin {}", saved.name, p.component, p.pin))?;
            let cur_wire = pin.wire.get().ok_or(format!("Probe {}: component {} pin {} is not connected to wire", saved.name, p.component, p.pin))?;
            match wire {
                Some(prev) => {
                    if cur_wire != prev {
                        return Err(format!("Probe {}: multiple components were connected to one wire, now they don't", saved.name));
                    }
                }
                None => {
                    wire = Some(cur_wire);
                }
            }
        }

        Ok(CanvasProbe {
            location: saved.location,
            probe: Probe {
                name: saved.name,
                circuit: circ_instance,
                wire: wire.unwrap(),
            },
        })
    }
    fn to_saved(&self, circuits: &InstantiatedCircuits)
                -> SavedProbe {
        let mut subcircuit_path: Vec<Uuid> = Vec::new();
        {
            let mut cur_circ = self.probe.circuit;
            loop {
                match circuits.parents[cur_circ] {
                    None => break,
                    Some((parent, comp)) => {
                        subcircuit_path.push(circuits.instantiated_circuits[parent].0.components[comp].uuid);
                        cur_circ = parent;
                    }
                }
            }
        }
        subcircuit_path.reverse();
        let circult = &circuits.instantiated_circuits[self.probe.circuit].0;
        let pins: Vec<ProbePin> = circult.wires[self.probe.wire].connected_components
            .iter()
            .filter_map(|(comp_idx, pin_idx)| {
                match circult.components[*comp_idx].get_pins()[*pin_idx].direction {
                    Direction::Input => None,
                    Direction::Output => Some(ProbePin {
                        component: circult.components[*comp_idx].uuid,
                        pin: *pin_idx,
                    }),
                    Direction::Inout => Some(ProbePin {
                        component: circult.components[*comp_idx].uuid,
                        pin: *pin_idx,
                    }),
                }
            })
            .collect();
        SavedProbe {
            name: self.probe.name.clone(),
            location: self.location,
            subcircuit_path,
            pins,
        }
    }
}

pub fn from_workbench_file(workbench_file: WorkbenchFile,
                           circuits: &InstantiatedCircuits)
                           -> (Vec<Result<CanvasProbe, String>>,
                               osc::Oscilloscope) {
    let canvas_probes: Vec<Result<CanvasProbe, String>> = workbench_file.probes
        .into_iter()
        .map(|x| CanvasProbe::from_saved(x, circuits))
        .collect();

    let mut osciloscope = osc::Oscilloscope {
        rows: Vec::new(),
        trace: Default::default(),
        last_row_id: workbench_file.oscilloscope_config.last_row_id,
    };
    for i in workbench_file.oscilloscope_config.rows {
        let idx = osciloscope.trace.add_row();
        osciloscope.rows.push(osc::OscilloscopeRow {
            name: i.name,
            source: i.source,
            repr: (),
            trace_idx: idx,
        })
    }

    (canvas_probes, osciloscope)
}

pub fn to_workbench_file(canvas_probes: &[CanvasProbe],
                         oscilloscope: &osc::Oscilloscope,
                         circuits: &InstantiatedCircuits)
                         -> WorkbenchFile {
    let saved_probes: Vec<SavedProbe> = canvas_probes
        .iter()
        .map(|x| CanvasProbe::to_saved(x, circuits))
        .collect();

    let oscilloscope_config = OscilloscopeConfig {
        rows: oscilloscope.rows.iter().map(|x| {
            OscilloscopeRow {
                name: x.name.clone(),
                source: x.source.clone(),
            }
        })
            .collect(),
        last_row_id: oscilloscope.last_row_id,
    };

    WorkbenchFile {
        probes: saved_probes,
        oscilloscope_config,
    }
}
