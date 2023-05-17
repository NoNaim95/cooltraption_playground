#![allow(dead_code, unused)]
#![feature(option_get_or_insert_default)]
#[macro_use]
extern crate derive_builder;

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

#[derive(Default)]
pub struct RuntimeConfigurationBuilder<'a> {
    runtime_config: RuntimeConfiguration<'a>
}

impl<'a> RuntimeConfigurationBuilder<'a> {
    pub fn add_task(&mut self, task: Task) -> &mut Self {
        self.runtime_config.tasks.push_back(task);
        self
    }

    pub fn set_last_task(&mut self, task: Task) -> &mut Self {
        if self.runtime_config.last_task.is_some() {
            panic!("Last task on RuntimeConfiguration was set twice !!!");
        }
        self.runtime_config.last_task = Some(task);
        self
    }

    pub fn simulation_builder(&mut self) -> &mut SimulationImplBuilder {
        &mut self.runtime_config.sim_builder
    }

    pub fn simulation_run_options_builder(&mut self) -> &mut SimulationRunOptionsBuilder<'a> {
        &mut self.runtime_config.sim_run_options_builder
    }

    pub fn build(self) -> RuntimeConfiguration<'a> {
        self.runtime_config
    }
}




pub type Task = Box<dyn FnOnce() + Send + 'static>;

#[derive(Default)]
pub struct Runtime {}

impl<'a> Runtime {
    pub fn run(config: RuntimeConfiguration<'static>) -> ! {

        let run_options = config
            .sim_run_options_builder
            .build();
        std::thread::spawn(|| {
        let mut simulation = config
            .sim_builder
            .build();
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
