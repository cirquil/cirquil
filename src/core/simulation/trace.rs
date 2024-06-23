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

    // pub fn remove_row(&mut self, trace: Rc<Vec<Value>>) {
    //     let idx = self.traces.iter().position(|x| **x == *trace).unwrap();
    //     self.traces.remove(idx);
    // }
}