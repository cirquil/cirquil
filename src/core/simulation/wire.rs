use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use crate::core::simulation::component::ComponentIdx;
use crate::core::simulation::pin::PinIdx;
use crate::core::simulation::value::Value;


pub type WireIdx = usize;

pub struct Wire {
    pub value: Cell<Value>,
    pub connected_components: Vec<(ComponentIdx, PinIdx)>
}

impl Debug for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Wire: {:?}, {:?}", self.value.get(), self.connected_components).as_str())
    }
}