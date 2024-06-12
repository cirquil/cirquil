use eframe::epaint::Color32;

use crate::core::simulation::value::Value;

pub fn get_value_color(value: Value, bits: u8) -> Color32 {
    let defined_value = value.get_defined_value();
    let error_mask = value.get_error();
    let undefined_mask = value.get_undefined();

    match bits {
        1 => {
            if undefined_mask & 1 != 0 {
                return Color32::DARK_BLUE;
            }
            if error_mask & 1 != 0 {
                return Color32::DARK_RED;
            }
            if defined_value & 1 != 0 {
                return Color32::LIGHT_GREEN;
            }
            return Color32::DARK_GREEN;
        }
        _ => {
            Color32::BLACK
        }
    }
}