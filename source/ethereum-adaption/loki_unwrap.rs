use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{Map, Value, json};
use crate::protos::orderer::etcdraft::configuration::*;
use crate::protos::orderer::cluster::*;

pub fn encode(msg: loki_spec::loki_spec::message::Message, content:Map<String, Value>) -> Vec<u8>{
	let msg_name = msg.get_name();
	match msg_name.as_str() {
		"NewBlockMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<NewBlockMsg>(&json_str).expect("NewBlockMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("NewBlockMsg write to bytes error!"); 
			return bytes;
		}
		"TransactionsMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<TransactionsMsg>(&json_str).expect("TransactionsMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("TransactionsMsg write to bytes error!"); 
			return bytes;
		}
		"GetBlockHeadersMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<GetBlockHeadersMsg>(&json_str).expect("GetBlockHeadersMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("GetBlockHeadersMsg write to bytes error!"); 
			return bytes;
		}
		"BlockHeadersMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<BlockHeadersMsg>(&json_str).expect("BlockHeadersMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("BlockHeadersMsg write to bytes error!"); 
			return bytes;
		}
		"GetBlockBodiesMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<GetBlockBodiesMsg>(&json_str).expect("GetBlockBodiesMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("GetBlockBodiesMsg write to bytes error!"); 
			return bytes;
		}
		"BlockBodiesMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<BlockBodiesMsg>(&json_str).expect("BlockBodiesMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("BlockBodiesMsg write to bytes error!"); 
			return bytes;
		}
		"GetNodeDataMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str_with_options::<GetNodeDataMsg>(&json_str,&parse_options).expect("GetNodeDataMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("GetNodeDataMsg write to bytes error!"); 
			return bytes;
		}
		"NodeDataMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<NodeDataMsg>(&json_str).expect("NodeDataMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("NodeDataMsg write to bytes error!"); 
			return bytes;
		}
		"ReceiptsMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<ReceiptsMsg>(&json_str).expect("ReceiptsMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("ReceiptsMsg write to bytes error!"); 
			return bytes;
		}
		_ => println!()
	}
	return [].to_vec()
}