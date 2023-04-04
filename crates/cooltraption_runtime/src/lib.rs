use cooltraption_simulation::simulation_state::SimulationState;
use cooltraption_simulation::{Simulation, SimulationImpl};
use cooltraption_window::instance_renderer::WorldState;
use fbp_rs::components::Component;
use fbp_rs::{IocGeneratorComponent, ProcessorComponent};
use std::sync::Arc;
use std::time::Duration;

trait Runtime {
    fn run();
}

struct RuntimeImpl<T: SimulationState> {
    simulation: Box<dyn ProcessorComponent<I = DeltaTime, O = Arc<T>>>,
    window: Box<dyn IocGeneratorComponent<I = WorldState, O = ()>>,
}

impl<T: SimulationState + Default> Runtime for RuntimeImpl<T> {
    fn run() {
        let simulation: Box<SimulationComponent<T>> =
            Box::new(SimulationComponent(SimulationImpl::default()));

        /*let runtime = RuntimeImpl {
            simulation,
            window: _,
        };
        let start_time = Instant::now();
        self.simulation.process(DeltaTime(Instant::now() - start_time));
        self.window.*/
    }
}

struct DeltaTime(Duration);

struct SimulationComponent<T: SimulationState>(SimulationImpl<T>);

impl<T: SimulationState> Component for SimulationComponent<T> {
    type I = DeltaTime;
    type O = T;
}

impl<T: SimulationState> ProcessorComponent for SimulationComponent<T> {
    fn process(&mut self, input: Self::I) -> Self::O {
        self.0.step_simulation(input.0);
        Arc::new(self.0.state())
    }
}
