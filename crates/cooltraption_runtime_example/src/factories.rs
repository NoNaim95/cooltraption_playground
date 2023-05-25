use cooltraption_runtime::RuntimeConfigurationBuilder;
use cooltraption_simulation::action::{Action, SpawnBallAction};
use std::iter;

fn sometimes_spawn_ball(
    mut rt_config_builder: RuntimeConfigurationBuilder<'_>,
) {
    let mut i = 0;
    let boxed_it = Box::new(iter::from_fn(move || {
        i += 1;
        if i % 10 == 0 {
            return Some(Action::SpawnBall(SpawnBallAction {
                position: Default::default(),
            }));
        }
        None
    }));
    rt_config_builder
        .simulation_run_options_builder()
        .set_actions(boxed_it);
}
