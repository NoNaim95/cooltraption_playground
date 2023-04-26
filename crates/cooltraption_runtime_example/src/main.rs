use crate::render_component::Renderer;
use cooltraption_common::events::MutEventPublisher;
use cooltraption_network::client;
use cooltraption_network::network_state::NetworkStateEventHandler;
use cooltraption_network::network_state_handler::NetworkStateHandler;
use cooltraption_network::server::ServerNetworkingEngine;
use cooltraption_simulation::action::{Action, ActionPacket, SpawnBallAction};
use cooltraption_simulation::simulation_state::ComponentIter;
use cooltraption_simulation::stages::physics_stage::Vec2f;
use cooltraption_simulation::*;
use fixed::prelude::ToFixed;
use pipeline_rs::pipes::receive_pipe::*;
use pipeline_rs::pipes::send_pipe::SendPipe;
use pipeline_rs::pipes::transformer_pipe::TransformerPipe;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;

pub mod render_component;

use rand::random;

fn main() {
    //cooltraption_runtime::run();
    std::thread::spawn(|| {
        println!("Launching server...");
        server_example();
    });
    std::thread::sleep(Duration::from_secs(4));
    std::thread::spawn(|| {
        println!("Launching 1 client...");
        query_example();
    });
    println!("Launching 1 headless_simulation...");
    headless_simulation();
}

pub fn server_example() {
    let network_state_handler = NetworkStateHandler::new(3);

    let mut network_state_event_handler = NetworkStateEventHandler::default();
    network_state_event_handler.add_handler(network_state_handler);

    let mut event_publisher = MutEventPublisher::default();
    event_publisher.add_event_handler(network_state_event_handler);

    ServerNetworkingEngine {}.run(5000, event_publisher);
}

pub fn run() {
    let (s, r) = channel::<Vec<Position>>();
    let position_receiver = ReceivePipe::new(move || r.try_recv().ok());
    let mut renderer = Renderer::new(position_receiver.into_try_iter());
    std::thread::spawn(move || {
        renderer.render();
    });

    sleep(Duration::from_millis(40));
    let mut i = 0;
    loop {
        i += 1;
        let mut positions = vec![Position(Vec2f::new(i.to_fixed(), i.to_fixed()))];
        for j in 1..200 {
            positions.push(Position(Vec2f::new(
                (i * j / 8).to_fixed(),
                (i + j * 5).to_fixed(),
            )))
        }
        println!("sending vec with {} positions", positions.len());
        s.send(positions).unwrap();
        sleep(Duration::from_millis(10));
    }
}

pub fn query_example() {
    let (s, r) = channel::<Vec<Position>>();
    let position_receiver = ReceivePipe::new(move || r.try_recv().ok());
    let mut renderer = Renderer::new(position_receiver.into_try_iter());
    std::thread::spawn(move || {
        renderer.render();
    });

    let action_generator = || None;

    let (node_handler, mut event_receiver, node_task, server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");
    let iter = std::iter::from_fn(move || event_receiver.try_receive()).map(|stored_event| {
        match stored_event.network() {
            cooltraption_network::server::node::StoredNetEvent::Message(_, message) => {
                return serde_yaml::from_slice::<ActionPacket>(&message).unwrap();
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
    sim.run(std::iter::from_fn(action_generator), iter);
}

pub fn headless_simulation() {
    let (node_handler, mut event_receiver, node_task, server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");

    let mut i = 1;
    let action_generator = move || {
        if random::<u32>() % 4 == 0 {
            i += 1;
            let pos = Position(Vec2f::new(
                (i * (rand::random::<u32>() % 16)).to_fixed(),
                ((i * 3) % 1000).to_fixed(),
            ));
            let pos2 = Position(Vec2f::new(1000.to_fixed(), 1000.to_fixed()));
            let action;
            if random::<u32>() % 32 == 0 {
                action = Action::OutwardForce(action::OutwardForceAction {
                    position: pos2,
                    strength: 10.to_fixed(),
                });
            } else {
                action = Action::SpawnBall(SpawnBallAction { position: pos });
            }
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
        std::iter::from_fn(|| None)
    );
}
