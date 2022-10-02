//! The interface that user needs to extend and implement for LOKI adaption
extern crate libloading; 
use std::{env};

extern crate libc;
use std::ffi::{CStr, CString};
use std::thread;
use std::time::Duration;

use crate::loki_message::LokiMessage;
use serde_json::{Map, Value};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoString {
    pub p: *const ::std::os::raw::c_char,
    pub n: isize,
}


/// encode the LOKI message into a stream
pub fn encode(msg_name: String, content: Map<String, Value>) -> Vec<u8> {
    let msg = crate::loki_message::get_structure_from_msg_type(msg_name).unwrap();
    return crate::loki_wrap::encode(msg, content);
}

/// decode a stream to a LOKI message
pub fn decode(msg_name: String, stream: &[u8]) -> Map<String, Value> {
    let msg = crate::loki_message::get_structure_from_msg_type(msg_name).unwrap();
    return crate::loki_unwrap::decode(msg, stream);
}

#[no_mangle]
pub extern "C" fn recv_packets(
    msg_type: std::os::raw::c_int,
    from_id_c: *const libc::c_char,
    payload_c: *const libc::c_uchar,
    payload_size: std::os::raw::c_int,
) -> *const libc::c_char {
    let from_id_name = unsafe { std::ffi::CStr::from_ptr(from_id_c) };
    let from_id = std::str::from_utf8(from_id_name.to_bytes())
        .unwrap()
        .to_string();
    let payload = unsafe { std::slice::from_raw_parts(payload_c, payload_size as usize) };

    let msg_type_string = get_fabric_msg_type(msg_type.try_into().unwrap());
    let msg_structure =
        crate::loki_message::get_structure_from_msg_type(msg_type_string.clone()).unwrap();
    let raw_message_content = decode("Envelope".to_string(), payload);

    let mut content = match &msg_type_string[..] {
        "RedeemRequest"
        | "AllowanceRecipientShare"
        | "ApproveRequest"
        | "SignedCommand"
        | "TokenTransaction"
        | "KVRWSet"
        | "KVRead" => decode(
            "KVWrite".to_string(),
            &base64::decode(
                raw_message_content["payLoad"]
                    .as_str()
                    .expect("Error in as_str()"),
            )
            .unwrap()[..],
        ),
        "KVMetadataWrite" | "KVMetadataEntry" => decode(
            "TxReadWriteSet".to_string(),
            &base64::decode(
                raw_message_content["payLoad"]
                    .as_str()
                    .expect("Error in as_str()"),
            )
            .unwrap()[..],
        ),
        _ => {
            panic!("received type is non of the pbft messages");
        }
    };
    let mut new_content = serde_json::Map::new();
    new_content.insert("message".to_string(), serde_json::Value::from(content));
    let mut recv_msg = LokiMessage::new(from_id, msg_structure, new_content);
    let sent_packet = crate::engine::LOKI_ENGINE
        .lock()
        .unwrap()
        .passive_sending(recv_msg);

        
    let encoded_payload = match &sent_packet.get_structure().get_name()[..] {
        "RedeemRequest"
        | "AllowanceRecipientShare"
        | "ApproveRequest"
        | "SignedCommand"
        | "TokenTransaction"
        | "KVRWSet"
        | "KVRead" => decode(
            "KVWrite".to_string(),
            sent_packet
                .get_content()
                .get("message")
                .unwrap()
                .as_object()
                .unwrap()
                .clone(),
        ),
        "KVMetadataWrite" | "KVMetadataEntry" => decode(
            "TxReadWriteSet".to_string(),
            sent_packet
                .get_content()
                .get("message")
                .unwrap()
                .as_object()
                .unwrap()
                .clone(),
        ),
        _ => {
            panic!("passive sending generate a type that cannot be recognized by Fabric");
        }
    };
   
    
    let mut content_send_raw_packet = serde_json::Map::new();
    content_send_raw_packet.insert("version".to_string(), serde_json::Value::from(0_i32));
    content_send_raw_packet.insert(
        "type".to_string(),
        serde_json::Value::from(get_fabric_type_int(
            sent_packet.get_structure().get_name(),
        )),
    );
    // here the signature Data need to be calculated
    if sent_packet.get_structure().get_name() == "SignedCommand"
    {
        // if the type is SignedCommand, need to handle the signature
        content_send_raw_packet.insert("ECDSASignature".to_string(), serde_json::Value::from(String::new()));
    } else {
        content_send_raw_packet.insert("ECDSASignature".to_string(), serde_json::Value::from(String::new()));
    }
    content_send_raw_packet.insert(
        "payLoad".to_string(),
        serde_json::Value::from(base64::encode(&encoded_payload[..])),
     );
    let encoded_data = encode("Envelope".to_string(), content_send_raw_packet);
    let encoded_data_str = base64::encode(&encoded_data[..]);
    let encoded_data_c = CString::new(encoded_data_str).unwrap();
    return encoded_data_c.as_ptr();
}


#[no_mangle]
pub extern "C" fn get_active_sending_targets() -> Vec<u8> {
    let hash_map_target_messages = LOKI_ENGINE.lock().unwrap().active_sending();
    if hash_map_target_messages.is_empty() {
        return vec![];
    }
    return hash_map_target_messages
        .keys()
        .clone()
        .next()
        .unwrap()
        .to_string()
        .into_bytes();
}

#[no_mangle]
pub extern "C" fn get_active_sending_packets() -> Vec<u8> {
    let hash_map_target_messages = LOKI_ENGINE.lock().unwrap().active_sending();
    if hash_map_target_messages.is_empty() {
        return vec![];
    }
    return hash_map_target_messages
        .values()
        .clone()
        .next()
        .unwrap()
        .to_vec();
}

pub fn get_fabric_type_int(msg_type: String) -> u32 {
    match &msg_type[..] {
        "RedeemRequest" => 0,
        "AllowanceRecipientShare" => 1,
        "ApproveRequest" => 2,
        "SignedCommand" => 3,
        "KVRWSet" => 4,
        "TokenTransaction" => 5,
        "HashedRWSet" => 6,
        "KVRead" => 7,
        "KVWrite" => 8,
        _ => 100,
    }
}