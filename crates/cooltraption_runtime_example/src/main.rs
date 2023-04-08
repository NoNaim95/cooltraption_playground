use crate::render_component::Renderer;
use cooltraption_network::server::{
    run_event_handler, listen, node, NetEvent, NetworkState, NodeEvent,
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

pub mod render_component;

fn main() {
    //cooltraption_runtime::run();
    //query_example();

    //let event_send_pipe = SendPipe::new(move |event: NodeEvent<()>| {
    //    match event {
    //        NodeEvent::Network(ref net_event) => match net_event {
    //            NetEvent::Connected(_endpoint, _ok) => unreachable!(),
    //            NetEvent::Accepted(endpoint, _id) => {
    //                network_state.connected_clients.insert(*endpoint);
    //            }
    //            NetEvent::Message(endpoint, data) => {
    //                let msg = String::from_utf8_lossy(data);
    //                let msg2 = String::from(msg);
    //                println!("Received Message: {}", msg2);
    //                network_state.sent_messages.push((*endpoint, msg2));
    //            }
    //            NetEvent::Disconnected(endpoint) => {
    //                network_state.connected_clients.remove(endpoint);
    //            }
    //        },
    //        NodeEvent::Signal(ref signal) => match signal {
    //            () => {
    //                println!("Received Greeting Signal")
    //            }
    //        },
    //    }
    //    network_state.current_event = Some(event);
    //    on_network_state(&network_state);
    //});

    let on_network_state = |network_state: &NetworkState|{
        dbg!(network_state);
    };

    let (handler, listener) = node::split::<()>();
    let mut network_state = NetworkState::default();

    let event_send_pipe2 = SendPipe::new(move |event: NodeEvent<()>| {
        dbg!(event);
    });

    listen(&handler, 5000);
    listener.for_each(event_send_pipe2.into_inner());
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
