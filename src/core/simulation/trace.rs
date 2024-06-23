use crate::core::simulation::value::Value;

#[derive(Debug, Clone, Default)]
pub struct Trace {
    pub recorded_samples: u64,
    pub traces: Vec<Vec<Option<Value>>>,
}

impl Trace {
    pub fn add_sample(&mut self, sample: Vec<(usize, Value)>) {
        for (idx, value) in sample {
            self.traces.get_mut(idx).unwrap().push(Some(value));
        }

        self.recorded_samples += 1;
    }

    pub fn add_row(&mut self) -> usize {
        self.traces.push(vec![None; self.recorded_samples as usize]);

        self.traces.len() - 1
    }

    pub fn clear_traces(&mut self) {
        self.traces.iter_mut().for_each(|trace| trace.clear());
        self.recorded_samples = 0;
    }
}