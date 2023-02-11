use std::collections::HashMap;
use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::{Duration, Instant};

use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
use bevy_ecs::system::Resource;

use action::{Action, ActionPacket, ActionRequest};
use components::Position;
use simulation_state::SimulationState;
use stages::physics_stage::{self, DeltaTime, PhysicsStage, Vec2f};

pub mod action;
mod components;
pub mod simulation_state;
pub mod stages;

#[derive(Debug, Resource, Clone)]
pub struct Tick(u64);

#[derive(Resource, Clone)]
pub struct Actions(Vec<Action>);

#[derive(Default)]
pub struct SimulationOptions<S: SimulationState> {
    state: S,
}

pub trait Simulation<T: SimulationState> {
    fn step_simulation(&mut self, dt: Duration);
}

#[derive(Default)]
pub struct SimulationImpl<T: SimulationState> {
    simulation_state: T,
    schedule: Schedule,
    current_tick: u64,
    action_table: HashMap<u64, Vec<Action>>,
}

pub struct ActionHandler {
    action_request_receive_channel: Receiver<ActionRequest>,
    action_packet_receive_channel: Receiver<ActionPacket>,
    action_packet_send_channel: Sender<ActionPacket>,
}

impl ActionHandler {
    fn load_action_packets(&self, current_tick: u64) -> Vec<ActionPacket> {
        let mut action_packets = self
            .action_packet_receive_channel
            .iter()
            .collect::<Vec<ActionPacket>>();
        let action_requests = self
            .action_request_receive_channel
            .iter()
            .collect::<Vec<ActionRequest>>();
        for action_request in action_requests {
            let action = match action_request {
                ActionRequest::SpawnBall {
                    requested_position: req_pos,
                } => Action::SpawnBall {
                    pos: Position(Vec2f::new(req_pos.0, req_pos.1)),
                },
            };
            let action_packet = ActionPacket {
                tick_no: current_tick,
                action,
            };
            self.action_packet_send_channel
                .send(action_packet.clone())
                .unwrap();
            action_packets.push(action_packet);
        }
        action_packets
    }
}

impl<T: SimulationState + 'static> SimulationImpl<T> {
    pub fn new(options: SimulationOptions<T>) -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        Self {
            simulation_state: options.state,
            schedule,
            current_tick: 0,
            action_table: HashMap::default(),
        }
    }

    pub fn run(&mut self) {
        let mut start_time = Instant::now();
        let mut frame_time = start_time - Instant::now();

        loop {
            self.step_simulation(frame_time);
            frame_time = Instant::now() - start_time;
            start_time = Instant::now();

            sleep(Duration::from_secs_f64(1.0 / 1000.0));
        }
    }
    pub fn state(&self) -> &T {
        &self.simulation_state
    }
}

impl<T: SimulationState> Simulation<T> for SimulationImpl<T> {
    fn step_simulation(&mut self, dt: Duration) {
        self.simulation_state
            .world_mut()
            .insert_resource(Tick(self.current_tick));

        self.simulation_state
            .world_mut()
            .insert_resource(DeltaTime::from(dt));

        self.simulation_state
            .world_mut()
            .insert_resource(Actions(std::mem::take(
                &mut self.action_table.entry(self.current_tick).or_default(),
            )));

        self.schedule.run(self.simulation_state.world_mut());
    }
}