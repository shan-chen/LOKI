use crate::protos::orderer::etcdraft::configuration::*;
use crate::protos::orderer::cluster::*;
use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{json, Map, Value};

pub fn decode(msg: loki_spec::loki_spec::message::Message, stream: &[u8]) -> Map<String, Value> {
    let msg_name = msg.get_name();
    match msg_name.as_str() {
        "RedeemRequest" => {
            let tmp =
                RedeemRequest::parse_from_bytes(stream).expect("RedeemRequest parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "AllowanceRecipientShare" => {
            let tmp = AllowanceRecipientShare::parse_from_bytes(stream)
                .expect("AllowanceRecipientShare parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "ApproveRequest" => {
            let tmp = ApproveRequest::parse_from_bytes(stream)
                .expect("ApproveRequest parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "SignedCommand" => {
            let tmp = SignedCommand::parse_from_bytes(stream)
                .expect("SignedCommand parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
				enum_values_int: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            println!("current json_str is {:?}",json_str);
			let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			println!("current v is {:?}",v.clone());
            return v;
        }
        "TokenTransaction" => {
            let tmp = TokenTransaction::parse_from_bytes(stream)
                .expect("TokenTransaction parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "KVRWSet" => {
            let tmp = KVRWSet::parse_from_bytes(stream)
                .expect("KVRWSet parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "HashedRWSet" => {
            let tmp =
                HashedRWSet::parse_from_bytes(stream).expect("HashedRWSet parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "KVRead" => {
            let tmp =
                KVRead::parse_from_bytes(stream).expect("KVRead parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "KVWrite" => {
            let tmp =
                KVWrite::parse_from_bytes(stream).expect("KVWrite parse from bytes error!");
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        _ => println!(),
    }
    return serde_json::Map::new();
}