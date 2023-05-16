#![feature(option_get_or_insert_default)]
extern crate derive_more;
#[macro_use]
extern crate derive_builder;

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
use cooltraption_common::events::{EventPublisher, MutEventPublisher, EventHandler, MutEventHandler};
use events::MutEvent;
use simulation_state::SimulationState;
use system_sets::physics_set;

use derive_more::{Add, AddAssign, Deref, Div, From, Into, Mul, Sub};
use serde::{Deserialize, Serialize};

pub mod action;
pub mod components;
pub mod events;
pub mod simulation_state;
pub mod system_sets;
pub use events::Event;

#[derive( Debug, Resource, Clone, Default, Eq, Hash, PartialEq, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign,)]
pub struct Tick(pub u64);

#[derive(Resource, Clone, Default)]
pub struct Actions(Vec<Action>);

type BoxedIt<T> = Box<dyn Iterator<Item = T> + Send>;

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(default)]
pub struct SimulationRunOptions<'a>
{
    actions: BoxedIt<Action>,
    action_packets: BoxedIt<ActionPacket>,
    #[builder(setter(custom))]
    state_complete_publisher: MutEventPublisher<'a, MutEvent<'a, SimulationState>>,
    #[builder(setter(custom))]
    local_action_packet_publisher: EventPublisher<'a, Event<'a, ActionPacket>>,
}

impl<'a> SimulationRunOptionsBuilder<'a> {
    pub fn add_state_complete_handler(mut self, event_handler: impl for<'e> MutEventHandler<MutEvent<'e, SimulationState>> + Send + 'a) -> Self {
        self.state_complete_publisher.get_or_insert_default().add_event_handler(event_handler);
        self
    }

    pub fn add_local_action_packet_handler(mut self, event_handler: impl for<'e> EventHandler<Event<'e, ActionPacket>> + Send + 'a) -> Self {
        self.local_action_packet_publisher.get_or_insert_default().add_event_handler(event_handler);
        self
    }
}

impl<'a> Default for SimulationRunOptions<'a>
{
    fn default() -> Self {
        Self {
            actions: Box::new(iter::from_fn(||None)),
            action_packets: Box::new(iter::from_fn(||None)),
            state_complete_publisher: Default::default(),
            local_action_packet_publisher: Default::default()
        }
    }
}

pub trait Simulation {
    fn step_simulation(&mut self, dt: Duration);
}

#[derive(Default, Builder)]
#[builder(pattern = "owned")]
#[builder(default)]
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

    pub fn run(&mut self, mut run_options: SimulationRunOptions) -> !
    {
        let mut start_time = Instant::now();
        loop {
            let frame_time = Instant::now() - start_time;

            self.handle_actions(
                &mut run_options.actions,
                &mut run_options.action_packets,
                &mut run_options.local_action_packet_publisher,
            );
            self.step_simulation(frame_time);
            run_options.state_complete_publisher
                .publish(&mut MutEvent::new(&mut self.simulation_state, &mut ()));

            start_time = Instant::now();
            sleep(Duration::from_millis(10));
        }
    }

    pub fn state(&self) -> &SimulationState {
        &self.simulation_state
    }

    fn handle_actions(
        &mut self,
        actions: &mut BoxedIt<Action>,
        action_packets: &mut BoxedIt<ActionPacket>,
        local_action_packet_publisher: &mut EventPublisher<Event<ActionPacket>>,
    )
    {
        for local_action_packet in
            actions.map(|action| ActionPacket::new(self.simulation_state.current_tick(), action))
        {
            local_action_packet_publisher.publish(&Event::new(&local_action_packet, &()));
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
