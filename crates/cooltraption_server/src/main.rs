use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use cooltraption_common::types::TimePoint;
use cooltraption_network::builder::MessageIoInterfaceBootstrapperBuilder;
use cooltraption_network::network_state::*;
use cooltraption_network::packets::*;
use cooltraption_simulation::ResetRequest;
use cooltraption_simulation::SimulationPacket;

fn main() {
    let handler1 =
        |network_state_event: &NetworkInterfaceEvent<SimulationPacket>,
         concurrent_nw_interface: &mut ConcurrentNetworkInterface<SimulationPacket>| {
            let locked_network_interface = concurrent_nw_interface.lock().unwrap();
            if let NetworkInterfaceEvent::Accepted(connection) = network_state_event {
                let chat_msg = ChatMessage(String::from("Hello, this is a chat message"));
                locked_network_interface.send_packet(Packet::ChatMessage(chat_msg), connection);

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
                for conn in locked_network_interface.connections() {
                    locked_network_interface.send_packet(
                        Packet::ClientPacket(SimulationPacket::ResetRequest(reset_requeset)),
                        &conn,
                    )
                }
            }

            if let NetworkInterfaceEvent::Message(connection, packet) = network_state_event {
                match packet {
                    Packet::ChatMessage(chat_message) => {
                        locked_network_interface
                            .send_packet(Packet::ChatMessage(chat_message.clone()), connection);
                    }
                    Packet::ClientPacket(simulation_packet) => {
                        for conn in locked_network_interface
                            .connections()
                            .iter()
                            .filter(|c| **c != *connection)
                        {
                            locked_network_interface
                                .send_packet(Packet::ClientPacket(*simulation_packet), conn);
                        }
                    }
                }
            }
        };

    let mut builder = MessageIoInterfaceBootstrapperBuilder::<SimulationPacket>::default();

    builder.add_network_interface_event_handler(Box::new(handler1));
    let bootstrapper = builder.build();

    bootstrapper.listen("0.0.0.0:5001");
}
