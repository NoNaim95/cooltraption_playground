use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub use bevy_ecs::entity::*;
pub use bevy_ecs::prelude::*;
pub use bevy_ecs::query::QueryIter;
use bevy_ecs::query::WorldQuery;
pub use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
pub use bevy_ecs::system::Resource;
pub use bevy_ecs::world::*;
use fixed::prelude::ToFixed;

use crate::simulation_state::ComponentIter;
use crate::stages::physics_stage::Vec2f;
use action::{Action, ActionPacket};
pub use components::{Acceleration, PhysicsBundle, Position, Velocity};
use cooltraption_common::events::{Event, MutEvent};
use simulation_state::SimulationState;
use stages::physics_stage::{self, PhysicsStage};

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
    fn add_component_handler<C: WorldQuery<ReadOnly = C>>(
        &mut self,
        f: impl FnMut(ComponentIter<C>) + 'static,
    );
}

#[derive(Default)]
pub struct SimulationImpl<I: Iterator<Item = Action>> {
    simulation_state: SimulationState,
    schedule: Schedule,
    action_queue: I,
    action_table: HashMap<Tick, Vec<Action>>,
    state_complete_event: MutEvent<SimulationState>,
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
            options.state.world_mut().spawn(PhysicsBundle {
                pos: Position::default(),
                vel: Velocity(Vec2f::new(10.to_fixed(), 20.to_fixed())),
                acc: Acceleration::default(),
            });
        }

        Self {
            simulation_state: options.state,
            schedule,
            action_table: HashMap::default(),
            state_complete_event: Default::default(),
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
            sleep(Duration::from_millis(30));
        }
    }

    pub fn state(&self) -> &SimulationState {
        &self.simulation_state
    }
}

impl<I: Iterator<Item = Action>> Simulation for SimulationImpl<I> {
    fn step_simulation(&mut self, dt: Duration) {
        for action in &mut self.action_queue {
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
        self.state_complete_event.invoke(&mut self.simulation_state);
    }

    fn add_component_handler<C: WorldQuery<ReadOnly = C>>(
        &mut self,
        mut f: impl FnMut(ComponentIter<C>) + 'static,
    ) {
        self.state_complete_event
            .add_event_handler(move |s| s.query(|i| f(i)));
    }
}
