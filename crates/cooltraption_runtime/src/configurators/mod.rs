use std::iter;
use std::rc::Rc;
use std::sync::mpsc;

use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::simulation_state::SimulationState;
use cooltraption_common::events::MutEventPublisher;

use crate::RuntimeConfiguration;
use crate::factories;
use crate::render_component;

pub mod renderer_configurator;

pub trait Configurator : Send{
    fn configure<'a>(&self, runtime_config: RuntimeConfiguration<'a>) -> RuntimeConfiguration<'a>;
}

#[derive(Default)]
pub struct ConfiguratorPipeline<'a> {
    configurators: Vec<Box<dyn Configurator + 'a>>
}

impl<'a> ConfiguratorPipeline<'a> {
    pub fn add_configurator(&mut self, configurator: impl Configurator + 'a) -> &mut Self{
        self.configurators.push(Box::new(configurator));
        self
    }
}

impl<'a> Configurator for ConfiguratorPipeline<'a> {
    fn configure<'b>(&self, mut runtime_config: RuntimeConfiguration<'b>) -> RuntimeConfiguration<'b> {
        for configurator in &self.configurators {
            runtime_config = configurator.configure(runtime_config);
        }
        runtime_config
    }
}

impl<F> Configurator for F
where
    F: Fn(RuntimeConfiguration) -> RuntimeConfiguration + Send
{
    fn configure<'a>(&self, runtime_config: RuntimeConfiguration<'a>) -> RuntimeConfiguration<'a> {
        self(runtime_config)
    }
}


