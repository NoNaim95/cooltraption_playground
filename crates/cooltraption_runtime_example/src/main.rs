use crate::render_component::Renderer;
use cooltraption_common::events::MutEventPublisher;
use cooltraption_network::client;
use cooltraption_network::network_state::NetworkStateEventHandler;
use cooltraption_network::network_state_handler::NetworkStateHandler;
use cooltraption_network::server::ServerNetworkingEngine;
use cooltraption_simulation::action::{Action, ActionPacket, SpawnBallAction};
use cooltraption_simulation::simulation_state::ComponentIter;
use cooltraption_simulation::system_sets::physics_set::{Float, FromNum2, Vec2f};
use cooltraption_simulation::*;

use std::iter;

use std::sync::mpsc::channel;
use std::time::Duration;

pub mod render_component;

use rand::random;

fn main() {
    let server_handle = std::thread::spawn(|| {
        println!("Launching server...");
        server_example();
    });
    std::thread::sleep(Duration::from_secs(1));
    let sim_handle = std::thread::spawn(|| {
        println!("Launching 1 client...");
        query_example();
    });
    println!("Launching 1 headless_simulation...");
    headless_simulation();

    server_handle.join().unwrap();
    sim_handle.join().unwrap();
}

pub fn server_example() {
    let network_state_handler = NetworkStateHandler::new(3);

    let mut network_state_event_handler = NetworkStateEventHandler::default();
    network_state_event_handler.add_handler(network_state_handler);

    let mut event_publisher = MutEventPublisher::default();
    event_publisher.add_event_handler(network_state_event_handler);

    ServerNetworkingEngine {}.run(5000, event_publisher);
}

pub fn query_example() {
    let (s, r) = channel::<Vec<Position>>();
    std::thread::spawn(move || {
        let renderer = Renderer::new(iter::from_fn(move || r.try_recv().ok()));
        renderer.render();
    });

    let action_generator = move || None;

    let (node_handler, mut event_receiver, _node_task, server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");

    let action_packet_iter =
        std::iter::from_fn(move || event_receiver.try_receive()).map(|stored_event| {
            match stored_event.network() {
                cooltraption_network::server::node::StoredNetEvent::Message(_, message) => {
                    serde_yaml::from_slice::<ActionPacket>(&message).unwrap()
                }
                cooltraption_network::server::node::StoredNetEvent::Disconnected(_) => {
                    panic!("We got disconnected")
                }
                _ => unreachable!(),
            }
        });

    let sim_options = SimulationOptions::new();
    let mut sim = SimulationImpl::new(sim_options);

    sim.add_component_handler(move |pos: ComponentIter<Position>| {
        s.send(pos.cloned().collect()).unwrap();
    });

    sim.add_local_action_handler(move |action_packet| {
        node_handler.network().send(
            server,
            serde_yaml::to_string(action_packet).unwrap().as_bytes(),
        );
    });
    sim.run(iter::from_fn(action_generator), action_packet_iter);
}

pub fn headless_simulation() {
    let (node_handler, _event_receiver, _node_task, server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");

    let _i: u64 = 1;
    let action_generator = move || {
        if random::<u32>() % 4 == 0 {
            let pos = Position(Vec2f::from_num(
                rand::random::<u64>() % 1920,
                rand::random::<u64>() % 1080,
            ));
            let action = if random::<u32>() % 32 == 0 {
                Action::CircularForce(action::CircularForceAction {
                    position: Position(Vec2f::from_num(1920 / 2, 1080 / 2)),
                    strength: Float::from_num(1.5),
                })
            } else {
                Action::SpawnBall(SpawnBallAction { position: pos })
            };
            Some(action)
        } else {
            None
        }
    };
    let mut sim_options = SimulationOptions::new();
    sim_options.state.load_current_tick(Tick(30));
    let mut sim = SimulationImpl::new(sim_options);
    sim.add_local_action_handler(move |action_packet| {
        node_handler.network().send(
            server,
            serde_yaml::to_string(action_packet).unwrap().as_bytes(),
        );
    });
    sim.run(
        std::iter::from_fn(action_generator),
        std::iter::from_fn(|| None),
    );
}
