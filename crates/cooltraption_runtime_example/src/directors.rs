use std::sync::mpsc::Sender;

use cooltraption_common::events::EventPublisher;
use cooltraption_simulation::{
    system_sets::{action_set, physics_set},
    IntoSystemConfig, IntoSystemConfigs, Schedule, Event, action::ActionPacket,
};

use super::SimulationImplBuilder;

pub struct SimulationImplDirector {}

impl SimulationImplDirector {
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
}
