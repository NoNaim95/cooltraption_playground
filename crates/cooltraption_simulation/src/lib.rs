extern crate derive_more;
#[macro_use]
extern crate derive_builder;

use std::collections::HashMap;
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
use cooltraption_common::events::{EventPublisher, MutEventPublisher, EventHandler};
use events::MutEvent;
use simulation_state::SimulationState;
use system_sets::physics_set;

use derive_more::{Add, AddAssign, Deref, Div, From, Into, Mul, Sub};
use serde::{Deserialize, Serialize};

pub mod action;
//pub mod builder;
pub mod components;
pub mod simulation_state;
pub mod system_sets;
pub mod events;
pub use events::Event;

#[derive( Debug, Resource, Clone, Default, Eq, Hash, PartialEq, Copy, Serialize, Deserialize, Deref, Add, Mul, Sub, Div, From, Into, AddAssign,)]
pub struct Tick(pub u64);

#[derive(Resource, Clone, Default)]
pub struct Actions(Vec<Action>);

#[derive(Default)]
pub struct SimulationOptions {
    pub state: SimulationState,
}

impl SimulationOptions {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }
}

pub trait Simulation {
    fn step_simulation<I, IP>(
        &mut self,
        dt: Duration,
        action_generator: &mut I,
        action_packet_generator: &mut IP,
    ) where
        I: Iterator<Item = Action>,
        IP: Iterator<Item = ActionPacket>;

    fn add_query_iter_handler<WQ: WorldQuery<ReadOnly = WQ>>(
        &mut self,
        f: impl FnMut(QueryIter<WQ, ()>) + 'static,
    );
    fn add_local_action_handler(&mut self, f: impl for<'e> EventHandler<Event<'e, ActionPacket, ()>> + 'static);
}

#[derive(Default, Builder)]
#[builder(pattern = "owned")]
#[builder(default)]
pub struct SimulationImpl<'a> {
    simulation_state: SimulationState,
    schedule: Schedule,
    action_table: HashMap<Tick, Vec<Action>>,
    state_complete_publisher: MutEventPublisher<'a, MutEvent<'a, SimulationState>>,
    local_action_packet_publisher: EventPublisher<'a, Event<'a, ActionPacket>>,
}

impl<'a> SimulationImpl<'a> {
    pub fn new(
        simulation_state: SimulationState,
        schedule: Schedule,
        action_table: HashMap<Tick, Vec<Action>>,
        state_complete_event: MutEventPublisher<'a, MutEvent<'a, SimulationState>>,
        local_action_packet_event: EventPublisher<'a, Event<'a, ActionPacket>>,
    ) -> Self {

        Self {
            simulation_state,
            schedule,
            action_table,
            state_complete_publisher: state_complete_event,
            local_action_packet_publisher: local_action_packet_event,
        }
    }

    pub fn run<I, IP>(&mut self, mut action_generator: I, mut action_packet_generator: IP) -> !
    where
        I: Iterator<Item = Action>,
        IP: Iterator<Item = ActionPacket>,
    {
        let mut start_time = Instant::now();
        const FPS: u64 = 60;
        loop {
            let frame_time = Instant::now() - start_time;
            self.step_simulation(
                frame_time,
                &mut action_generator,
                &mut action_packet_generator,
            );
            start_time = Instant::now();
            //let max = std::cmp::max(0, (1000 / FPS) - frame_time.as_millis() as u64);
            sleep(Duration::from_millis(10));
        }
    }

    pub fn state(&self) -> &SimulationState {
        &self.simulation_state
    }
}

impl<'a> Simulation for SimulationImpl<'a> {
    fn step_simulation<I, IP>(&mut self, dt: Duration, actions: &mut I, action_packets: &mut IP)
    where
        I: Iterator<Item = Action>,
        IP: Iterator<Item = ActionPacket>,
    {
        for local_action_packet in
            actions.map(|action| ActionPacket::new(self.simulation_state.current_tick(), action))
        {
            self.local_action_packet_publisher.publish(&Event::new(&local_action_packet, &()));
            let actions_for_tick = self.action_table.entry(local_action_packet.tick).or_default();
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
        self.simulation_state.load_delta_time(dt.into());

        self.schedule.run(self.simulation_state.world_mut());
        self.state_complete_publisher
            .publish(&mut MutEvent::new(&mut self.simulation_state, &mut ()) );
        self.simulation_state.advance_tick();
    }

    fn add_query_iter_handler<WQ: WorldQuery<ReadOnly = WQ>>(
        &mut self,
        mut f: impl FnMut(QueryIter<WQ, ()>) + 'static,
    ) {
        self.state_complete_publisher
            .add_event_handler(move |e: &mut MutEvent<SimulationState>| e.mut_payload().query(|i| f(i)));
    }

    fn add_local_action_handler(&mut self, f: impl for<'e> EventHandler<Event<'e, ActionPacket, ()>> + 'static) {
        self.local_action_packet_publisher.add_event_handler(f);
    }
}
