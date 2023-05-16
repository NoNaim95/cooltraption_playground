use std::iter;
use std::sync::mpsc;

use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::simulation_state::SimulationState;
use cooltraption_common::events::MutEventPublisher;

use crate::RuntimeConfiguration;
use cooltraption_simulation::events::MutEvent as SimMutEvent;
use crate::factories;
use crate::render_component;

pub fn add_renderer<'a>(mut runtime_config: RuntimeConfiguration<'a>) -> RuntimeConfiguration<'a> {
    let (world_state_sender, world_state_receiver) = mpsc::sync_channel::<WorldState>(20);
    let mut sim_state_sender = factories::sim_state_sender(world_state_sender);


    runtime_config.sim_run_options_builder =
    runtime_config.sim_run_options_builder.add_state_complete_handler(move |s: &mut SimMutEvent<SimulationState>| s.mut_payload().query(|i| sim_state_sender(i)));


    let world_state_iterator = iter::from_fn(move || world_state_receiver.try_recv().ok());

    runtime_config.last_task = Some(Box::new(move || {
        render_component::run_renderer(world_state_iterator)
    }));
    runtime_config
}

