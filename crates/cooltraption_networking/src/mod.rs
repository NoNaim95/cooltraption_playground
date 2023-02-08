use crate::simulation::action::{Action, ActionRequest};
use quinn::{Connection, Endpoint};
use tokio::sync::mpsc::{Receiver, Sender};

trait NetworkingModule {
    fn run();
}

struct NetworkingModuleImpl {
    simulation_action_incoming: Sender<Action>,
    simulation_action_outgoing: Receiver<Action>,
    endpoint: Endpoint,
    connection: Connection,
}

impl NetworkingModuleImpl {
    fn new(
        simulation_action_incoming: Sender<Action>,
        simulation_action_outgoing: Receiver<Action>,
    ) -> Self {
        let endpoint = Endpoint::client("127.0.0.1:25556".parse().unwrap())
            .expect("Could not Create endpoint on that location");

        Self {
            simulation_action_incoming,
            simulation_action_outgoing,
            endpoint,
            connection: todo!(),
        }
    }
}
