use cooltraption_common::events::{EventPublisher, MutEventPublisher};
use cooltraption_network::client;
use cooltraption_network::network_state::NetworkStateEventHandler;
use cooltraption_network::network_state_handler::NetworkStateHandler;
use cooltraption_network::server::ServerNetworkingEngine;
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::action::{Action, ActionPacket};
use cooltraption_simulation::*;
use directors::SimulationImplDirector;

use std::iter;

use cooltraption_input::input::{InputEvent, InputEventHandler};

use std::sync::mpsc::{self, SyncSender};
use std::time::Duration;

pub mod directors;
pub mod factories;
pub mod render_component;
pub mod sfml_component;

fn main() {
    let (input_action_sender, input_action_receiver) = mpsc::channel::<Action>();
    let local_action_iterator = iter::from_fn(move || input_action_receiver.try_recv().ok());
    let (state_send, state_recv) = mpsc::sync_channel(5);

    let _sim_handle = std::thread::spawn(move || {
        run_simulation(local_action_iterator, iter::from_fn(|| None), state_send);
    });

    let mut event_publisher = EventPublisher::<InputEvent>::default();
    event_publisher.add_event_handler(factories::create_input_handler(input_action_sender));

    let it = iter::from_fn(move || state_recv.try_recv().ok());
    render_component::run_renderer(it, InputEventHandler::new(event_publisher));
}


pub fn server_example() {
    let network_state_handler = NetworkStateHandler::new(3);

    let mut network_state_event_handler = NetworkStateEventHandler::default();
    network_state_event_handler.add_handler(network_state_handler);

    let mut event_publisher = MutEventPublisher::default();
    event_publisher.add_event_handler(network_state_event_handler);

    ServerNetworkingEngine {}.run(5000, event_publisher);
}



pub fn run_simulation<I, IP>(
    local_action_iterator: I,
    action_packet_iterator: IP,
    world_state_sender: SyncSender<WorldState>,
) where
    I: Iterator<Item = Action>,
    IP: Iterator<Item = ActionPacket>,
{
    let mut sim = SimulationImplBuilder::default()
        .schedule(SimulationImplDirector::create_schedule())
        .build()
        .unwrap();

    sim.add_query_iter_handler(factories::sim_state_sender(world_state_sender));

    //sim.add_local_action_handler(move |action_packet| {
    //    node_handler.network().send(
    //        server,
    //        serde_yaml::to_string(action_packet).unwrap().as_bytes(),
    //    );
    //});
    sim.run(local_action_iterator, action_packet_iterator);
}

pub fn headless_simulation<I>(local_action_iterator: I)
where
    I: Iterator<Item = Action>,
{
    let (node_handler, _event_receiver, _node_task, server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");

    let mut sim_options = SimulationOptions::new();
    sim_options.state.load_current_tick(Tick(30));
    let mut sim = SimulationImpl::default();
    sim.add_local_action_handler(move |action_packet| {
        node_handler.network().send(
            server,
            serde_yaml::to_string(action_packet).unwrap().as_bytes(),
        );
    });
    sim.run(local_action_iterator, std::iter::from_fn(|| None));
}
