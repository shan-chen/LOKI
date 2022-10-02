//! The interface that user needs to extend and implement for LOKI adaption
use crate::loki_message::LokiMessage;
use serde_json::{Map, Value};

/// the adatption interface
/// encode the LOKI message into a stream
pub fn encode(msg_name: String, content: Map<String, Value>) -> Vec<u8> {
    let msg = crate::loki_message::get_structure_from_msg_type(msg_name).unwrap();
    return crate::encode_map_to_buffer::encode(msg, content);
}

/// decode a stream to a LOKI message
pub fn decode(msg_name: String, stream: &[u8]) -> Map<String, Value> {
    let msg = crate::loki_message::get_structure_from_msg_type(msg_name).unwrap();
    return crate::decode_map_to_buffer::decode(msg, stream);
}

/// the package sending interface of tested systems
pub fn send_packets(_target_id: String, _msg: LokiMessage) {
    todo!();
}

/// receive the packets
pub fn recv_packets() {}
