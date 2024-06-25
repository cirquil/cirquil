use std::collections::HashMap;

use egui::{Color32, ComboBox, RichText, ScrollArea, Ui};
use serde::{Deserialize, Serialize};

use crate::core::compiler::project::InstantiatedCircuits;
use crate::core::simulation::probe::CanvasProbe;
use crate::core::simulation::trace::Trace;
use crate::core::simulation::value::Value;
use crate::gui::value::get_value_color;
use crate::player::csv::{save_csv_from_oscilloscope, show_save_csv_file_dialog};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum TriggerType {
    Rising,
    Falling,

    Both,

    #[default]
    Always,
}

impl From<(usize, usize)> for TriggerType {
    fn from(value: (usize, usize)) -> Self {
        match value {
            (0, 1) => TriggerType::Rising,
            (1, 0) => TriggerType::Falling,
            (_, _) => TriggerType::Both,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OscilloscopeRow {
    pub name: String,
    pub source: String,
    pub repr: (),
    pub trace_idx: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Oscilloscope {
    pub rows: Vec<OscilloscopeRow>,
    pub trace: Trace,
    pub last_row_id: usize,
    pub trigger_type: TriggerType,
    pub trigger_source: String,
    pub trigger_value: usize,
}

impl Oscilloscope {
    pub fn collect_probe_values(&mut self, probes: &[CanvasProbe], circuits: &InstantiatedCircuits) {
        let values: HashMap<String, Value> = probes.iter()
            .map(|CanvasProbe { probe, .. }| {
                let (circuit, _) = circuits.instantiated_circuits.get(probe.circuit).unwrap();
                (probe.name.clone(), circuit.wires.get(probe.wire).unwrap().value.get())
            })
            .collect();

        let trigger_value = values.get(self.trigger_source.as_str())
            .copied()
            .unwrap_or_default();

        let new_trigger_value = trigger_value.get_defined_value() as usize;

        let trigger_event = TriggerType::from((self.trigger_value, new_trigger_value));

        self.trigger_value = new_trigger_value;

        if (trigger_event != self.trigger_type)
            && !(self.trigger_type == TriggerType::Both && (trigger_event == TriggerType::Rising
            || trigger_event == TriggerType::Falling))
            && !(self.trigger_type == TriggerType::Always) {
            return;
        }

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
            if ui.button("Save CSV").clicked() {
                if let Some(path) = show_save_csv_file_dialog() {
                    save_csv_from_oscilloscope(path, osc);
                }

                ui.close_menu();
            }

            ui.separator();

            if ui.button("Clear traces").clicked() {
                osc.trace.clear_traces();

                ui.close_menu();
            }
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
                                    let mut selected = probes.iter().position(|x| *x == row.source).unwrap_or(0);

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

                ui.horizontal(|ui| {
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

                    ui.label("Trigger: ");

                    ComboBox::from_id_source(ui.next_auto_id())
                        .selected_text(format!("{:?}", osc.trigger_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut osc.trigger_type, TriggerType::Rising, "Rising");
                            ui.selectable_value(&mut osc.trigger_type, TriggerType::Falling, "Falling");
                            ui.selectable_value(&mut osc.trigger_type, TriggerType::Both, "Both");
                            ui.selectable_value(&mut osc.trigger_type, TriggerType::Always, "Always");
                        },
                        );

                    if osc.trigger_type != TriggerType::Always {
                        ui.label("Source: ");

                        if !probes.is_empty() {
                            let mut selected = probes.iter().position(|x| *x == osc.trigger_source).unwrap_or(0);

                            ComboBox::from_id_source(ui.next_auto_id())
                                .selected_text(osc.trigger_source.as_str())
                                .show_index(
                                    ui,
                                    &mut selected,
                                    probes.len(),
                                    |i| probes[i],
                                );

                            osc.trigger_source = probes[selected].to_string()
                        } else {
                            ui.label("No probes present");
                        }
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