use crate::render_component::Renderer;
use cooltraption_network::client_storage::{ClientStorage, ClientStorageEventHandler};
use cooltraption_network::server::{
    run_event_handler, listen, node, NetEvent, NodeEvent, Signal, ServerNetworkingEngine,
};
use cooltraption_network::*;
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

use base64::encode;

pub mod render_component;

fn main() {
    //cooltraption_runtime::run();
    //query_example();
    let (handler, listener) = node::split();
    listen(&handler, 5000);


    let x = ClientStorageEventHandler::default();

    let server = ServerNetworkingEngine::new();
    server.run(5000, x);

    listener.for_each(server.into_handler());
    println!("dispatch_event_handler");
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
