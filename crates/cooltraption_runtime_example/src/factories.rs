use cooltraption_simulation::{
    action::{Action, SpawnBallAction},
    system_sets::physics_set::{FromNum2, Vec2f},
    Position,
};
use rand::random;
use std::iter;

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
