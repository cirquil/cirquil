use std::collections::HashMap;

use crate::core::simulation::value::Value;

#[derive(Debug, Clone)]
pub struct Trace {
    pub recorded_samples: u64,
    pub traces: HashMap<String, Vec<Value>>,
}

impl Default for Trace {
    fn default() -> Self {
        Self {
            recorded_samples: 0,
            traces: HashMap::new(),
        }
    }
}

impl Trace {
    pub fn add_sample(&mut self, sample: Vec<(&str, Value)>) {
        for (name, value) in sample {
            self.traces.get_mut(name).unwrap().push(value);
        }
    }

    pub fn add_row(&mut self, name: &str) {
        self.traces.insert(name.to_string(), vec![Value::default(); self.recorded_samples as usize]);
    }

    pub fn remove_row(&mut self, name: &str) {
        self.traces.remove(name);
    }
}