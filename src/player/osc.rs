use std::collections::HashMap;

use egui::{Color32, ComboBox, RichText, ScrollArea, Ui};

use crate::core::compiler::project::InstantiatedCircuits;
use crate::core::simulation::probe::CanvasProbe;
use crate::core::simulation::trace::Trace;
use crate::core::simulation::value::Value;
use crate::gui::value::get_value_color;

#[derive(Debug, Clone, Default)]
pub struct OscilloscopeRow {
    name: String,
    source: String,
    repr: (),
    trace_idx: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Oscilloscope {
    pub rows: Vec<OscilloscopeRow>,
    pub trace: Trace,
    pub last_row_id: usize,
}

impl Oscilloscope {
    pub fn collect_probe_values(&mut self, probes: &[CanvasProbe], circuits: &InstantiatedCircuits) {
        let values: HashMap<String, Value> = probes.iter()
            .map(|CanvasProbe { probe, .. }| {
                let (circuit, _) = circuits.instantiated_circuits.get(probe.circuit).unwrap();
                (probe.name.clone(), circuit.wires.get(probe.wire).unwrap().value.get())
            })
            .collect();

        let mut records = vec![];

        for row in self.rows.iter() {
            if let Some(value) = values.get(row.source.as_str()) {
                records.push((row.trace_idx, *value));
            } else {
                records.push((row.trace_idx, Value::default()))
            }
        }

        self.trace.add_sample(records);
    }
}

pub fn draw_osc(ui: &mut Ui, osc: &mut Oscilloscope, probes: &[CanvasProbe]) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            let _ = ui.button("Save CSV");
        });
    });

    ScrollArea::vertical().id_source(ui.next_auto_id()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                let probes: Vec<&str> = probes.iter()
                    .map(|probe| probe.probe.name.as_str())
                    .collect();

                let mut row_to_remove = None;

                for (row_idx, row) in osc.rows.iter_mut().enumerate() {
                    ui.vertical(|ui| {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Name: ");
                                ui.text_edit_singleline(&mut row.name);
                            });
                            ui.horizontal(|ui| {
                                if !probes.is_empty() {
                                    let mut selected = if let Some(selected) = probes.iter().position(|x| *x == row.source) {
                                        selected
                                    } else {
                                        0
                                    };

                                    ui.label("Source: ");

                                    ComboBox::from_id_source(ui.next_auto_id())
                                        .selected_text(row.source.as_str())
                                        .show_index(
                                            ui,
                                            &mut selected,
                                            probes.len(),
                                            |i| probes[i],
                                        );

                                    row.source = probes[selected].to_string()
                                } else {
                                    ui.label("No probes present");
                                }

                                if ui.button("Remove").clicked() {
                                    row_to_remove = Some(row_idx);
                                }
                            });
                        });
                    });
                }

                if let Some(idx) = row_to_remove {
                    osc.rows.remove(idx);
                }

                ui.group(|ui| {
                    if ui.button("+").clicked() {
                        let trace_idx = osc.trace.add_row();

                        osc.rows.push(OscilloscopeRow {
                            name: format!("row_{}", osc.last_row_id),
                            source: "".to_string(),
                            repr: (),
                            trace_idx,
                        });

                        osc.last_row_id += 1;
                    }
                });
            });

            ui.separator();

            ScrollArea::horizontal().id_source(ui.next_auto_id()).stick_to_right(true).show(ui, |ui| {
                ui.vertical(|ui| {
                    for OscilloscopeRow { trace_idx, .. } in osc.rows.iter() {
                        let spacing = 7.0;

                        ui.add_space(spacing);

                        ui.horizontal(|ui| {
                            let trace = osc.trace.traces.get(*trace_idx).unwrap();

                            for value in trace.iter() {
                                let (text, color) = match value {
                                    Some(v) => {
                                        (
                                            v.get_defined_value().to_string(),
                                            get_value_color(*v, 1)
                                        )
                                    }
                                    None => {
                                        (
                                            "-".to_string(),
                                            Color32::BLACK,
                                        )
                                    }
                                };
                                ui.monospace(
                                    RichText::new(text)
                                        .size(25.0)
                                        .color(color)
                                );

                                ui.separator();
                            }
                        });

                        ui.add_space(spacing);

                        ui.separator();
                    }
                });
            });
        });
    });
}