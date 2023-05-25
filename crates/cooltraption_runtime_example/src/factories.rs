use cgmath::Vector2;
use cooltraption_common::events::EventHandler;
use cooltraption_input::input::{InputEvent, InputState, KeyboardInputEvent};
//use cooltraption_network as networking;
//use cooltraption_network::client;
use cooltraption_render::world_renderer::{
    world_state::{Drawable, Id, Rotation, Scale},
    WorldState,
};
use cooltraption_simulation::{
    action::{Action, CircularForceAction, SpawnBallAction},
    system_sets::physics_set::{Float, FromNum2, Vec2f},
    Entity, Position, QueryIter,
};
use cooltraption_window::window::winit::event::VirtualKeyCode;
use std::sync::mpsc::{Sender, SyncSender};

use cooltraption_input::events::Event as CtnInputEvent;

pub fn create_input_handler(
    input_action_sender: Sender<Action>,
) -> impl for<'e> EventHandler<CtnInputEvent<'e, InputEvent, InputState>> {
    return move |input_event: &CtnInputEvent<InputEvent, InputState>| {
        if let InputEvent::KeyboardInputEvent(keyboard_input_event) = input_event.payload() {
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
                        let mouse_pos = &input_event.context().mouse_state.mouse_position();
                        let spawn_ball_action = SpawnBallAction {
                            position: Position(Vec2f::from_num(mouse_pos.x, mouse_pos.y)),
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
