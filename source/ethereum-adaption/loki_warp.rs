use crate::protos::orderer::etcdraft::configuration::*;
use crate::protos::orderer::cluster::*;
use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{json, Map, Value};

pub fn decode(msg: loki_spec::loki_spec::message::Message, stream: &[u8]) -> Map<String, Value> {
    let msg_name = msg.get_name();
    match msg_name.as_str() {
        "NewBlockMsg" => {
            let tmp =
                NewBlockMsg::parse_from_bytes(stream).expect("NewBlockMsg parse from bytes error!");
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
        "TransactionsMsg" => {
            let tmp = TransactionsMsg::parse_from_bytes(stream)
                .expect("TransactionsMsg parse from bytes error!");
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
        "GetBlockHeadersMsg" => {
            let tmp = GetBlockHeadersMsg::parse_from_bytes(stream)
                .expect("GetBlockHeadersMsg parse from bytes error!");
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
        "BlockHeadersMsg" => {
            let tmp = BlockHeadersMsg::parse_from_bytes(stream)
                .expect("BlockHeadersMsg parse from bytes error!");
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
        "GetBlockBodiesMsg" => {
            let tmp = GetBlockBodiesMsg::parse_from_bytes(stream)
                .expect("GetBlockBodiesMsg parse from bytes error!");
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
        "BlockBodiesMsg" => {
            let tmp = BlockBodiesMsg::parse_from_bytes(stream)
                .expect("BlockBodiesMsg parse from bytes error!");
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
        "GetNodeDataMsg" => {
            let tmp =
                GetNodeDataMsg::parse_from_bytes(stream).expect("GetNodeDataMsg parse from bytes error!");
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
        "NodeDataMsg" => {
            let tmp =
                NodeDataMsg::parse_from_bytes(stream).expect("NodeDataMsg parse from bytes error!");
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
        "ReceiptsMsg" => {
            let tmp =
                ReceiptsMsg::parse_from_bytes(stream).expect("ReceiptsMsg parse from bytes error!");
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