//! encoding and decoding methods for different rules
//! Currently, we support the protobuf and RLP

use crate::loki_message::*;
use integer_encoding::VarInt;

/// the encoding ways for protobuf
pub fn encode_protobuf(message: &LokiMessage) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let cur_structure = message.get_structure();
    let mut field_num = 1;
    for attr in cur_structure.get_attrs() {
        let attr_ty = attr.get_attr_type();
        let attr_name = attr.get_attr_name();
        let msg_content = message.get_content();
        match &attr_ty[..] {
            //varint
            "Number" => {
                let cur_val = msg_content.get(&attr_name).unwrap();
                if cur_val.is_u64() {
                    let key: u8 = (field_num << 3) | 0;
                    res.push(key);
                    let cur_u64_val = cur_val.as_u64().unwrap();
                    let mut stream = cur_u64_val.encode_var_vec();
                    res.append(&mut stream);
                } else {
                    let key: u8 = (field_num << 3) | 0;
                    res.push(key);
                    let cur_i64_val = cur_val.as_i64().unwrap();
                    let mut stream = cur_i64_val.encode_var_vec();
                    res.append(&mut stream);
                }
            }
            // varint
            "Bool" => {
                let cur_val = msg_content.get(&attr_name).unwrap();
                let cur_bool_val = cur_val.as_bool().unwrap();
                let key: u8 = (field_num << 3) | 0;
                res.push(key);
                if cur_bool_val == true {
                    let mut stream = 1_u64.encode_var_vec();
                    res.append(&mut stream);
                } else {
                    let mut stream = 0_u64.encode_var_vec();
                    res.append(&mut stream);
                }
            }
            // length delimited
            "String" | "Timestamp" | "BigNumber" => {
                let cur_val = msg_content.get(&attr_name).unwrap();
                let cur_string_val = cur_val.as_str().unwrap().to_string();
                let mut data = length_delimited_string(field_num, cur_string_val);
                res.append(&mut data);
            }
            // length delimited
            "Byte" | "Hash" | "Signature" => {
                let cur_val = msg_content.get(&attr_name).unwrap();
                let mut cur_byte_val = cur_val
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| (v.as_u64().unwrap() as u8))
                    .collect();
                let mut data = length_delimited_bytes(field_num, &mut cur_byte_val);
                res.append(&mut data);
            }
            // length delimited
            "Array" => {
                let cur_val = msg_content.get(&attr_name).unwrap();
                let cur_arr_val = cur_val.as_array().unwrap();
                let array_type = attr.get_attr_reff();
                let mut stream_data: Vec<u8> = vec![];
                for v in cur_arr_val.iter() {
                    match &array_type[..] {
                        "Number" => {
                            if v.is_u64() {
                                let cur_val = v.as_u64().unwrap();
                                let mut stream = cur_val.encode_var_vec();
                                stream_data.append(&mut stream);
                            } else {
                                let cur_val = v.as_i64().unwrap();
                                let mut stream = cur_val.encode_var_vec();
                                stream_data.append(&mut stream);
                            }
                        }
                        "Bool" => {
                            let cur_bool_val = v.as_bool().unwrap();
                            if cur_bool_val == true {
                                let mut stream = 1_u64.encode_var_vec();
                                stream_data.append(&mut stream);
                            } else {
                                let mut stream = 0_u64.encode_var_vec();
                                stream_data.append(&mut stream);
                            }
                        }
                        "String" | "Timestamp" | "BigNumber" => {
                            let cur_string_val = v.as_str().unwrap().to_string();
                            // here i think the the string array should not be added by the field number
                            let mut data = length_delimited_string_without_field(cur_string_val);
                            stream_data.append(&mut data);
                        }
                        "Byte" | "Hash" | "Signature" => {
                            let mut cur_byte_val = v
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| (v.as_u64().unwrap() as u8))
                                .collect();
                            let mut data = length_delimited_bytes_without_field(&mut cur_byte_val);
                            stream_data.append(&mut data);
                        }
                        _ => {}
                    }
                }
                let mut data = length_delimited_bytes(field_num, &mut stream_data);
                res.append(&mut data);
            }
            // length delimited
            "Struct" => {
                // this is a new json object
                let cur_val = msg_content.get(&attr_name).unwrap();
                let refer_message = attr.get_attr_reff();
                let mut refer_loki_msg = generate_loki_message_by_type(refer_message);
                refer_loki_msg.set_content(cur_val.as_object().unwrap().clone());
                // call the encode for the new message
                let mut refer_data = encode_protobuf(&refer_loki_msg);
                let mut data = length_delimited_bytes(field_num, &mut refer_data);
                res.append(&mut data);
            }
            _ => {}
        }
        field_num += 1;
    }
    res
}

/// the decoding ways for protobuf
pub fn decode_protobuf(_stream: Vec<u8>, _from_node: String) -> LokiMessage {
    todo!()
}

/// get the length delimited encoding for a string
pub fn length_delimited_string(field_num: u8, data: String) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let key: u8 = (field_num << 3) | 2;
    res.push(key);
    let mut field_data = data.as_bytes().to_vec();
    res.append(&mut field_data.len().encode_var_vec());
    res.append(&mut field_data);
    res
}

/// get the length delimited encoding for a string without the field number
pub fn length_delimited_string_without_field(data: String) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let mut field_data = data.as_bytes().to_vec();
    res.append(&mut field_data.len().encode_var_vec());
    res.append(&mut field_data);
    res
}

/// get the length delimited encoding for bytes
pub fn length_delimited_bytes(field_num: u8, data: &mut Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let key: u8 = (field_num << 3) | 2;
    res.push(key);
    res.append(&mut data.len().encode_var_vec());
    res.append(data);
    res
}

/// get the length delimited encoding for bytes without the field number
pub fn length_delimited_bytes_without_field(data: &mut Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    res.append(&mut data.len().encode_var_vec());
    res.append(data);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_delimited() {
        let res = length_delimited_string(2, "testing".to_string());
        let expect_res = vec![0x12_u8, 0x07_u8, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
        assert_eq!(res, expect_res);
    }

    #[test]
    fn test_protobuf_encode() {
        assert!(true);
    }
}
