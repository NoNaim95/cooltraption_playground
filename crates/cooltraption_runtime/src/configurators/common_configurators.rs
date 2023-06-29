use std::iter;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

use cooltraption_input::input::{InputEvent, InputEventHandler, InputState};
use cooltraption_render::world_renderer::interpolator::Drawable;
use cooltraption_simulation::action::Action;
use cooltraption_simulation::simulation_state::SimulationState;

use crate::factories;
use crate::factories::create_input_handler;
use crate::render_component;
use crate::RuntimeConfiguration;
use crate::RuntimeConfigurationBuilder;

pub fn add_renderer(
    mut runtime_config_builder: &mut RuntimeConfigurationBuilder,
    input_action_sender: Sender<Action>,
) {
    let (world_state_sender, world_state_receiver) = mpsc::sync_channel::<Vec<Drawable>>(20);
    let mut sim_state_sender = factories::sim_state_sender(world_state_sender);

    runtime_config_builder
        .simulation_run_options_builder()
        .state_complete_callbacks()
        .push(Box::new(move |s: &mut SimulationState| {
            s.query(|i| sim_state_sender(i))
        }));

    let world_state_iterator = iter::from_fn(move || world_state_receiver.try_recv().ok());

    let mut input_event_callbacks: Vec<Box<dyn FnMut(&InputEvent, &InputState) + 'static + Send>> = vec![];

    input_event_callbacks.push(Box::new(create_input_handler(input_action_sender)));
    let input_event_handler = InputEventHandler::new(input_event_callbacks);

    runtime_config_builder.set_last_task(Box::new(move || {
        render_component::run_renderer(world_state_iterator, input_event_handler)
    }));
}
