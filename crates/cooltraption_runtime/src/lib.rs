#![allow(dead_code, unused)]
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::simulation_state::SimulationState;
use smart_default::SmartDefault;
use std::{collections::VecDeque, iter, marker::PhantomData, sync::mpsc};

use cooltraption_simulation::SimulationImplBuilder;

use cooltraption_common::events::EventPublisher;
use cooltraption_common::events::MutEventPublisher;
use cooltraption_input::{
    self,
    input::{InputEvent, InputEventHandler},
};

mod configurators;
mod render_component;
mod factories;
pub mod events;

#[derive(SmartDefault)]
pub struct RuntimeConfiguration<'a> {
    sim_builder: SimulationImplBuilder<'a>,
    tasks: VecDeque<Task>,
    last_task: Option<Task>,
}
pub type Task = Box<dyn FnOnce() + Send + 'static>;

struct Runtime {}

impl<'a> Runtime {
    pub fn run(config: RuntimeConfiguration<'a>) -> ! {
        let simulation = config.sim_builder.build().expect("Correctly built SimBuilder");

        //simulation.run(action_generator, action_packet_generator);
        for task in config.tasks{
            std::thread::spawn(task);
        }
        loop {}
    }
    pub fn config() -> RuntimeConfiguration<'a> {
        RuntimeConfiguration::default()
    }
}

