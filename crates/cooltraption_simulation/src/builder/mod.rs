use std::collections::HashMap;

use crate::{
    action::{Action, ActionPacket},
    simulation_state::SimulationState,
    SimulationImpl, Tick,
};
use bevy_ecs::schedule::Schedule;
use cooltraption_common::events::{EventPublisher, MutEventPublisher};

use anyhow::{anyhow, Result};

pub mod directors;

pub trait SimulationBuilder {
    type ConcreteSimulation;
    fn set_action_table(&mut self, )
    fn set_schedule(&mut self, schedule: Schedule);
    fn set_simulation_state(&mut self, simulation_state: SimulationState);
    fn build(self) -> Result<Self::ConcreteSimulation>;
}

struct SimulationBuilderImpl<'a> {
    simulation_state: Option<SimulationState>,
    schedule: Option<Schedule>,
    action_table: Option<HashMap<Tick, Vec<Action>>>,
    state_complete_event: Option<MutEventPublisher<'a, SimulationState>>,
    local_action_packet_event: Option<EventPublisher<'a, ActionPacket>>,
}

impl<'a> SimulationBuilder for SimulationBuilderImpl<'a> {
    type ConcreteSimulation = SimulationImpl<'a>;

    fn set_simulation_state(&mut self, simulation_state: SimulationState) {
        self.simulation_state = Some(simulation_state);
    }

    fn build(self) -> Result<Self::ConcreteSimulation> {
        Ok(Self::ConcreteSimulation::new(
            self.simulation_state
                .ok_or(anyhow!("Please provide simulation_state"))?,
            self.schedule.ok_or(anyhow!("Please provide schedule"))?,
            self.action_table
                .ok_or(anyhow!("Please provide action_table"))?,
            self.state_complete_event
                .ok_or(anyhow!("Please provide state_complete_event"))?,
            self.local_action_packet_event
                .ok_or(anyhow!("Please provide local_action_packet_event"))?,
        ))
    }

    fn set_schedule(&mut self, schedule: Schedule) {
        self.schedule = Some(schedule);
    }
}
