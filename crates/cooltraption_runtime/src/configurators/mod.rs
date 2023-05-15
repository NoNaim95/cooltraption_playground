use std::iter;
use std::sync::mpsc;

use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::simulation_state::SimulationState;
use cooltraption_common::events::MutEventPublisher;

use crate::RuntimeConfiguration;
use crate::factories;
use crate::render_component;

pub mod renderer_configurator;

pub trait Configurator {
    fn configure(&self, runtime_config: RuntimeConfiguration) -> RuntimeConfiguration;
}

struct ConfiguratorPipeline {
    configurators: Vec<Box<dyn Configurator>>
}

impl Configurator for ConfiguratorPipeline {
    fn configure(&self, mut runtime_config: RuntimeConfiguration) -> RuntimeConfiguration {
        for configurator in &self.configurators {
            runtime_config = configurator.configure(runtime_config);
        }
        runtime_config
    }
}

impl<F> Configurator for F
where
    F: Fn(RuntimeConfiguration) -> RuntimeConfiguration
{
    fn configure<'a>(&self, runtime_config: RuntimeConfiguration) -> RuntimeConfiguration {
        self(runtime_config)
    }
}


