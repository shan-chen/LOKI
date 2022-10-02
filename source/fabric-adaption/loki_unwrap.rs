use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{Map, Value, json};
use crate::protos::orderer::etcdraft::configuration::*;
use crate::protos::orderer::cluster::*;

pub fn encode(msg: loki_spec::loki_spec::message::Message, content:Map<String, Value>) -> Vec<u8>{
	let msg_name = msg.get_name();
	match msg_name.as_str() {
		"RedeemRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<RedeemRequest>(&json_str).expect("RedeemRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("RedeemRequest write to bytes error!"); 
			return bytes;
		}
		"AllowanceRecipientShare" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<AllowanceRecipientShare>(&json_str).expect("AllowanceRecipientShare parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("AllowanceRecipientShare write to bytes error!"); 
			return bytes;
		}
		"ApproveRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<ApproveRequest>(&json_str).expect("ApproveRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("ApproveRequest write to bytes error!"); 
			return bytes;
		}
		"SignedCommand" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<SignedCommand>(&json_str).expect("SignedCommand parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("SignedCommand write to bytes error!"); 
			return bytes;
		}
		"KVRWSet" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<KVRWSet>(&json_str).expect("RawNewViewMessage parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("KVRWSet write to bytes error!"); 
			return bytes;
		}
		"TokenTransaction" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<TokenTransaction>(&json_str).expect("ProposalRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("TokenTransaction write to bytes error!"); 
			return bytes;
		}
		"HashedRWSet" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str_with_options::<HashedRWSet>(&json_str,&parse_options).expect("HashedRWSet parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("HashedRWSet write to bytes error!"); 
			return bytes;
		}
		"KVRead" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<KVRead>(&json_str).expect("KVRead parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("KVRead write to bytes error!"); 
			return bytes;
		}
		"KVWrite" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<KVWrite>(&json_str).expect("KVWrite parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("KVWrite write to bytes error!"); 
			return bytes;
		}
		_ => println!()
	}
	return [].to_vec()
}