#![feature(closure_lifetime_binder)]
use std::iter;
use std::sync::mpsc::channel;

use cooltraption_runtime::configurators::common_configurators::add_renderer;
use cooltraption_runtime::configurators::{
    ConfiguratorOnce, ConfiguratorOncePipeline, ConfiguratorPipeline,
};
use cooltraption_runtime::factories::create_schedule;
use cooltraption_runtime::{Runtime, RuntimeConfigurationBuilder};

pub mod factories;

fn main() {
    let (input_action_sender, input_action_receiver) = channel();

    let mut runtime_config_builder = RuntimeConfigurationBuilder::default();
    let mut configurator_pipeline = ConfiguratorPipeline::default();
    let mut configurator_once_pipeline = ConfiguratorOncePipeline::default();

    let input_action_iter = Box::new(iter::from_fn(move || input_action_receiver.try_recv().ok()));

    let input_action_configurator = move |rt_config: &mut RuntimeConfigurationBuilder<'_>| {
        rt_config
            .simulation_run_options_builder()
            .set_actions(input_action_iter);
    };

    let add_schedule_configurator = |rt_config: &mut RuntimeConfigurationBuilder<'_>| {
        rt_config
            .simulation_builder()
            .set_schedule(create_schedule());
    };


    let render_configurator = move |config: &mut RuntimeConfigurationBuilder<'_>| {
        return add_renderer(config, input_action_sender.clone());
    };

    configurator_pipeline.add_configurator(add_schedule_configurator);
    configurator_pipeline.add_configurator(render_configurator);

    configurator_once_pipeline.add_configurator_once(configurator_pipeline);
    configurator_once_pipeline.add_configurator_once(input_action_configurator);

    configurator_once_pipeline.boxed().configure_once(&mut runtime_config_builder);

    Runtime::run(runtime_config_builder.build());
}
