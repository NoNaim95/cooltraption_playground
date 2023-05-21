#![feature(closure_lifetime_binder)]
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::{env, iter};

use cooltraption_common::events::{EventPublisher, MutEventPublisher};
use cooltraption_network::events::MutEvent as CtnNetworkMutEvent;
use cooltraption_network::packets::{ChatMessage, Packet};
use fork::{fork, Fork};

use cooltraption_runtime::configurators::common_configurators::add_renderer;
use cooltraption_runtime::configurators::{
    ConfiguratorOnce, ConfiguratorOncePipeline, ConfiguratorPipeline,
};
use cooltraption_runtime::factories::create_schedule;
use cooltraption_runtime::{Runtime, RuntimeConfigurationBuilder};

pub mod factories;

fn main() {
    server_example();
}

use cooltraption_network::node_event_handler::NetworkStateImpl;
use cooltraption_network::node_event_handler::{NetworkStateEvent, NodeEventHandler};
use cooltraption_network::server::Server;
use cooltraption_network::server::Signal;
use cooltraption_network::{node, NodeEvent, Transport};

fn server_example() {
    let yaml_string = serde_yaml::to_string(&Packet::<()>::ChatMessage(ChatMessage(String::from(
        "Hello, this is a chat message",
    ))))
    .unwrap();
    print!("{}", yaml_string);
    let (node_handler, node_listener) = node::split::<Signal>();
    node_handler
        .network()
        .listen(Transport::FramedTcp, "0.0.0.0:5000")
        .unwrap();
    let server = Server {
        listener: node_listener,
    };
    let mut event_publisher = MutEventPublisher::default();

    event_publisher.add_event_handler(
        |event: &mut CtnNetworkMutEvent<NetworkStateEvent, NetworkStateImpl>| {
            dbg!(event.mut_context().connections());
            match event.mut_payload() {
                NetworkStateEvent::Accepted(connection) => {
                    dbg!(&connection);
                }
                NetworkStateEvent::Message(connection, message) => {
                    dbg!(message);
                }
                _ => (),
            }
        },
    );

    let mut node_event_handler =
        NodeEventHandler::new(NetworkStateImpl::new(node_handler), event_publisher);

    server.run(move |node_event: NodeEvent<'_, Signal>| -> () {
        node_event_handler.handle_node_event(node_event)
    })
}

fn runtime_example() {
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

    configurator_once_pipeline
        .boxed()
        .configure_once(&mut runtime_config_builder);

    Runtime::run(runtime_config_builder.build());
}
