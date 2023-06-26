use message_io::node::NodeHandler;
use serde::{Deserialize, Serialize};

use crate::{server::Signal, node_event_handler::Connection};

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet<T> {
    ChatMessage(ChatMessage),
    ClientPacket(T)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage(pub String);
