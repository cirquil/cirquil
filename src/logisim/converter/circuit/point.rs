use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl<'de> Deserialize<'de> for Point {
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
            let x: i32 = i32::from_str(parts[0]).expect("Failed to parse x");
            let y: i32 = i32::from_str(parts[1]).expect("Failed to parse y");
            Point { x, y }
        })
    }
}
