use cgmath::Point2;
use std::iter;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::MutexGuard;

use cooltraption_input::input::{InputEvent, InputEventHandler, InputState};
use cooltraption_network::builder::NodeEventHandlerBuilder;
use cooltraption_network::client::connect;
use cooltraption_network::network_state::NetworkStateEvent;
use cooltraption_network::network_state::NetworkStateImpl;
use cooltraption_network::packets::Packet;
use cooltraption_render::world_renderer::interpolator::Drawable;
use cooltraption_simulation::action::Action;
use cooltraption_simulation::action::ActionPacket;
use cooltraption_simulation::simulation_state::SimulationState;
use cooltraption_simulation::ResetRequest;
use cooltraption_simulation::SimulationPacket;

use crate::factories;
use crate::factories::{create_input_handler, create_world_input_handler};
use crate::render_component;
use crate::RuntimeConfigurationBuilder;

use cooltraption_common::overwritechannel::overwrite_channel;
use cooltraption_render::world_renderer::camera::controls::CameraView;
use log::debug;

pub type InputEventCallback = Box<dyn FnMut(&InputEvent, &InputState) + 'static>;

pub fn add_renderer(
    runtime_config_builder: &mut RuntimeConfigurationBuilder,
    input_action_sender: Sender<Action>,
    reset_sender: Sender<ResetRequest>,
) {
    let (world_state_sender, world_state_receiver) = mpsc::sync_channel::<Vec<Drawable>>(20);
    let mut sim_state_sender = factories::sim_state_sender(world_state_sender);
    runtime_config_builder
        .simulation_run_options_builder()
        .add_state_complete_callback(Box::new(move |s: &mut SimulationState| {
            s.query(|i| sim_state_sender(i))
        }));

    let world_state_iterator = iter::from_fn(move || world_state_receiver.try_recv().ok());


    runtime_config_builder.set_last_task(Box::new(move || {
    let (camera_view_writer, camera_view_reader) = overwrite_channel::<CameraView>(CameraView {
        position: Point2 { x: 0.0, y: 0.0 },
        zoom: 1.0,
    });

    let input_event_callbacks: Vec<InputEventCallback> = vec![
        Box::new(create_input_handler(
            input_action_sender.clone(),
            reset_sender,
        )),
        Box::new(create_world_input_handler(camera_view_reader, input_action_sender)),
    ];

    let input_event_handler = InputEventHandler::new(input_event_callbacks);
        render_component::run_renderer(world_state_iterator, input_event_handler, camera_view_writer)
    }));
}

pub fn add_networking_client(
    runtime_config_builder: &mut RuntimeConfigurationBuilder,
    reset_sender: Sender<ResetRequest>,
) {
    let mut node_event_handler_builder = NodeEventHandlerBuilder::default();
    let (action_sender, action_receiver) = channel::<ActionPacket>();

    let handler =
        move |event: &NetworkStateEvent<SimulationPacket>,
              _locked_network_state: &mut MutexGuard<NetworkStateImpl<SimulationPacket>>| {
            if let NetworkStateEvent::Message(_connection, packet) = event {
                match packet {
                    Packet::ChatMessage(msg) => {
                        debug!("Received Chat Message!: {}", msg.0);
                    }
                    Packet::ClientPacket(simulation_packet) => match simulation_packet {
                        SimulationPacket::ActionPacket(action_packet) => {
                            action_sender.send(*action_packet).unwrap()
                        }
                        SimulationPacket::ResetRequest(reset_request) => {
                            reset_sender.send(*reset_request).unwrap()
                        }
                    },
                }
            }
        };
    node_event_handler_builder.add_network_state_event_handler(Box::new(handler));

    runtime_config_builder
        .simulation_run_options_builder()
        .set_action_packets(Box::new(iter::from_fn(move || {
            action_receiver.try_recv().ok()
        })));

    let node_event_handler = node_event_handler_builder.build();
    let concurrent_network_state = node_event_handler.concurrent_network_state();

    let task = Box::new(|| connect("deni-ismailov.de:5001", node_event_handler));
    runtime_config_builder.add_task(task);

    runtime_config_builder
        .simulation_run_options_builder()
        .add_local_action_packet_callback(Box::new(move |local_action_packet| {
            let locked_network_state = concurrent_network_state.lock().unwrap();
            locked_network_state.send_packet(
                Packet::<SimulationPacket>::ClientPacket(SimulationPacket::ActionPacket(
                    *local_action_packet,
                )),
                locked_network_state.connections()[0],
            )
        }));
}
