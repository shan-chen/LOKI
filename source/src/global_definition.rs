//! Global variable definition

/********************
 * Definition of Boundary Values of Integer Types in Different Languages
 ********************/
#![allow(missing_docs)]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub const MIN_UINT: u128 = 0;

/// Rust: unsigned integer values
pub const RUST_MAX_U8: u128 = 255;
pub const RUST_MAX_U16: u128 = 65535;
pub const RUST_MAX_U32: u128 = 4294967295;
pub const RUST_MAX_U64: u128 = 18446744073709551615;
pub const RUST_MAX_U128: u128 = 340282366920938463463374607431768211455;

/// Rust: signed integer values
pub const RUST_MAX_I8: i128 = 127;
pub const RUST_MIN_I8: i128 = -128;
pub const RUST_MAX_I16: i128 = 32767;
pub const RUST_MIN_I16: i128 = -32768;
pub const RUST_MAX_I32: i128 = 2147483647;
pub const RUST_MIN_I32: i128 = -2147483648;
pub const RUST_MAX_I64: i128 = 9223372036854775807;
pub const RUST_MIN_I64: i128 = -9223372036854775808;
pub const RUST_MAX_I128: i128 = 170141183460469231731687303715884105727;
pub const RUST_MIN_I128: i128 = -170141183460469231731687303715884105728;

// C/C++: unsigned integer values
pub const C_MAX_UINT8: u128 = 255;
pub const C_MAX_USHORT_UINT16: u128 = 65535;
pub const C_MAX_UINT_ULONG_UINT32: u128 = 4294967295;
pub const C_MAX_ULONGLONG_UINT64: u128 = 18446744073709551615;

// C/C++: signed integer values
pub const C_MAX_INT8: i128 = 127;
pub const C_MIN_INT8: i128 = -128;
pub const C_MAX_SHORT_INT16: i128 = 32767;
pub const C_MIN_SHORT_INT16: i128 = -32768;
pub const C_MAX_INT_LONG_INT32: i128 = 2147483647;
pub const C_MIN_INT_LONG_INT32: i128 = -2147483648;
pub const C_MAX_LONGLONG_INT64: i128 = 9223372036854775807;
pub const C_MIN_LONGLONG_INT64: i128 = -9223372036854775808;

// Golang: unsigned integer values
pub const GO_MAX_UINT8: u128 = 255;
pub const GO_MAX_UINT16: u128 = 65535;
pub const GO_MAX_UINT32: u128 = 4294967295;
pub const GO_MAX_UINT64: u128 = 18446744073709551615;

// Golang: signed integer values
pub const GO_MAX_INT8: i128 = 127;
pub const GO_MIN_INT8: i128 = -128;
pub const GO_MAX_INT16: i128 = 32767;
pub const GO_MIN_INT16: i128 = -32768;
pub const GO_MAX_INT32: i128 = 2147483647;
pub const GO_MIN_INT32: i128 = -2147483648;
pub const GO_MAX_INT64: i128 = 9223372036854775807;
pub const GO_MIN_INT64: i128 = -9223372036854775808;

/// indicates the encoded method of current blockchain
/// 0 means protobuf
/// 1 means RLP
/// 2 means user-defined encode function
pub const ENCODE_METHOD: u32 = 0;

lazy_static! {
    pub static ref HASH_FUNCTIONS: Mutex<HashMap<String, fn(Vec<u8>) -> String>> =
        Mutex::new(HashMap::new());
}

pub static mut SPEC_MESSAGES: Vec<loki_spec::loki_spec::message::Message> = vec![];

/// set the spec visitor
pub fn set_message_list(new_messages_list: Vec<loki_spec::loki_spec::message::Message>) {
    unsafe {
        SPEC_MESSAGES = new_messages_list;
    }
}

/// get the spec visitor
pub fn get_message_list() -> Vec<loki_spec::loki_spec::message::Message> {
    unsafe { SPEC_MESSAGES.clone() }
}

/// set all of the hash functions
pub fn add_hash_function(new_type: String, hash_fn: fn(Vec<u8>) -> String) {
    let mut l = HASH_FUNCTIONS.lock().unwrap();
    (*l).insert(new_type, hash_fn);
}

/// get the hash function
pub fn get_hash_func_from_name(hash_type: String) -> fn(Vec<u8>) -> String {
    let res = HASH_FUNCTIONS.lock().unwrap();
    *(*res)
        .clone()
        .get(&hash_type)
        .expect("Cannot find the hash function")
}
