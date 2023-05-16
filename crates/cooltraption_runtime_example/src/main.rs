#![feature(closure_lifetime_binder)]

use cooltraption_common::events::EventPublisher;
use cooltraption_render::world_renderer::WorldState;
use cooltraption_runtime::configurators::common_configurators::add_renderer;
use cooltraption_runtime::configurators::{ConfiguratorPipeline, Configurator};
use cooltraption_runtime::factories::create_schedule;
use cooltraption_simulation::action::{Action, ActionPacket, SpawnBallAction};
use cooltraption_simulation::*;
use directors::SimulationImplDirector;
use rand::random;

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

    let random_action_configurator = for<'a> |mut rt_config: RuntimeConfiguration<'a>| -> RuntimeConfiguration<'a>{
        let mut i = 0;
        rt_config.sim_run_options_builder =
        rt_config.sim_run_options_builder.actions(Box::new(iter::from_fn(move||{
                i += 1;
                if i % 10 == 0{
                    return Some(Action::SpawnBall(SpawnBallAction { position: Default::default()}))
                }
                None
        })));
        rt_config
    };

    let add_schedule_configurator = for<'a> |mut rt_config: RuntimeConfiguration<'a>| -> RuntimeConfiguration<'a>{
        rt_config.sim_builder =
        rt_config.sim_builder.schedule(create_schedule());
        rt_config
    };

    configurator_pipeline
        .add_configurator(random_action_configurator)
        .add_configurator(add_schedule_configurator)
        .add_configurator(add_renderer);

    runtime_config = configurator_pipeline.configure(runtime_config);
    Runtime::run(runtime_config);
}
