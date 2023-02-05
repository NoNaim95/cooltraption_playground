use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

use bevy_ecs::prelude::Query;
use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
use log::debug;
use tokio::sync::mpsc::{Sender, UnboundedSender};
use tokio::time::sleep_until;
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::components::{Drawable, Position};
use crate::render::RenderStage;
use crate::render::{RenderMachine, RenderWorld};
use crate::simulation::simulation_state::{LoadSimulation, SimulationState};
use crate::stages::physics_stage;
use crate::stages::physics_stage::{DeltaTime, PhysicsStage};

pub mod simulation_state;

pub struct SimulationOptions<S: SimulationState, E: Error> {
    pub simulation_loader: Box<dyn LoadSimulation<S, E>>,
    pub state_send: Sender<RenderWorld>,
}

pub trait Simulation<'r> {
    fn load_simulation<T: SimulationState + 'r>(&mut self, simulation: T);
    fn step_simulation(&mut self, dt: Duration);
}

pub struct SimulationImpl<'r> {
    simulation: Box<dyn SimulationState + 'r>,
    schedule: Schedule,
}

impl SimulationImpl<'static> {
    pub fn new<S: SimulationState + 'static, E: Error>(options: SimulationOptions<S, E>) -> Self {
        let simulation = Box::new(
            options
                .simulation_loader
                .load()
                .expect("valid simulation object"),
        );

        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        schedule.add_stage_after(
            PhysicsStage,
            RenderStage,
            SystemStage::parallel().with_system(move |query: Query<(&Position, &Drawable)>| {
                let _ = options.state_send.try_send(RenderWorld::new(query));
            }),
        );

        Self {
            simulation,
            schedule,
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
}

impl<'r> Simulation<'r> for SimulationImpl<'r> {
    fn load_simulation<T: SimulationState + 'r>(&mut self, simulation: T) {
        self.simulation = Box::new(simulation);
    }

    fn step_simulation(&mut self, dt: Duration) {
        self.simulation
            .world_mut()
            .insert_resource(DeltaTime::from(dt));
        self.schedule.run(self.simulation.world_mut());
    }
}
