use crate::protos::Consensus::*;
use crate::protos::PBFT::*;
use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{json, Map, Value};

pub fn decode(msg: loki_spec::loki_spec::message::Message, stream: &[u8]) -> Map<String, Value> {
    let msg_name = msg.get_name();
    match msg_name.as_str() {
        "BaseMessage" => {
            let tmp =
                BaseMessage::parse_from_bytes(stream).expect("BaseMessage parse from bytes error!");
            //convert tmp => map
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            // let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("BaseMessage parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "PBFTRawProposal" => {
            let tmp = PBFTRawProposal::parse_from_bytes(stream)
                .expect("PBFTRawProposal parse from bytes error!");
            //convert tmp => map
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            // let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("BaseMessage parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "PBFTRawMessage" => {
            let tmp = PBFTRawMessage::parse_from_bytes(stream)
                .expect("PBFTRawMessage parse from bytes error!");
            //convert tmp => map
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            // let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("BaseMessage parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "RawViewChangeMessage" => {
            let tmp = RawViewChangeMessage::parse_from_bytes(stream)
                .expect("RawViewChangeMessage parse from bytes error!");
            //convert tmp => map
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
				enum_values_int: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            // let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("BaseMessage parse to map_str error!");
            println!("current json_str is {:?}",json_str);
			let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
			println!("current v is {:?}",v.clone());
            return v;
        }
        "RawNewViewMessage" => {
            let tmp = RawNewViewMessage::parse_from_bytes(stream)
                .expect("RawNewViewMessage parse from bytes error!");
            //convert tmp => map
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            // let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("BaseMessage parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "ProposalRequest" => {
            let tmp = ProposalRequest::parse_from_bytes(stream)
                .expect("ProposalRequest parse from bytes error!");
            //convert tmp => map
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            // let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("BaseMessage parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "RawMessage" => {
            let tmp =
                RawMessage::parse_from_bytes(stream).expect("RawMessage parse from bytes error!");
            //convert tmp => map
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            // let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("BaseMessage parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        "RawProposal" => {
            let tmp =
                RawProposal::parse_from_bytes(stream).expect("RawProposal parse from bytes error!");
            //convert tmp => map
            let print_options = protobuf_json_mapping::PrintOptions {
                proto_field_name: true,
                always_output_default_values: true,
                ..Default::default()
            };
            let json_str =
                protobuf_json_mapping::print_to_string_with_options(&tmp, &print_options)
                    .expect("Member parse to map_str error!");
            // let json_str = protobuf_json_mapping::print_to_string(&tmp).expect("BaseMessage parse to map_str error!");
            let v: Map<String, Value> =
                serde_json::from_str(&json_str).expect("serde_json::from_str error!");
            return v;
        }
        _ => println!(),
    }
    return serde_json::Map::new();
}