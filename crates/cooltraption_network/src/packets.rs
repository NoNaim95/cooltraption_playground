use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet<T> {
    ChatMessage(ChatMessage),
    ClientPacket(T),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage(pub String);
