use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub use bevy_ecs::entity::*;
pub use bevy_ecs::prelude::*;
pub use bevy_ecs::query::QueryIter;
pub use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
pub use bevy_ecs::system::Resource;
pub use bevy_ecs::world::*;
use fixed::prelude::ToFixed;
use stages::action_stage::{self, ActionStage};

use crate::simulation_state::ComponentIter;
use crate::stages::physics_stage::Vec2f;
use action::{Action, ActionPacket};
pub use components::{Acceleration, PhysicsBundle, Position, Velocity};
use cooltraption_common::events::{EventPublisher, MutEventPublisher};
use simulation_state::SimulationState;
use stages::physics_stage::{self, PhysicsStage};

use serde::{Deserialize, Serialize};

pub mod action;
pub mod components;
pub mod simulation_state;
pub mod stages;

#[derive(Debug, Resource, Clone, Default, Eq, Hash, PartialEq, Copy, Serialize, Deserialize)]
pub struct Tick(pub u64);

#[derive(Resource, Clone, Default)]
pub struct Actions(Vec<Action>);

#[derive(Default)]
pub struct SimulationOptions<I, IP>
where
    I: Iterator<Item = Action>,
    IP: Iterator<Item = ActionPacket>,
{
    pub state: SimulationState,
    action_queue: I,
    action_packet_queue: IP,
}

impl<I, IP> SimulationOptions<I, IP>
where
    I: Iterator<Item = Action>,
    IP: Iterator<Item = ActionPacket>,
{
    pub fn new(action_generator: I, action_packet_generator: IP) -> Self {
        Self {
            state: Default::default(),
            action_queue: action_generator,
            action_packet_queue: action_packet_generator,
        }
    }
}

pub trait Simulation {
    fn step_simulation(&mut self, dt: Duration);
    fn add_component_handler<C: Component>(&mut self, f: impl FnMut(ComponentIter<C>) + 'static);
    fn add_local_action_handler(&mut self, f: impl FnMut(&ActionPacket) + 'static);
}

#[derive(Default)]
pub struct SimulationImpl<'a, I, IP>
where
    I: Iterator<Item = Action>,
    IP: Iterator<Item = ActionPacket>,
{
    simulation_state: SimulationState,
    schedule: Schedule,
    action_queue: I,
    action_packet_queue: IP,
    action_table: HashMap<Tick, Vec<Action>>,
    state_complete_event: MutEventPublisher<'a, SimulationState>,
    local_action_packet_event: EventPublisher<'a, ActionPacket>,
}

impl<'a, I, IP> SimulationImpl<'a, I, IP>
where
    I: Iterator<Item = Action>,
    IP: Iterator<Item = ActionPacket>,
{
    pub fn new(mut options: SimulationOptions<I, IP>) -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        schedule.add_stage(
            ActionStage,
            SystemStage::parallel()
                .with_system(action_stage::apply_spawn_ball_action)
                .with_system(action_stage::apply_outward_force_action),
        );

        for i in 0..10 {
            options.state.world_mut().spawn(PhysicsBundle {
                pos: Position::default(),
                vel: Velocity(Vec2f::new((i * 10).to_fixed(), (i * 30).to_fixed())),
                acc: Acceleration::default(),
            });
        }

        Self {
            simulation_state: options.state,
            schedule,
            action_table: HashMap::default(),
            state_complete_event: Default::default(),
            action_queue: options.action_queue,
            action_packet_queue: options.action_packet_queue,
            local_action_packet_event: Default::default(),
        }
    }

    pub fn run(&mut self) {
        let mut start_time = Instant::now();
        const FPS: u64 = 60;
        loop {
            let frame_time = Instant::now() - start_time;
            self.step_simulation(frame_time);
            start_time = Instant::now();
            //let max = std::cmp::max(0, (1000 / FPS) - frame_time.as_millis() as u64);
            sleep(Duration::from_millis(10));
        }
    }

    pub fn state(&self) -> &SimulationState {
        &self.simulation_state
    }
}

impl<'a, I, IP> Simulation for SimulationImpl<'a, I, IP>
where
    I: Iterator<Item = Action>,
    IP: Iterator<Item = ActionPacket>,
{
    fn step_simulation(&mut self, dt: Duration) {
        for action_packet in (&mut self.action_queue)
            .map(|action| ActionPacket::new(self.simulation_state.current_tick(), action))
        {
            self.local_action_packet_event.publish(&action_packet);
            let actions_for_tick = self.action_table.entry(action_packet.tick).or_default();
            actions_for_tick.push(action_packet.action);
        }
        for action_packet in &mut self.action_packet_queue {
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
        self.state_complete_event
            .publish(&mut self.simulation_state);
        let current_tick = self.simulation_state.current_tick();
        self.simulation_state.load_current_tick(Tick(current_tick.0 + 1));
    }

    fn add_component_handler<C: Component>(
        &mut self,
        mut f: impl FnMut(ComponentIter<C>) + 'static,
    ) {
        self.state_complete_event
            .add_event_handler(move |s: &mut SimulationState| s.query(|i| f(i)));
    }

    fn add_local_action_handler(&mut self, f: impl FnMut(&ActionPacket) + 'static) {
        self.local_action_packet_event.add_event_handler(f);
    }
}
