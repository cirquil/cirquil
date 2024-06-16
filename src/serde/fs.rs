use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Write};
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn serialize_to_file<T, P>(obj: &T, path: P) -> Result<(), Error>
    where
        T: Serialize,
        P: AsRef<Path>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, obj)?;
    writer.flush()?;

    Ok(())
}

pub fn deserialize_from_file<T, P>(path: P) -> Result<T, Error>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let deserialized = serde_json::from_reader(reader)?;

    Ok(deserialized)
}
