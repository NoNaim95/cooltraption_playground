use std::time::{Duration, Instant};
use cooltraption_window;
use cooltraption_simulation::{*, simulation_state::*};

struct Runtime {}

impl Runtime {
    fn run() {
        let mut sim_interface = SimulationImplInterface::default();
        std::thread::spawn(move ||{
            loop {
                sim_interface.step_sim();

            }
        });
    }
    pub fn run_sim(sim: &mut SimulationImplInterface) {
        loop {
            sim.step_sim();
        }
    }
}

#[derive(Default)]
struct SimulationImplInterface {
    simulation: SimulationImpl<SimulationStateImpl>,
}

impl SimulationImplInterface {
    fn step_sim(&mut self) -> &SimulationStateImpl {
        self.simulation.step_simulation(Duration::from_millis(16));
        self.simulation.state()
    }
}
