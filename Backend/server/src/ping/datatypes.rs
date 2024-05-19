use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DualMessage {
    pub message_1: String,
    pub message_2: String,
}
