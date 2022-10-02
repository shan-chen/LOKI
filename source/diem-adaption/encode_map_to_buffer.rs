use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{Map, Value, json};
use crate::protos::Consensus::*;
use crate::protos::PBFT::*;

pub fn encode(msg: loki_spec::loki_spec::message::Message, content:Map<String, Value>) -> Vec<u8>{
	let msg_name = msg.get_name();
	match msg_name.as_str() {
		"ProposalMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<ProposalMsg>(&json_str).expect("ProposalMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("ProposalMsg write to bytes error!"); 
			return bytes;
		}
		"SyncInfo" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<SyncInfo>(&json_str).expect("SyncInfo parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("SyncInfo write to bytes error!"); 
			return bytes;
		}
		"VoteMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<VoteMsg>(&json_str).expect("VoteMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("VoteMsg write to bytes error!"); 
			return bytes;
		}
		"CommitVoteMsg" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<CommitVoteMsg>(&json_str).expect("CommitVoteMsg parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("CommitVoteMsg write to bytes error!"); 
			return bytes;
		}
		"EpochChangeProof" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<EpochChangeProof>(&json_str).expect("EpochChangeProof parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("EpochChangeProof write to bytes error!"); 
			return bytes;
		}
		"EpochRetrievalRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<EpochRetrievalRequest>(&json_str).expect("EpochRetrievalRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("EpochRetrievalRequest write to bytes error!"); 
			return bytes;
		}
	}
	return [].to_vec()
}