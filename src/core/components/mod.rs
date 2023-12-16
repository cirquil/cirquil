pub mod clock_generator;
pub mod logic;

#[macro_export]
macro_rules! declare_component {
    (
        $(#[derive($($derive:meta),*)])?
        $pub:vis struct $name:ident {
            $($fpub:vis $field:ident : $type:ty),*
        }
    ) => {
        use crate::core::property::Property;
        use crate::core::component::Component;
        use crate::core::pin::PinIdx;
        use crate::core::wire::WireIdx;

        $(#[derive($($derive),*)])?
        $pub struct $name {
            pins: Vec<Pin>,
            properties: Vec<Box<dyn Property>>,
            $($fpub $field : $type),*
        }
        impl $name {
            $pub fn new(
                pins: Vec<Pin>,
                properties: Vec<Box<dyn Property>>,
                $($field:$type,)*)
            -> Self {
                Self {
                    pins, properties,
                    $($field,)*
                }
            }
        }

        impl Component for $name {
            fn get_pins(&self) -> &Vec<Pin> {
                &self.pins
            }

            fn get_pin_value(&self, idx: usize) -> Value {
                self.pins[idx].value.get()
            }

            fn set_pin_value(&self, idx: usize, value: Value) {
                self.pins[idx].value.set(value);
            }

            fn set_pin_wire(&self, pin: PinIdx, wire: Option<WireIdx>) {
                self.pins.get(pin).unwrap().wire.set(wire);
            }

            fn get_properties(&self) -> &Vec<Box<dyn Property>> {
                &self.properties
            }

            fn get_property_value(&self, idx: usize) -> String {
                self.properties[idx].get_value()
            }

            fn set_property_value(&self, idx: usize, value: String) {
                self.properties[idx].parse(value.as_str()).unwrap()
            }
        }
    }
}
