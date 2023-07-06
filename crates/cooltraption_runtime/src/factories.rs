use cgmath::{Point2, Vector2};
use cooltraption_input::input::{InputEvent, InputState, KeyboardInputEvent};
//use cooltraption_network as networking;
//use cooltraption_network::client;
use cooltraption_render::world_renderer::interpolator::Transform;
use cooltraption_render::world_renderer::interpolator::{Drawable, Id, Scale};
use cooltraption_simulation::ResetRequest;
use cooltraption_simulation::{
    action::{Action, CircularForceAction, SpawnBallAction},
    system_sets::physics_set::{Float, FromNum2, Vec2f},
    Entity, Position, QueryIter,
};
use cooltraption_window::window::winit::event::VirtualKeyCode;
use std::sync::mpsc::{Sender, SyncSender};

use cooltraption_simulation::{
    system_sets::{action_set, physics_set},
    IntoSystemConfig, IntoSystemConfigs, Schedule,
};

use cooltraption_common::overwritechannel::OverwriteChannelReader;
use cooltraption_render::world_renderer::camera::controls::CameraView;

pub fn create_input_handler(
    input_action_sender: Sender<Action>,
    reset_request_sender: Sender<ResetRequest>,
) -> impl for<'a> FnMut(&InputEvent, &InputState) + Send {
    move |input_event: &InputEvent, _input_state: &InputState| {
        if let InputEvent::KeyboardInputEvent(KeyboardInputEvent::KeyPressed(key_code, ..)) =
            input_event
        {
            match key_code {
                VirtualKeyCode::Space => {
                    let circular_force_action = CircularForceAction {
                        position: Position(Vec2f::from_num(100, 100)),
                        strength: Float::from_num(30),
                    };
                    input_action_sender
                        .send(Action::CircularForce(circular_force_action))
                        .unwrap();
                }
                VirtualKeyCode::E => {
                    let spawn_ball_action = SpawnBallAction {
                        position: Position(Vec2f::from_num(0, 0)),
                    };
                    input_action_sender
                        .send(Action::SpawnBall(spawn_ball_action))
                        .unwrap();
                }

                VirtualKeyCode::Back => reset_request_sender.send(ResetRequest::Now).unwrap(),
                _ => (),
            }
        }
    }
}

pub fn create_world_input_handler(
    camera_state: OverwriteChannelReader<CameraView>,
    input_action_sender: Sender<Action>,
) -> impl for<'a> FnMut(&InputEvent, &InputState) {
    move |input_event: &InputEvent, input_state: &InputState| {
        if let InputEvent::KeyboardInputEvent(KeyboardInputEvent::KeyPressed(key_code, ..)) =
            input_event
        {
            if key_code == &VirtualKeyCode::F {
                let state = camera_state.read();

                let window_size = Vector2 {
                    x: input_state.window_size.width as f32,
                    y: input_state.window_size.height as f32,
                };
                let mouse_pos = Point2 {
                    x: input_state.mouse_state.mouse_position().x as f32,
                    y: input_state.mouse_state.mouse_position().y as f32,
                };

                let world_pos = state.world_pos(mouse_pos, window_size);
                //println!("{:?},{:?},{:?}", mouse_pos, window_size, world_pos);

                let spawn_ball_action = SpawnBallAction {
                    position: Position(Vec2f::from_num(world_pos.x, world_pos.y)),
                };
                input_action_sender
                    .send(Action::SpawnBall(spawn_ball_action))
                    .unwrap();
            }
        }
    }
}

pub fn sim_state_sender(
    world_state_sender: SyncSender<Vec<Drawable>>,
) -> impl FnMut(QueryIter<'_, '_, (Entity, &Position), ()>) {
    move |comp_iter: QueryIter<(Entity, &Position), ()>| {
        let mut drawables = vec![];
        for (entity, pos) in comp_iter {
            let rpos = pos.0;
            let pos: Vector2<f32> = Vector2::new(rpos.x.0.to_num(), rpos.y.0.to_num());
            //pos /= 100.0;
            let drawable = Drawable {
                id: Id(entity.index() as u64),
                asset_name: String::from("dude"),
                transform: Transform {
                    position: cooltraption_render::world_renderer::interpolator::Position(pos),
                    scale: Scale(Vector2::new(1.0, 1.0)),
                    rot: Default::default(),
                },
            };
            drawables.push(drawable);
        }
        world_state_sender.send(drawables).unwrap();
    }
}

pub fn create_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_system(physics_set::solve_movement.in_set(physics_set::PhysicsSet::Movement));
    schedule.add_systems(
        (
            action_set::apply_spawn_ball_action,
            action_set::apply_outward_force_action,
            action_set::apply_circular_force_action,
        )
            .chain()
            .before(physics_set::PhysicsSet::Movement),
    );
    schedule
}
