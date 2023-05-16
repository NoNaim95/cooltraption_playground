#![feature(closure_lifetime_binder)]

use cooltraption_common::events::EventPublisher;
use cooltraption_render::world_renderer::WorldState;
use cooltraption_runtime::configurators::{ConfiguratorPipeline, Configurator};
use cooltraption_simulation::action::{Action, ActionPacket};
use cooltraption_simulation::*;
use directors::SimulationImplDirector;

use std::iter;

use cooltraption_input::input::{InputEvent, InputEventHandler, InputState};

use std::sync::mpsc::{self, SyncSender};

pub mod directors;
pub mod factories;
pub mod render_component;

use cooltraption_input::events::Event as CtnInputEvent;

use cooltraption_runtime::{Runtime, RuntimeConfiguration};

fn main() {
    let mut runtime_config = RuntimeConfiguration::default();
    let mut configurator_pipeline = ConfiguratorPipeline::default();

    let configurator1 = for<'a> |mut rt_config: RuntimeConfiguration<'a>| -> RuntimeConfiguration<'a>{
        rt_config.last_task = Some(Box::new(||{
            println!("This was the last task!");
        }));
        rt_config
    };
    configurator_pipeline.add_configurator(configurator1);

    Runtime::run_pipeline(configurator_pipeline);
}
