extern crate derive_more;

use std::collections::HashMap;
use std::iter;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub use bevy_ecs::entity::*;
pub use bevy_ecs::prelude::*;
pub use bevy_ecs::query::QueryIter;
pub use bevy_ecs::query::WorldQuery;
pub use bevy_ecs::schedule::Schedule;
pub use bevy_ecs::system::Resource;
pub use bevy_ecs::world::*;

use action::{Action, ActionPacket};
pub use components::{Acceleration, PhysicsBundle, Position, Velocity};
use cooltraption_common::types::TimePoint;
use simulation_state::SimulationState;
use system_sets::physics_set;

use derive_more::{Add, AddAssign, Deref, Div, From, Into, Mul, Sub};
use rsntp::SntpClient;
use serde::{Deserialize, Serialize};

use builders::*;

pub mod action;
pub mod builders;
pub mod components;
pub mod simulation_state;
pub mod system_sets;

#[rustfmt::skip]
#[derive( Debug, Resource, Clone, Default, Eq, Hash, PartialEq, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign,)]
pub struct Tick(pub u64);

#[derive(Resource, Clone, Default)]
pub struct Actions(Vec<Action>);

type BoxedIt<T> = Box<dyn Iterator<Item = T> + Send>;
type BoxedGenerator<T> = Box<dyn FnMut() -> T + Send>;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SimulationPacket {
    ActionPacket(ActionPacket),
    ResetRequest(ResetRequest),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ResetRequest {
    Now,
    AtTime(TimePoint),
}

impl ResetRequest {
    pub fn sleep_until(&self) {
        match self {
            ResetRequest::Now => (),
            ResetRequest::AtTime(time_point) => {
                let client = SntpClient::new();
                let sleep_millis = time_point.millis()
                    - client
                        .synchronize("time.google.com")
                        .unwrap()
                        .datetime()
                        .unix_timestamp()
                        .unwrap()
                        .as_millis();
                sleep(Duration::from_millis(sleep_millis as u64))
            }
        }
    }
}

pub struct SimulationRunConfig {
    actions: BoxedIt<Action>,
    action_packets: BoxedIt<ActionPacket>,
    state_complete_handler: Vec<SimulationStateHandler>,
    local_action_packet_callbacks: Vec<LocalActionPacketHandler>,
    should_reset_generator: BoxedGenerator<Option<ResetRequest>>,
}

impl Default for SimulationRunConfig {
    fn default() -> Self {
        Self {
            actions: Box::new(iter::from_fn(|| None)),
            action_packets: Box::new(iter::from_fn(|| None)),
            state_complete_handler: Default::default(),
            local_action_packet_callbacks: Default::default(),
            should_reset_generator: Box::new(|| None),
        }
    }
}

pub trait Simulation {
    fn step_simulation(&mut self, dt: Duration);
}

#[derive(Default)]
pub struct SimulationImpl {
    simulation_state: SimulationState,
    schedule: Schedule,
    action_table: HashMap<Tick, Vec<Action>>,
}

impl SimulationImpl {
    pub fn new(
        simulation_state: SimulationState,
        schedule: Schedule,
        action_table: HashMap<Tick, Vec<Action>>,
    ) -> Self {
        Self {
            simulation_state,
            schedule,
            action_table,
        }
    }

    pub fn run(&mut self, mut run_options: SimulationRunConfig) -> ! {
        let target_dt_ms = 16;
        let mut start_time = Instant::now();
        let mut root_time = start_time;
        loop {
            let frame_time = Instant::now() - start_time;
            self.handle_actions(
                &mut run_options.actions,
                &mut run_options.action_packets,
                &mut run_options.local_action_packet_callbacks,
            );
            self.step_simulation(frame_time);

            if let Some(reset_request) = (run_options.should_reset_generator)() {
                self.simulation_state.reset();
                self.action_table.clear();
                reset_request.sleep_until();
                root_time = Instant::now();
            }

            for handler in &mut run_options.state_complete_handler {
                handler(&mut self.simulation_state)
            }

            start_time = Instant::now();
            let sleep_target = root_time
                + Duration::from_millis(target_dt_ms)
                    * self.simulation_state.current_tick().0 as u32;
            sleep(sleep_target - Instant::now());
        }
    }

    pub fn state(&self) -> &SimulationState {
        &self.simulation_state
    }

    fn handle_actions(
        &mut self,
        actions: &mut BoxedIt<Action>,
        action_packets: &mut BoxedIt<ActionPacket>,
        local_action_packet_handlers: &mut [LocalActionPacketHandler],
    ) {
        for local_action_packet in actions
            .map(|action| ActionPacket::new(self.simulation_state.current_tick() + Tick(5), action))
        //TODO +30 ticks as buffer for latency
        {
            for handler in local_action_packet_handlers.iter_mut() {
                handler(&local_action_packet);
            }

            let actions_for_tick = self
                .action_table
                .entry(local_action_packet.tick)
                .or_default();
            actions_for_tick.push(local_action_packet.action);
        }
        for action_packet in action_packets {
            let actions_for_tick = self.action_table.entry(action_packet.tick).or_default();
            actions_for_tick.push(action_packet.action);
        }

        let actions_in_table = self
            .action_table
            .entry(self.simulation_state.current_tick())
            .or_default();
        let actions = std::mem::take(actions_in_table);
        self.simulation_state.load_actions(Actions(actions));
    }
}

impl Simulation for SimulationImpl {
    fn step_simulation(&mut self, dt: Duration) {
        self.simulation_state.load_delta_time(dt.into());
        self.schedule.run(self.simulation_state.world_mut());
        self.simulation_state.advance_tick();
    }
}

//fn add_query_iter_handler<WQ: WorldQuery<ReadOnly = WQ>>(
//    &mut self,
//    mut f: impl FnMut(QueryIter<WQ, ()>) + 'static,
//) {
//    self.state_complete_publisher.add_event_handler(
//        move |e: &mut MutEvent<SimulationState>| e.mut_payload().query(|i| f(i)),
//    );
//}
