use cgmath::Vector2;
use cooltraption_common::events::{EventHandler, EventPublisher, MutEventPublisher};
use cooltraption_network as networking;
use cooltraption_network::client;
use cooltraption_network::network_state::NetworkStateEventHandler;
use cooltraption_network::network_state_handler::NetworkStateHandler;
use cooltraption_network::server::ServerNetworkingEngine;
use cooltraption_render::world_renderer::world_state::{Drawable, Id, Rotation, Scale};
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::action::{
    Action, ActionPacket, CircularForceAction, SpawnBallAction,
};
use cooltraption_simulation::system_sets::physics_set::{Float, FromNum2, Vec2f};
use cooltraption_simulation::*;
use cooltraption_window::window::winit::event::VirtualKeyCode;
use directors::SimulationImplDirector;

use std::iter;

use cooltraption_input::input::{InputEvent, InputEventHandler, KeyboardInputEvent};

use std::sync::mpsc::{self, SyncSender, Sender};
use std::time::Duration;

pub mod directors;
pub mod factories;
pub mod render_component;
pub mod sfml_component;

fn main() {
    let (input_action_sender, input_action_receiver) = mpsc::channel::<Action>();
    let local_action_iterator = iter::from_fn(move || input_action_receiver.try_recv().ok());
    let (state_send, state_recv) = mpsc::sync_channel(5);

    let _sim_handle = std::thread::spawn(move || {
        run_simulation(local_action_iterator, iter::from_fn(|| None), state_send);
    });

    let mut event_publisher = EventPublisher::<InputEvent>::default();
    event_publisher.add_event_handler(create_input_handler(input_action_sender));

    let it = iter::from_fn(move || state_recv.try_recv().ok());
    render_component::run_renderer(it, InputEventHandler::new(event_publisher));
}

pub fn create_input_handler(input_action_sender: Sender<Action>) -> impl EventHandler<InputEvent>{
    return move |input_event: &InputEvent| {
        if let InputEvent::KeyboardInputEvent(keyboard_input_event) = input_event {
            if let KeyboardInputEvent::KeyPressed(key_code, ..) = keyboard_input_event {
                match key_code {
                    VirtualKeyCode::Space => {
                        let circular_force_action = CircularForceAction {
                            position: Position(Vec2f::from_num(0, 0)),
                            strength: Float::from_num(30),
                        };
                        input_action_sender
                            .send(Action::CircularForce(circular_force_action))
                            .unwrap();
                    }
                    VirtualKeyCode::E => {
                        let spawn_ball_action = SpawnBallAction {
                            position: Position(Vec2f::from_num(10, 10)),
                        };
                        input_action_sender
                            .send(Action::SpawnBall(spawn_ball_action))
                            .unwrap();
                    }
                    _ => (),
                }
            }
        }
    };
}

pub fn server_example() {
    let network_state_handler = NetworkStateHandler::new(3);

    let mut network_state_event_handler = NetworkStateEventHandler::default();
    network_state_event_handler.add_handler(network_state_handler);

    let mut event_publisher = MutEventPublisher::default();
    event_publisher.add_event_handler(network_state_event_handler);

    ServerNetworkingEngine {}.run(5000, event_publisher);
}

pub fn action_packet_from_server_iter() -> impl Iterator<Item = ActionPacket> {
    let (_node_handler, mut event_receiver, _node_task, _server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");
    std::iter::from_fn(move || event_receiver.try_receive()).map(|stored_event| match stored_event
        .network()
    {
        networking::StoredNetEvent::Message(_, message) => {
            serde_yaml::from_slice::<ActionPacket>(&message).unwrap()
        }
        networking::StoredNetEvent::Disconnected(_) => {
            panic!("We got disconnected")
        }
        _ => unreachable!(),
    })
}

fn sim_state_sender(
    world_state_sender: SyncSender<WorldState>,
) -> impl FnMut(QueryIter<'_, '_, (Entity, &Position), ()>) {
    move |comp_iter: QueryIter<(Entity, &Position), ()>| {
        let mut drawables = vec![];
        for (entity, pos) in comp_iter {
            let rpos = pos.0;
            let mut pos: Vector2<f32> = Vector2::new(rpos.x.0.to_num(), rpos.y.0.to_num());
            pos /= 100.0;
            let drawable = Drawable {
                id: Id(entity.index() as u64),
                position: cooltraption_render::world_renderer::world_state::Position(pos),
                scale: Scale(Vector2::new(1.0, 1.0)),
                asset_name: String::from("dude"),
                rot: Rotation::default(),
            };
            drawables.push(drawable);
        }
        let world_state = WorldState { drawables };
        world_state_sender.send(world_state).unwrap();
    }
}

pub fn run_simulation<I, IP>(
    local_action_iterator: I,
    action_packet_iterator: IP,
    world_state_sender: SyncSender<WorldState>,
) where
    I: Iterator<Item = Action>,
    IP: Iterator<Item = ActionPacket>,
{
    let mut sim = SimulationImplBuilder::default()
        .schedule(SimulationImplDirector::create_schedule())
        .build()
        .unwrap();

    sim.add_query_iter_handler(sim_state_sender(world_state_sender));

    //sim.add_local_action_handler(move |action_packet| {
    //    node_handler.network().send(
    //        server,
    //        serde_yaml::to_string(action_packet).unwrap().as_bytes(),
    //    );
    //});
    sim.run(local_action_iterator, action_packet_iterator);
}

pub fn headless_simulation<I>(local_action_iterator: I)
where
    I: Iterator<Item = Action>,
{
    let (node_handler, _event_receiver, _node_task, server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");

    let mut sim_options = SimulationOptions::new();
    sim_options.state.load_current_tick(Tick(30));
    let mut sim = SimulationImpl::default();
    sim.add_local_action_handler(move |action_packet| {
        node_handler.network().send(
            server,
            serde_yaml::to_string(action_packet).unwrap().as_bytes(),
        );
    });
    sim.run(local_action_iterator, std::iter::from_fn(|| None));
}
