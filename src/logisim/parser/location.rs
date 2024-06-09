use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::core::canvas::location::Location;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct LogisimLocation {
    pub x: i16,
    pub y: i16,
}

impl From<LogisimLocation> for Location {
    fn from(value: LogisimLocation) -> Self {
        Location::new(value.x, value.y)
    }
}

impl<'de> Deserialize<'de> for LogisimLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|s: String| {
            // Разбиваем строку на подстроки, удаляем скобки, разбиваем по запятой
            let parts: Vec<&str> = s
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .collect();

            // Парсим числа
            let x: i16 = i16::from_str(parts[0]).expect("Failed to parse x");
            let y: i16 = i16::from_str(parts[1]).expect("Failed to parse y");
            LogisimLocation { x, y }
        })
    }
}
