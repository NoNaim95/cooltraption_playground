use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::{Duration, Instant};

use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
use bevy_ecs::system::Resource;

use action::{Action, ActionPacket, ActionRequest};
use components::Position;
use simulation_state::SimulationState;
use stages::physics_stage::{self, DeltaTime, PhysicsStage, Vec2f};


use cooltraption_common::events::{MutEvent, Event};

pub mod action;
mod components;
pub mod simulation_state;
pub mod stages;

#[derive(Debug, Resource, Clone)]
pub struct Tick(u64);

#[derive(Resource, Clone)]
pub struct Actions(Vec<Action>);

#[derive(Default)]
pub struct SimulationOptions {
    state: SimulationState,
}

pub trait Simulation {
    fn step_simulation(&mut self, dt: Duration);
    fn register_action_packet(&mut self, action: ActionPacket);
    fn register_tick_event(&mut self, f: impl Fn(&mut Self) + 'static);
    fn register_simulation_ready_event(&mut self, f: impl Fn(&SimulationState) + 'static);
}

#[derive(Default)]
pub struct SimulationImpl {
    simulation_state: SimulationState,
    schedule: Schedule,
    current_tick: u64,
    action_table: HashMap<u64, Vec<Action>>,
    tick_event: MutEvent<Self>,
    simulation_ready_event: Event<SimulationState>,
}

impl SimulationImpl {
    pub fn new(options: SimulationOptions) -> Self {
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
            tick_event: Default::default(),
            simulation_ready_event: Default::default(),
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

impl Simulation for SimulationImpl {
    fn step_simulation(&mut self, dt: Duration) {
        let tick_event = std::mem::take(&mut self.tick_event);
        MutEvent::invoke(&tick_event, self);
        self.tick_event = tick_event;

        self.simulation_state
            .world_mut()
            .insert_resource(Tick(self.current_tick));

        self.simulation_state
            .world_mut()
            .insert_resource(DeltaTime::from(dt));

        self.simulation_state
            .world_mut()
            .insert_resource(Actions(std::mem::take(
                self.action_table.entry(self.current_tick).or_default(),
            )));

        self.schedule.run(self.simulation_state.world_mut());
        self.simulation_ready_event.invoke(&self.simulation_state);
    }
    fn register_action_packet(&mut self, action: ActionPacket) {
    }

    fn register_tick_event(&mut self, f: impl Fn(&mut Self) + 'static) {
        self.tick_event.add_event_handler(Box::new(f));
    }

    fn register_simulation_ready_event(&mut self, f: impl Fn(&SimulationState) + 'static) {
        self.simulation_ready_event.add_event_handler(Box::new(f));
    }
}
