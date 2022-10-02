//! The message pool of Loki fuzz engine

use crate::loki_message::LokiMessage;
use anyhow::{anyhow, Result};

/// The message pool struct
#[derive(Debug, Clone)]
pub struct MessagePool {
    /// the messages in the pool
    messages: Vec<LokiMessage>,
}

impl MessagePool {
    /// construct a new message pool
    pub fn new(messages: Vec<LokiMessage>) -> Self {
        Self { messages }
    }

    /// add received messages to the message pool
    pub fn add_msgs_to_pool(&mut self, new_messages: &mut Vec<LokiMessage>) -> Result<bool> {
        if new_messages.is_empty() {
            return Err(anyhow!(
                "The added messages are empty! Cannot add them to the message pool."
            ));
        } else {
            self.messages.append(new_messages);
            return Ok(true);
        }
    }

    /// add one message to the message pool
    pub fn add_msg_to_pool(&mut self, message: LokiMessage) -> Result<bool> {
        self.messages.push(message);
        return Ok(true);
    }

    /// delete the oldest element from the messages pool
    pub fn delete_from_messasge_pool(&mut self) -> Result<bool> {
        if self.messages.is_empty() {
            return Err(anyhow!("The message pool is currently empty."));
        }
        self.messages.remove(0);
        Ok(true)
    }

    /// find the latest message of certain type
    pub fn find_latest_message_with_type(&self, msg_type: String) -> Option<LokiMessage> {
        for mes in self.messages.iter().rev() {
            if mes.get_structure().get_name() == msg_type {
                return Some(mes.clone());
            }
        }
        // cannot find the latest message with certain type
        return None;
    }
}
