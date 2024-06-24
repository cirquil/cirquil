use std::fs::File;
use std::path::Path;

pub type TimeSeriesRecord = Vec<u32>;

pub struct TimeSeries {
    pub names: Vec<String>,
    pub records: Vec<TimeSeriesRecord>,
}

pub fn save_csv<P>(path: P, series: TimeSeries)
    where P: AsRef<Path>
{
    let file = File::create(path).unwrap();
    let mut writer = csv::Writer::from_writer(file);

    writer.write_record(series.names).unwrap();

    for record in series.records {
        let string_record: Vec<String> = record.into_iter()
            .map(|record| record.to_string())
            .collect();

        writer.write_record(string_record).unwrap();
    }

    writer.flush().unwrap();
}
