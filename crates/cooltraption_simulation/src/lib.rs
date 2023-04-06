use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::{Duration, Instant};

use bevy_ecs::query::WorldQuery;
pub use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
pub use bevy_ecs::world::*;
pub use bevy_ecs::entity::*;
pub use bevy_ecs::prelude::*;
pub use bevy_ecs::system::Resource;

use action::{Action, ActionPacket, ActionRequest};
pub use components::{PhysicsBundle, Position, Velocity, Acceleration};
use simulation_state::SimulationState;
use stages::physics_stage::{self, DeltaTime, PhysicsStage, Vec2f};

use cooltraption_common::events::{Event, MutEvent};

pub mod action;
pub mod components;
pub mod simulation_state;
pub mod stages;

#[derive(Debug, Resource, Clone, Default, Eq, Hash, PartialEq, Copy)]
pub struct Tick(u64);

#[derive(Resource, Clone, Default)]
pub struct Actions(Vec<Action>);

#[derive(Default)]
pub struct SimulationOptions<I: Iterator<Item = Action>> {
    state: SimulationState,
    action_queue: I,
}

impl<I: Iterator<Item = Action>> SimulationOptions<I> {
    pub fn new(generator: I) -> Self {
        Self {
            state: Default::default(),
            action_queue: generator,
        }
    }
}

pub trait Simulation {
    fn step_simulation(&mut self, dt: Duration);
    fn add_simulation_state_handler(&mut self, f: impl FnMut(&SimulationState) + 'static);
}

#[derive(Default)]
pub struct SimulationImpl<I: Iterator<Item = Action>> {
    simulation_state: SimulationState,
    schedule: Schedule,
    action_queue: I,
    action_table: HashMap<Tick, Vec<Action>>,
    simulation_ready_event: Event<SimulationState>,
    publish_action_packet: Event<ActionPacket>,
}

impl<I: Iterator<Item = Action>> SimulationImpl<I> {
    pub fn new(mut options: SimulationOptions<I>) -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        for _ in 0..10 {
            options
                .state
                .world_mut()
                .spawn(PhysicsBundle {
                    pos: Position::default(),
                    vel: Velocity::default(),
                    acc: Acceleration::default(),
                })
                .id();
        }

        Self {
            simulation_state: options.state,
            schedule,
            action_table: HashMap::default(),
            simulation_ready_event: Default::default(),
            action_queue: options.action_queue,
            publish_action_packet: Default::default(),
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
    pub fn state(&self) -> &SimulationState {
        &self.simulation_state
    }
}

impl<I: Iterator<Item = Action>> Simulation for SimulationImpl<I> {
    fn step_simulation(&mut self, dt: Duration) {
        for action in &mut self.action_queue {
            println!("got action!");
            let action_packet = ActionPacket::new(self.simulation_state.current_tick(), action);
            self.publish_action_packet.invoke(&action_packet);
            let actions_for_tick = self.action_table.entry(action_packet.tick).or_default();
            actions_for_tick.push(action_packet.action);
        }
        let actions_in_table = self
            .action_table
            .entry(self.simulation_state.current_tick())
            .or_default();
        let actions = std::mem::take(actions_in_table);
        self.simulation_state.load_actions(Actions(actions));
        self.simulation_state.load_delta_time(dt.into());

        self.schedule.run(self.simulation_state.world_mut());
        self.simulation_ready_event.invoke(&self.simulation_state);
    }
    fn add_simulation_state_handler(&mut self, f: impl FnMut(&SimulationState) + 'static) {
        self.simulation_ready_event.add_event_handler(Box::new(f));
    }
}
