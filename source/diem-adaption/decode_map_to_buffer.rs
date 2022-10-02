use crate::protos::Consensus::*;
use crate::protos::PBFT::*;
use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{json, Map, Value};

pub fn decode(msg: loki_spec::loki_spec::message::Message, stream: &[u8]) -> Map<String, Value> {
    let msg_name = msg.get_name();
    match msg_name.as_str() {
        "ProposalMsg" => {
            let tmp =
            ProposalMsg::parse_from_bytes(stream).expect("ProposalMsg parse from bytes error!");
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
        "SyncInfo" => {
            let tmp = SyncInfo::parse_from_bytes(stream)
                .expect("SyncInfo parse from bytes error!");
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
        "VoteMsg" => {
            let tmp = VoteMsg::parse_from_bytes(stream)
                .expect("VoteMsg parse from bytes error!");
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
        "CommitVoteMsg" => {
            let tmp = CommitVoteMsg::parse_from_bytes(stream)
                .expect("CommitVoteMsg parse from bytes error!");
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
        "CommitDecisionMsg" => {
            let tmp = CommitDecisionMsg::parse_from_bytes(stream)
                .expect("CommitDecisionMsg parse from bytes error!");
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
        "EpochChangeProof" => {
            let tmp = EpochChangeProof::parse_from_bytes(stream)
                .expect("EpochChangeProof parse from bytes error!");
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
        "EpochRetrievalRequest" => {
            let tmp =
            EpochRetrievalRequest::parse_from_bytes(stream).expect("EpochRetrievalRequest parse from bytes error!");
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