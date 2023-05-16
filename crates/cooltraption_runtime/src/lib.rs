#![allow(dead_code, unused)]
use configurators::Configurator;
use configurators::ConfiguratorPipeline;
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::simulation_state::SimulationState;
use cooltraption_simulation::SimulationRunOptionsBuilder;
use smart_default::SmartDefault;
use std::{collections::VecDeque, iter, marker::PhantomData, sync::mpsc};

use cooltraption_simulation::SimulationImplBuilder;

use cooltraption_common::events::EventPublisher;
use cooltraption_common::events::MutEventPublisher;
use cooltraption_input::{
    self,
    input::{InputEvent, InputEventHandler},
};

pub mod configurators;
pub mod events;
pub mod factories;
mod render_component;

#[derive(SmartDefault)]
pub struct RuntimeConfiguration<'a> {
    pub sim_builder: SimulationImplBuilder,
    pub sim_run_options_builder: SimulationRunOptionsBuilder<'a>,
    pub tasks: VecDeque<Task>,
    pub last_task: Option<Task>,
}
pub type Task = Box<dyn FnOnce() + Send + 'static>;

#[derive(Default)]
pub struct Runtime {}

impl<'a> Runtime {
    pub fn run(config: RuntimeConfiguration<'static>) -> ! {

        let run_options = config
            .sim_run_options_builder
            .build()
            .expect("Correctly built SimOptions");
        std::thread::spawn(|| {
        let mut simulation = config
            .sim_builder
            .build()
            .expect("Correctly built SimBuilder");
            simulation.run(run_options);
        });
        for task in config.tasks {
            std::thread::spawn(task);
        }
        if let Some(last_task) = config.last_task {
            last_task()
        }
        loop {}
    }

    pub fn config() -> RuntimeConfiguration<'a> {
        RuntimeConfiguration::default()
    }
}
