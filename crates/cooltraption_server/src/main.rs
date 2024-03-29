use std::sync::MutexGuard;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use cooltraption_common::types::TimePoint;
use cooltraption_network::builder::NodeEventHandlerBuilder;
use cooltraption_network::client::*;
use cooltraption_network::network_state::*;
use cooltraption_network::packets::*;
use cooltraption_simulation::ResetRequest;
use cooltraption_simulation::SimulationPacket;

fn main() {
    let handler1 =
        |network_state_event: &NetworkStateEvent<SimulationPacket>,
         locked_network_state: &mut MutexGuard<NetworkStateImpl<SimulationPacket>>| {
            if let NetworkStateEvent::Accepted(connection) = network_state_event {
                let chat_msg = ChatMessage(String::from("Hello, this is a chat message"));
                locked_network_state.send_packet(Packet::ChatMessage(chat_msg), connection);

                let now_millis = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                let into_2_sec = now_millis % 2000;
                let time_point = if into_2_sec > 1500 {
                    TimePoint::from_millis(now_millis - into_2_sec + 4000)
                } else {
                    TimePoint::from_millis(now_millis - into_2_sec + 2000)
                };

                let reset_requeset = ResetRequest::AtTime(time_point);
                for conn in locked_network_state.connections() {
                    locked_network_state.send_packet(
                        Packet::ClientPacket(SimulationPacket::ResetRequest(reset_requeset)),
                        conn,
                    )
                }
            }

            if let NetworkStateEvent::Message(connection, packet) = network_state_event {
                match packet {
                    Packet::ChatMessage(chat_message) => {
                        locked_network_state
                            .send_packet(Packet::ChatMessage(chat_message.clone()), connection);
                    }
                    Packet::ClientPacket(simulation_packet) => {
                        for conn in locked_network_state
                            .connections()
                            .iter()
                            .filter(|c| **c != connection)
                        {
                            locked_network_state
                                .send_packet(Packet::ClientPacket(*simulation_packet), conn);
                        }
                    }
                }
            }
        };

    let mut builder = NodeEventHandlerBuilder::default();
    builder.add_network_state_event_handler(Box::new(handler1));
    let node_event_handler = builder.build();

    listen("0.0.0.0:5001", node_event_handler);
}
