use std::sync::MutexGuard;

use cooltraption_network::builder::NodeEventHandlerBuilder;
use cooltraption_network::client::*;
use cooltraption_network::network_state::*;
use cooltraption_network::packets::*;
use cooltraption_simulation::action::ActionPacket;

fn main() {
    let handler1 =
        |network_state_event: &NetworkStateEvent<ActionPacket>,
         locked_network_state: &mut MutexGuard<NetworkStateImpl<ActionPacket>>| {
            if let NetworkStateEvent::Accepted(connection) = network_state_event {
                let chat_msg = ChatMessage(String::from("Hello, this is a chat message"));
                locked_network_state.send_packet(Packet::ChatMessage(chat_msg), connection);
            }
            if let NetworkStateEvent::Message(connection, Packet::ChatMessage(msg)) =
                network_state_event
            {
                locked_network_state.send_packet(Packet::ChatMessage(msg.clone()), connection);
            }

            if let NetworkStateEvent::Message(connection, Packet::ClientPacket(action_packet)) =
                network_state_event
            {
                for conn in locked_network_state
                    .connections()
                    .iter()
                    .filter(|c| **c != connection)
                {
                    locked_network_state.send_packet(Packet::ClientPacket(*action_packet), conn);
                }
            }
        };

    let mut builder = NodeEventHandlerBuilder::default();
    builder.add_network_state_event_handler(Box::new(handler1));
    let node_event_handler = builder.build();

    listen("0.0.0.0:5001", node_event_handler);
}
