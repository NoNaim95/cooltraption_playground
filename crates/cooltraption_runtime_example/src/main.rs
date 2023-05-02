use cgmath::Vector2;
use cooltraption_common::events::MutEventPublisher;
use cooltraption_network as networking;
use cooltraption_network::client;
use cooltraption_network::network_state::NetworkStateEventHandler;
use cooltraption_network::network_state_handler::NetworkStateHandler;
use cooltraption_network::server::ServerNetworkingEngine;
use cooltraption_render::world_renderer::world_state::{Id, Scale, Drawable, Rotation};
use cooltraption_render::world_renderer::WorldState;
use cooltraption_simulation::action::{Action, ActionPacket, SpawnBallAction};
use cooltraption_simulation::system_sets::physics_set::{Float, FromNum2, Vec2f};
use cooltraption_simulation::*;

use std::iter;

use std::sync::mpsc::{self, SyncSender};
use std::time::Duration;

pub mod sfml_component;

pub mod render_component;

use rand::random;

fn main() {
    let (state_send, state_recv) = mpsc::sync_channel(5);


    let _server_handle = std::thread::spawn(|| {
        println!("Launching server...");
        server_example();
    });

    std::thread::sleep(Duration::from_secs(1));
    let _headless_sim_handle = std::thread::spawn(|| {
        println!("Launching 1 headless_simulation...");
        headless_simulation();
    });
    let _sim_handle = std::thread::spawn(|| {
        println!("Launching 1 client...");
        run_simulation(state_send);
    });

    let it = iter::from_fn(move||{state_recv.try_recv().ok()});
    render_component::run_renderer(it);
}

pub fn server_example() {
    let network_state_handler = NetworkStateHandler::new(3);

    let mut network_state_event_handler = NetworkStateEventHandler::default();
    network_state_event_handler.add_handler(network_state_handler);

    let mut event_publisher = MutEventPublisher::default();
    event_publisher.add_event_handler(network_state_event_handler);

    ServerNetworkingEngine {}.run(5000, event_publisher);
}

pub fn run_simulation(world_state_sender: SyncSender<WorldState>) {
    let action_generator = move || None;

    let (node_handler, mut event_receiver, _node_task, server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");

    let action_packet_iter =
        std::iter::from_fn(move || event_receiver.try_receive()).map(|stored_event| {
            match stored_event.network() {
                networking::StoredNetEvent::Message(_, message) => {
                    serde_yaml::from_slice::<ActionPacket>(&message).unwrap()
                }
                networking::StoredNetEvent::Disconnected(_) => {
                    panic!("We got disconnected")
                }
                _ => unreachable!(),
            }
        });

    let sim_options = SimulationOptions::new();
    let mut sim = SimulationImpl::new(sim_options);

    sim.add_query_iter_handler(move |comp_iter: QueryIter<(Entity, &Position), ()>| {
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
    });

    sim.add_local_action_handler(move |action_packet| {
        node_handler.network().send(
            server,
            serde_yaml::to_string(action_packet).unwrap().as_bytes(),
        );
    });
    sim.run(iter::from_fn(action_generator), action_packet_iter);
}

pub fn headless_simulation() {
    let (node_handler, _event_receiver, _node_task, server) =
        client::Client::connect("127.0.0.1:5000".parse().unwrap(), Duration::from_secs(3))
            .expect("could not connect from main");

    let _i: u64 = 1;
    let action_generator = move || {
        if random::<u32>() % 4 == 0 {
            let pos = Position(Vec2f::from_num(
                rand::random::<u64>() % 1920,
                rand::random::<u64>() % 1080,
            ));
            let action = if random::<u32>() % 32 == 0 {
                Action::CircularForce(action::CircularForceAction {
                    position: Position(Vec2f::from_num(1920 / 2, 1080 / 2)),
                    strength: Float::from_num(1.5),
                })
            } else {
                Action::SpawnBall(SpawnBallAction { position: pos })
            };
            Some(action)
        } else {
            None
        }
    };
    let mut sim_options = SimulationOptions::new();
    sim_options.state.load_current_tick(Tick(30));
    let mut sim = SimulationImpl::new(sim_options);
    sim.add_local_action_handler(move |action_packet| {
        node_handler.network().send(
            server,
            serde_yaml::to_string(action_packet).unwrap().as_bytes(),
        );
    });
    sim.run(
        std::iter::from_fn(action_generator),
        std::iter::from_fn(|| None),
    );
}
