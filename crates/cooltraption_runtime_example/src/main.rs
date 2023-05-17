#![feature(closure_lifetime_binder)]
use std::iter;
use std::sync::mpsc::channel;

use cooltraption_runtime::configurators::common_configurators::add_renderer;
use cooltraption_runtime::configurators::{Configurator, ConfiguratorOnce, ConfiguratorPipeline};
use cooltraption_runtime::factories::create_schedule;
use cooltraption_runtime::{Runtime, RuntimeConfigurationBuilder};

pub mod factories;

fn main() {
    let (input_action_sender, input_action_receiver) = channel();

    let mut runtime_config_builder = RuntimeConfigurationBuilder::default();
    let mut configurator_pipeline = ConfiguratorPipeline::default();

    let input_action_iter = Box::new(iter::from_fn(move || input_action_receiver.try_recv().ok()));
    let input_action_configurator = for<'a> move |mut rt_config: RuntimeConfigurationBuilder<'a>| -> RuntimeConfigurationBuilder<'a>
    {
        rt_config
            .simulation_run_options_builder()
            .set_actions(input_action_iter);
        rt_config
    };

    let add_schedule_configurator =
        for<'a> |mut rt_config: RuntimeConfigurationBuilder<'a>| -> RuntimeConfigurationBuilder<'a> {
            rt_config.simulation_builder().set_schedule(create_schedule());
            rt_config
        };

    configurator_pipeline.add_configurator(add_schedule_configurator);

    let render_configurator =
        for<'a> move |config: RuntimeConfigurationBuilder<'a>| -> RuntimeConfigurationBuilder<'a> {
            return add_renderer(config, input_action_sender.clone());
        };

    configurator_pipeline.add_configurator(render_configurator);

    runtime_config_builder = input_action_configurator.configure_once(runtime_config_builder);
    runtime_config_builder = configurator_pipeline.configure(runtime_config_builder);

    Runtime::run(runtime_config_builder.build());
}
