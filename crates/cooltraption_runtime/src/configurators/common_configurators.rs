use std::iter;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::MutexGuard;

use cooltraption_input::input::{InputEvent, InputEventHandler, InputState};
use cooltraption_network::builder::NodeEventHandlerBuilder;
use cooltraption_network::network_state::NetworkStateEvent;
use cooltraption_network::network_state::NetworkStateImpl;
use cooltraption_render::world_renderer::interpolator::Drawable;
use cooltraption_simulation::action::Action;
use cooltraption_simulation::action::ActionPacket;
use cooltraption_simulation::simulation_state::SimulationState;

use crate::factories;
use crate::factories::create_input_handler;
use crate::render_component;
use crate::RuntimeConfigurationBuilder;

pub type InputEventCallback = Box<dyn FnMut(&InputEvent, &InputState) + 'static + Send>;

pub fn add_renderer(
    runtime_config_builder: &mut RuntimeConfigurationBuilder,
    input_action_sender: Sender<Action>,
) {
    let (world_state_sender, world_state_receiver) = mpsc::sync_channel::<Vec<Drawable>>(20);
    let mut sim_state_sender = factories::sim_state_sender(world_state_sender);
    runtime_config_builder
        .simulation_run_options_builder()
        .add_state_complete_callback(Box::new(move |s: &mut SimulationState| {
            s.query(|i| sim_state_sender(i))
        }));

    let world_state_iterator = iter::from_fn(move || world_state_receiver.try_recv().ok());

    let input_event_callbacks: Vec<InputEventCallback> =
        vec![Box::new(create_input_handler(input_action_sender))];
    let input_event_handler = InputEventHandler::new(input_event_callbacks);

    runtime_config_builder.set_last_task(Box::new(move || {
        render_component::run_renderer(world_state_iterator, input_event_handler)
    }));
}

pub fn add_networking_client(runtime_config_builder: &mut RuntimeConfigurationBuilder) {
    let mut node_event_handler_builder = NodeEventHandlerBuilder::default();
    let (action_sender, action_receiver) = channel::<ActionPacket>();

    let handler =
        |event: &NetworkStateEvent, locked_network_state: &mut MutexGuard<NetworkStateImpl>| {};

    node_event_handler_builder.add_network_state_event_handler(Box::new(handler))

    //runtime_config_builder.simulation_run_options_builder().set_action_packets()
}
