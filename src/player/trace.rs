use crate::core::simulation::probe::CanvasProbe;
use crate::player::CirquilPlayerApp;

impl CirquilPlayerApp {
    pub fn collect_probe_values(&mut self) {
        let values = self.probes.iter()
            .map(|CanvasProbe { probe, .. }| {
                let (circuit, _) = self.circuits.instantiated_circuits.get(probe.circuit).unwrap();
                (probe.name.as_str(), circuit.wires.get(probe.wire).unwrap().value.get())
            })
            .collect();

        self.trace.add_sample(values);

        self.trace.recorded_samples += 1;
    }
}