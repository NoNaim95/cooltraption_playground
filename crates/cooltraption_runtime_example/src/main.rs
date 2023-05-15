use cooltraption_common::events::EventPublisher;
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::action::{Action, ActionPacket};
use cooltraption_simulation::*;
use directors::SimulationImplDirector;

use std::iter;

use cooltraption_input::input::{InputEvent, InputEventHandler, InputState};

use std::sync::mpsc::{self, SyncSender};

pub mod directors;
pub mod factories;
pub mod render_component;
pub mod sfml_component;

use cooltraption_input::events::Event as CtnInputEvent;

fn main() {
    //let (input_action_sender, input_action_receiver) = mpsc::channel::<Action>();

    //let (state_send, state_recv) = mpsc::sync_channel(5);

    //let it = iter::from_fn(move || state_recv.try_recv().ok());

    //let mut event_publisher = EventPublisher::<CtnInputEvent<InputEvent, InputState>>::default();
    //event_publisher.add_event_handler(factories::create_input_handler(input_action_sender));
    //render_component::run_renderer(it, InputEventHandler::new(event_publisher));
}
