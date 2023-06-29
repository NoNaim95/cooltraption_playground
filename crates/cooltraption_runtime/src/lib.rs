extern crate derive_builder;

use std::collections::VecDeque;

use smart_default::SmartDefault;

use cooltraption_simulation::builders::{SimulationImplBuilder, SimulationRunOptionsBuilder};

pub mod configurators;
pub mod factories;
mod render_component;

#[derive(SmartDefault)]
pub struct RuntimeConfiguration {
    pub sim_builder: SimulationImplBuilder,
    pub sim_run_options_builder: SimulationRunOptionsBuilder,
    pub tasks: VecDeque<Task>,
    pub last_task: Option<Task>,
}

#[derive(Default)]
pub struct RuntimeConfigurationBuilder {
    runtime_config: RuntimeConfiguration,
}

impl RuntimeConfigurationBuilder {
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

    pub fn simulation_run_options_builder(&mut self) -> &mut SimulationRunOptionsBuilder {
        &mut self.runtime_config.sim_run_options_builder
    }

    pub fn build(self) -> RuntimeConfiguration {
        self.runtime_config
    }
}

pub type Task = Box<dyn FnOnce() + Send + 'static>;

#[derive(Default)]
pub struct Runtime {}

impl Runtime {
    pub fn run(config: RuntimeConfiguration) {
        let mut task_handles = vec![];
        let run_options = config.sim_run_options_builder.build();
        let sim_handle = std::thread::spawn(|| {
            let mut simulation = config.sim_builder.build();
            simulation.run(run_options);
        });
        task_handles.push(sim_handle);
        for task in config.tasks {
            let task_handle = std::thread::spawn(task);
            task_handles.push(task_handle);
        }
        if let Some(last_task) = config.last_task {
            last_task();
        }

        for task in task_handles {
            task.join().unwrap();
        }
    }

    pub fn config() -> RuntimeConfiguration {
        RuntimeConfiguration::default()
    }
}
