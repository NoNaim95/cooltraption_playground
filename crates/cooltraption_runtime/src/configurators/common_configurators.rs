use std::iter;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

use crate::events::Event;
use cooltraption_common::events::{EventPublisher, MutEventPublisher};
use cooltraption_input::events::Event as CtnInputEvent;
use cooltraption_input::input::{InputEvent, InputEventHandler, InputState};
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::action::Action;
use cooltraption_simulation::simulation_state::SimulationState;

use crate::factories;
use crate::factories::create_input_handler;
use crate::render_component;
use crate::RuntimeConfiguration;
use cooltraption_simulation::events::MutEvent as SimMutEvent;

pub fn add_renderer(
    mut runtime_config: RuntimeConfiguration,
    input_action_sender: Sender<Action>,
) -> RuntimeConfiguration {
    let (world_state_sender, world_state_receiver) = mpsc::sync_channel::<WorldState>(20);
    let mut sim_state_sender = factories::sim_state_sender(world_state_sender);

    runtime_config.sim_run_options_builder = runtime_config
        .sim_run_options_builder
        .add_state_complete_handler(move |s: &mut SimMutEvent<SimulationState>| {
            s.mut_payload().query(|i| sim_state_sender(i))
        });

    let world_state_iterator = iter::from_fn(move || world_state_receiver.try_recv().ok());
    let mut input_event_publisher: EventPublisher<CtnInputEvent<InputEvent, InputState>> =
        EventPublisher::default();
    let x = create_input_handler(input_action_sender);
    input_event_publisher.add_event_handler(x);
    let input_event_handler = InputEventHandler::new(input_event_publisher);
    runtime_config.last_task = Some(Box::new(move || {
        render_component::run_renderer(world_state_iterator, input_event_handler)
    }));
    runtime_config
}
