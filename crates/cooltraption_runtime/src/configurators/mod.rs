use std::iter;
use std::rc::Rc;
use std::sync::mpsc;

use cooltraption_common::events::MutEventPublisher;
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::simulation_state::SimulationState;

use crate::factories;
use crate::render_component;
use crate::RuntimeConfiguration;
use crate::RuntimeConfigurationBuilder;

pub mod common_configurators;

pub trait Configurator : ConfiguratorOnce {
    fn configure<'a>(&self, runtime_config: RuntimeConfigurationBuilder<'a>) -> RuntimeConfigurationBuilder<'a>;
}

pub trait ConfiguratorOnce {
    fn configure_once<'a>(self, runtime_config: RuntimeConfigurationBuilder<'a>) -> RuntimeConfigurationBuilder<'a>;
}

#[derive(Default)]
pub struct ConfiguratorPipeline<'a> {
    configurators: Vec<Box<dyn Configurator + 'a>>,
}

impl<'a> ConfiguratorPipeline<'a> {
    pub fn add_configurator(&mut self, configurator: impl Configurator + 'a) -> &mut Self {
        self.configurators.push(Box::new(configurator));
        self
    }
}

impl<'a> Configurator for ConfiguratorPipeline<'a> {
    fn configure<'b>(
        &self,
        mut runtime_config: RuntimeConfigurationBuilder<'b>,
    ) -> RuntimeConfigurationBuilder<'b> {
        for configurator in &self.configurators {
            runtime_config = configurator.configure(runtime_config);
        }
        runtime_config
    }
}

impl<'c> ConfiguratorOnce for ConfiguratorPipeline<'c> {
    fn configure_once<'a>(self, runtime_config: RuntimeConfigurationBuilder<'a>) -> RuntimeConfigurationBuilder<'a> {
        self.configure(runtime_config)
    }
}

impl<F> Configurator for F
where
    F: Fn(RuntimeConfigurationBuilder) -> RuntimeConfigurationBuilder,
{
    fn configure<'a>(&self, runtime_config: RuntimeConfigurationBuilder<'a>) -> RuntimeConfigurationBuilder<'a> {
        self(runtime_config)
    }
}

impl<F> ConfiguratorOnce for F
where
    F: FnOnce(RuntimeConfigurationBuilder) -> RuntimeConfigurationBuilder,
{
    fn configure_once<'a>(self, runtime_config: RuntimeConfigurationBuilder<'a>) -> RuntimeConfigurationBuilder<'a> {
        self(runtime_config)
    }
}


