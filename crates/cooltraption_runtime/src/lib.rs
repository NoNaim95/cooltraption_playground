use std::{
    sync::mpsc::{self, channel},
    thread::sleep,
    time::Duration,
};

use cooltraption_simulation::{
    action::Action,
    components::Position,
    simulation_state::SimulationState,
    stages::physics_stage::{Float, Vec2f},
    Simulation, SimulationImpl, SimulationOptions, World,
};
use fixed::traits::FromFixed;
use fixed::traits::ToFixed;
use fixed_macro::fixed;
use pipeline_rs::{
    self,
    pipes::{receive_pipe::ReceivePipe, send_pipe::SendPipe},
};

use cooltraption_simulation::Entity;

mod render_component;
use render_component::*;

pub struct Runtime<I: Iterator<Item = Action>> {
    simulation: SimulationImpl<I>,
}

impl<I: Iterator<Item = Action>> Runtime<I> {}

fn query_positions(world: &mut World) -> Vec<&Position> {
    let mut query = world.query::<&Position>();
    query.iter(world).collect()
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
            positions.push(Position(Vec2f::new((i * j/8).to_fixed(), (i + j*5).to_fixed())))
        }
        println!("sending vec with {} positions",positions.len());
        s.send(positions).unwrap();
        sleep(Duration::from_millis(10));
    }

    panic!();
    let mut counter = 1..;
    let action_recv_pipe_closure = move || {
        let i = counter.next()?;
        let pos = Position(Vec2f::new(i.to_fixed(), (i * 3).to_fixed()));
        let action = Action::SpawnBall { pos };
        std::thread::sleep(Duration::from_millis(300));
        Some(action)
    };
    let mut recv_pipe = ReceivePipe::new(action_recv_pipe_closure);
    let sim_options = SimulationOptions::new(recv_pipe.into_try_iter());
    let mut sim = SimulationImpl::new(sim_options);
    sim.run();
}
