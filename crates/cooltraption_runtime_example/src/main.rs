use std::sync::mpsc::channel;
use std::{env, iter};

use cooltraption_runtime::configurators::common_configurators::{
    add_networking_client, add_renderer,
};
use cooltraption_runtime::configurators::{
    ConfiguratorOnce, ConfiguratorOncePipeline, ConfiguratorPipeline,
};
use cooltraption_runtime::factories::create_schedule;
use cooltraption_runtime::{Runtime, RuntimeConfigurationBuilder};
use cooltraption_simulation::action::Action;
use cooltraption_simulation::ResetRequest;

pub mod factories;

fn main() {
    //logger_env =
    env::set_var(
        "RUST_LOG",
        [
            "cooltraption_simulation=debug",
            "cooltraption_runtime=debug",
            "cooltraption_runtime_example=debug",
            "cooltraption_network=debug",
        ]
        .join(","),
    );

    env_logger::init();
    runtime_example();
}

fn runtime_example() {
    let (input_action_sender, input_action_receiver) = channel::<Action>();
    let (reset_sender, reset_receiver) = channel::<ResetRequest>();

    let mut runtime_config_builder = RuntimeConfigurationBuilder::default();
    let mut configurator_pipeline = ConfiguratorPipeline::default();
    let mut configurator_once_pipeline = ConfiguratorOncePipeline::default();

    let input_action_iter = Box::new(iter::from_fn(move || input_action_receiver.try_recv().ok()));

    let input_action_configurator = move |rt_config: &mut RuntimeConfigurationBuilder| {
        rt_config
            .simulation_run_options_builder()
            .set_actions(input_action_iter);
    };

    let add_schedule_configurator = |rt_config: &mut RuntimeConfigurationBuilder| {
        rt_config
            .simulation_builder()
            .set_schedule(create_schedule());
    };
    let cloned_reset_sender = reset_sender.clone();
    let render_configurator = move |rt_config: &mut RuntimeConfigurationBuilder| {
        add_renderer(
            rt_config,
            input_action_sender.clone(),
            cloned_reset_sender.clone(),
        );
    };

    let reset_setter = move |rt_config: &mut RuntimeConfigurationBuilder| {
        rt_config
            .simulation_run_options_builder()
            .set_resetter(Box::new(move || reset_receiver.try_recv().ok()));
        // TODO Sort for the most distant ResetRequest, so that you don't reset multiple times, if many
        // resets are requested
    };
    configurator_pipeline
        .add_configurator(add_schedule_configurator)
        .add_configurator(render_configurator)
        .add_configurator(move |rt_config: &mut RuntimeConfigurationBuilder| {
            add_networking_client(rt_config, reset_sender.clone())
        });

    configurator_once_pipeline
        .add_configurator_once(configurator_pipeline)
        .add_configurator_once(input_action_configurator)
        .add_configurator_once(reset_setter);

    configurator_once_pipeline
        .boxed()
        .configure_once(&mut runtime_config_builder);

    Runtime::run(runtime_config_builder.build());
}
