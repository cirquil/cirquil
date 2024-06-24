use std::path::{Path, PathBuf};

use crate::core::simulation::value::Value;
use crate::player::osc::Oscilloscope;
use crate::serde::csv::{save_csv, TimeSeries, TimeSeriesRecord};

pub fn save_csv_from_oscilloscope<P>(path: P, osc: &Oscilloscope)
    where P: AsRef<Path>
{
    let mut records = vec![];

    for i in 0..osc.trace.recorded_samples {
        let record: TimeSeriesRecord = osc.rows.iter()
            .map(|row| {
                if let Some(value) = osc.trace.traces[row.trace_idx][i as usize] {
                    value.get_defined_value()
                } else {
                    Value::default().get_defined_value()
                }
            })
            .collect();

        records.push(record);
    }

    let time_series = TimeSeries {
        names: osc.rows.iter().map(|row| row.name.clone()).collect(),
        records,
    };

    save_csv(path, time_series);
}

pub fn show_save_csv_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("CSV file", vec!["csv"].as_slice())
        .save_file()
}