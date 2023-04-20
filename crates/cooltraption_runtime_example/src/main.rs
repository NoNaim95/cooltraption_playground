use crate::render_component::Renderer;
use cooltraption_common::events::MutEventPublisher;
use cooltraption_network::network_state::NetworkStateEventHandler;
use cooltraption_network::network_state_handler::NetworkStateHandler;
use cooltraption_network::server::ServerNetworkingEngine;
use cooltraption_simulation::action::Action;
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

fn main() {
    //cooltraption_runtime::run();
    //query_example();
    server_example();
}

pub fn server_example() {
    let network_state_handler = NetworkStateHandler::new(2);

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

    let mut i = 1;
    let action_generator = move || {
        if rand::random::<bool>() {
            i += 1;
            let pos = Position(Vec2f::new(i.to_fixed(), (i * 3).to_fixed()));
            let action = Action::SpawnBall { pos };
            Some(action)
        } else {
            None
        }
    };
    let action_recv = ReceivePipe::new(action_generator);
    let sim_options = SimulationOptions::new(action_recv.into_try_iter());
    let mut sim = SimulationImpl::new(sim_options);
    sim.add_component_handler(move |pos: ComponentIter<Position>| {
        let _ = s.send(pos.cloned().collect());
    });

    sim.run();
}
