use protobuf::Message;
use protobuf_json_mapping::*;
use serde_json::{Map, Value, json};
use crate::protos::Consensus::*;
use crate::protos::PBFT::*;

pub fn encode(msg: loki_spec::loki_spec::message::Message, content:Map<String, Value>) -> Vec<u8>{
	let msg_name = msg.get_name();
	match msg_name.as_str() {
		"BaseMessage" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<BaseMessage>(&json_str).expect("BaseMessage parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("BaseMessage write to bytes error!"); 
			return bytes;
		}
		"PBFTRawProposal" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PBFTRawProposal>(&json_str).expect("PBFTRawProposal parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PBFTRawProposal write to bytes error!"); 
			return bytes;
		}
		"PBFTRawMessage" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<PBFTRawMessage>(&json_str).expect("PBFTRawMessage parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("PBFTRawMessage write to bytes error!"); 
			return bytes;
		}
		"RawViewChangeMessage" => { 
			// println!("THe content is {:?}",content);
			let json_str = json!(content).to_string(); 
			// println!("THe json_str is {:?}",json_str);
			let mut tmp = protobuf_json_mapping::parse_from_str::<RawViewChangeMessage>(&json_str).expect("RawViewChangeMessage parse from map_str error!"); 
			// let bytes = tmp.write_to_bytes().expect("RawViewChangeMessage write to bytes error!"); 
			// println!("THe tmp is {:?}",tmp);
			let bytes = tmp.write_to_bytes().expect("RawViewChangeMessage write to bytes error!"); 
			return bytes;
		}
		"RawNewViewMessage" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<RawNewViewMessage>(&json_str).expect("RawNewViewMessage parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("RawNewViewMessage write to bytes error!"); 
			return bytes;
		}
		"ProposalRequest" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<ProposalRequest>(&json_str).expect("ProposalRequest parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("ProposalRequest write to bytes error!"); 
			return bytes;
		}
		"RawMessage" => { 
            println!("content is {:?}",content);
            let parse_options = protobuf_json_mapping::ParseOptions {
                ignore_unknown_fields: true,
                ..Default::default()
            };
			let json_str = json!(content).to_string(); 
            println!("json_str is {:?}",json_str);
			let mut tmp = protobuf_json_mapping::parse_from_str_with_options::<RawMessage>(&json_str,&parse_options).expect("RawMessage parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("RawMessage write to bytes error!"); 
			return bytes;
		}
		"RawProposal" => { 
			let json_str = json!(content).to_string(); 
			let mut tmp = protobuf_json_mapping::parse_from_str::<RawProposal>(&json_str).expect("RawProposal parse from map_str error!"); 
			let bytes = tmp.write_to_bytes().expect("RawProposal write to bytes error!"); 
			return bytes;
		}
		_ => println!()
	}
	return [].to_vec()
}