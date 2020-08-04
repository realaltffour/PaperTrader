use serde::{Serialize, Deserialize};

use crate::ds::message::message_type::{MessageType};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Message {
    pub message_type: MessageType,
    pub instruction: i64,
    pub argument_count: usize,
    pub data_message_number: usize,
    pub data_message_max: usize,
    pub data: Vec<u8>,
}
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {:#?})", self.message_type, self.instruction, 
               self.argument_count, self.data_message_number, self.data_message_max, self.data)
    }
}
