use crate::core::simulation::component::Component;
use crate::core::simulation::components::clock_generator::ClockGenerator;
use crate::core::simulation::components::input::button::InputButton;
use crate::core::simulation::components::logic::and_gate::AndGate;
use crate::core::simulation::components::logic::not_gate::NotGate;
use crate::core::simulation::components::logic::or_gate::OrGate;
use crate::core::simulation::components::subcircuit::input_pin::InputPin;
use crate::core::simulation::components::subcircuit::output_pin::OutputPin;
use crate::core::simulation::components::tunnel::Tunnel;
use crate::logisim::parser::component::LogisimComponent;

pub fn convert_logisim_component(logisim_component: &LogisimComponent) -> Component {
    debug_assert!(logisim_component.lib.is_some());

    match (
        logisim_component.lib.unwrap(),
        logisim_component.name.as_str(),
    ) {
        (0, "Clock") => ClockGenerator::create(),
        (5, "Button") => InputButton::create(),
        (1, "OR Gate") => OrGate::from_bit_width(1),
        (1, "AND Gate") => AndGate::from_bit_width(1),
        (1, "NOT Gate") => NotGate::from_bit_width(1),
        (0, "Tunnel") => Tunnel::from_name_width(logisim_component.get_param("label").unwrap(), 1),
        (0, "Pin") => {
            let label = logisim_component.get_param("label").unwrap();

            if let Some("true") = logisim_component.get_param("output") {
                OutputPin::create(label)
            } else {
                InputPin::create(label)
            }
        }

        _ => panic!(
            "Unknown component {} from lib {}",
            logisim_component.name,
            logisim_component.lib.unwrap()
        ),
    }
}
