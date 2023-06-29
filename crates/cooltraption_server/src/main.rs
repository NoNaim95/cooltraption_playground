use std::sync::MutexGuard;

use cooltraption_network::client::*;
use cooltraption_network::network_state::*;
use cooltraption_network::packets::*;

fn main() {
    let mut network_state_event_handlers: Vec<NetworkStateEventHandler> = vec![];
    let handler1 = |network_state_event: &NetworkStateEvent,
                    locked_network_state: &mut MutexGuard<NetworkStateImpl>| {
        if let NetworkStateEvent::Accepted(connection) = network_state_event {
            let chat_msg = ChatMessage(String::from("Hello, this is a chat message"));
            locked_network_state.send_packet(Packet::ChatMessage(chat_msg), connection);
        }
    };
    network_state_event_handlers.push(Box::new(handler1));
    let (handle, _concurrent_network_state) = listen("0.0.0.0:8765", network_state_event_handlers);
    handle.join().unwrap();
}
