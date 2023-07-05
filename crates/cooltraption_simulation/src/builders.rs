use super::*;

pub type SimulationStateHandler = Box<dyn FnMut(&mut SimulationState) + Send>;
pub type LocalActionPacketHandler = Box<dyn FnMut(&ActionPacket) + Send>;

#[derive(Default)]
pub struct SimulationRunOptionsBuilder {
    run_opts: SimulationRunConfig,
}

impl SimulationRunOptionsBuilder {
    pub fn set_actions(&mut self, actions: BoxedIt<Action>) -> &mut Self {
        self.run_opts.actions = actions;
        self
    }

    pub fn set_action_packets(&mut self, action_packets: BoxedIt<ActionPacket>) -> &mut Self {
        self.run_opts.action_packets = action_packets;
        self
    }

    pub fn set_resetter(&mut self, resetter: BoxedGenerator<Option<ResetRequest>>) -> &mut Self {
        self.run_opts.should_reset_generator = resetter;
        self
    }

    pub fn add_state_complete_callback(&mut self, handler: SimulationStateHandler) -> &mut Self {
        self.run_opts.state_complete_handler.push(handler);
        self
    }

    pub fn add_local_action_packet_callback(
        &mut self,
        handler: LocalActionPacketHandler,
    ) -> &mut Self {
        self.run_opts.local_action_packet_callbacks.push(handler);
        self
    }

    pub fn build(self) -> SimulationRunConfig {
        self.run_opts
    }
}

#[derive(Default)]
pub struct SimulationImplBuilder {
    simulation: SimulationImpl,
}

impl SimulationImplBuilder {
    pub fn schedule(&mut self) -> &mut Schedule {
        &mut self.simulation.schedule
    }

    pub fn set_schedule(&mut self, schedule: Schedule) -> &mut Self {
        self.simulation.schedule = schedule;
        self
    }

    pub fn build(self) -> SimulationImpl {
        self.simulation
    }
}
