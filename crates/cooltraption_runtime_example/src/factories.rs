use cgmath::Vector2;
use cooltraption_network as networking;
use cooltraption_common::events::EventHandler;
use cooltraption_input::input::{InputEvent, KeyboardInputEvent};
use cooltraption_network::client;
use cooltraption_render::world_renderer::{WorldState, world_state::{Drawable, Id, Scale, Rotation}};
use cooltraption_simulation::{
    action::{Action, SpawnBallAction, CircularForceAction, ActionPacket},
    system_sets::physics_set::{FromNum2, Vec2f, Float},
    Position, QueryIter, Entity,
};
use cooltraption_window::window::winit::event::VirtualKeyCode;
use rand::random;
use std::{iter, sync::mpsc::{Sender, SyncSender}, time::Duration};

fn randomspawn_action(max_x: i32, max_y: i32) -> Action {
    let (x, y) = (random::<i32>() % max_x, random::<i32>() % max_y);
    Action::SpawnBall(SpawnBallAction {
        position: Position(Vec2f::from_num(x, y)),
    })
}

pub fn sometimes_spawn_action(max_x: i32, max_y: i32, n: i32) -> impl Iterator<Item = Action> {
    let mut i = 0;
    iter::from_fn(move||{
        i += 1;
        if i % n == 0 {
            return Some(randomspawn_action(max_x, max_y))
        }
        None
    })
}

pub fn create_input_handler(input_action_sender: Sender<Action>) -> impl EventHandler<InputEvent> {
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

pub fn sim_state_sender(
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
