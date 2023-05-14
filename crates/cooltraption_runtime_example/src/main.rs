use cooltraption_common::events::{EventPublisher, MutEventPublisher};
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::action::{Action, ActionPacket};
use cooltraption_simulation::*;
use directors::SimulationImplDirector;

use std::iter;

use cooltraption_input::input::{InputEvent, InputEventHandler, InputState};

use std::sync::mpsc::{self, SyncSender};
use std::time::Duration;

pub mod directors;
pub mod factories;
pub mod render_component;
pub mod sfml_component;

use cooltraption_input::events::Event as CtnInputEvent;

fn main() {
    let (input_action_sender, input_action_receiver) = mpsc::channel::<Action>();
    //let local_action_iterator = iter::from_fn(move || input_action_receiver.try_recv().ok());
    let mut sometimes_it = factories::sometimes_spawn_action(3000, 3000, 10);
    //let local_action_iterator = iter::from_fn(move||{
    //    if let Some(spawn_action) = sometimes_it.next() {
    //        return Some(spawn_action);
    //    }
    //    let outward_force_action = factories::random_outward_force(3000, 3000, 10);
    //    if rand::random::<i32>() % 50 == 0{
    //        return Some(outward_force_action);
    //    }
    //    None
    //});

    let (state_send, state_recv) = mpsc::sync_channel(5);

    let _sim_handle = std::thread::spawn(move || {
        run_simulation(iter::from_fn(||input_action_receiver.try_recv().ok()), iter::from_fn(|| None), state_send);
    });

    let it = iter::from_fn(move || state_recv.try_recv().ok());

    let mut event_publisher = EventPublisher::<CtnInputEvent<InputEvent, InputState>>::default();
    event_publisher.add_event_handler(factories::create_input_handler(input_action_sender));
    render_component::run_renderer(it, InputEventHandler::new(event_publisher));
}

pub fn run_simulation<I, IP>(
    local_action_iterator: I,
    action_packet_iterator: IP,
    world_state_sender: SyncSender<WorldState>,
) where
    I: Iterator<Item = Action>,
    IP: Iterator<Item = ActionPacket>,
{
    let schedule = SimulationImplDirector::create_schedule();
    let mut sim = SimulationImplBuilder::default()
        .schedule(schedule)
        .build()
        .unwrap();

    sim.add_query_iter_handler(factories::sim_state_sender(world_state_sender));

    sim.run(local_action_iterator, action_packet_iterator);
}

pub fn headless_simulation<I>(local_action_iterator: I)
where
    I: Iterator<Item = Action>,
{
    let mut sim_options = SimulationOptions::new();
    sim_options.state.load_current_tick(Tick(30));
    let mut sim = SimulationImpl::default();
    sim.run(local_action_iterator, std::iter::from_fn(|| None));
}
