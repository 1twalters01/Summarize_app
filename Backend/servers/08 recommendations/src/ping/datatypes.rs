use serde::{Deserialize, Serialize};

/// An example message struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

/// An example message struct
#[derive(Debug, Serialize)]
pub struct DualMessage {
    pub message_1: String,
    pub message_2: String,
}
