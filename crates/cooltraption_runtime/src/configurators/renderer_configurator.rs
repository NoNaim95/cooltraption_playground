use std::iter;
use std::sync::mpsc;

use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::simulation_state::SimulationState;
use cooltraption_common::events::MutEventPublisher;

use crate::RuntimeConfiguration;
use crate::factories;
use crate::render_component;

pub fn add_renderer<'a>(mut runtime_config: RuntimeConfiguration) -> RuntimeConfiguration {
    let (world_state_sender, world_state_receiver) = mpsc::sync_channel::<WorldState>(5);
    let mut sim_state_sender = factories::sim_state_sender(world_state_sender);

    //let mut state_complete_publisher = MutEventPublisher::default();
    //state_complete_publisher
    //    .add_event_handler(move |s: &mut SimulationState| s.query(|i| sim_state_sender(i)));
    //runtime_config.sim_builder = runtime_config
    //    .sim_builder
    //    .state_complete_publisher(state_complete_publisher);



    let world_state_iterator = iter::from_fn(move || world_state_receiver.try_recv().ok());

    runtime_config.last_task = Some(Box::new(move || {
        render_component::run_renderer(world_state_iterator)
    }));
    runtime_config
}
