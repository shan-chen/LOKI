#![allow(missing_docs)]
//! the types supported by LOKI
use anyhow::Result;
use std::str::FromStr;

use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub enum BasicType {
    BOOL,
    NUMBER,
    BYTE,
    STRING,
    BIGNUMBER,
    TIMESTAMP,
}
pub static mut TIMESTAMP_LENGTH: usize = 16;
lazy_static! {
    pub static ref CURRENT_LANGUAGE: Mutex<String> = Mutex::new(String::from("CPP"));
}

lazy_static! {
    pub static ref PRIVATE_KEY_FILE: Mutex<String> = Mutex::new(String::from("privatekey.key"));
}
/// construct basic types according to the string
impl FromStr for BasicType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = match &s.to_ascii_lowercase()[..] {
            "bool" => Self::BOOL,
            "number" => Self::NUMBER,
            "byte" => Self::BYTE,
            "string" => Self::STRING,
            "bignumber" => Self::BIGNUMBER,
            "timestamp" => Self::TIMESTAMP,
            _ => return Err("unsupported basic type".to_string()),
        };
        Ok(t)
    }
}

/// set the timestamp length
pub fn set_timestamp_length(t_len: usize) {
    unsafe {
        TIMESTAMP_LENGTH = t_len;
    }
}

/// set the current language
pub fn set_current_language(language: String) {
    let mut l = CURRENT_LANGUAGE.lock().unwrap();
    *l = language;
}

/// get the current language
pub fn get_current_language() -> String {
    let res = CURRENT_LANGUAGE.lock().unwrap();
    (*res).to_string()
}

/// set the current private key path
pub fn set_current_private_key(file: String) {
    let mut l = PRIVATE_KEY_FILE.lock().unwrap();
    *l = file;
}

/// get the current private key path
pub fn get_current_private_key() -> String {
    let res = PRIVATE_KEY_FILE.lock().unwrap();
    (*res).to_string()
}

/// the array type in LOKI
#[derive(Debug)]
pub struct Array {
    ele_type: BasicType,
    length: u32,
    fixed: bool,
    content: Vec<String>,
}

impl Array {
    /// construct a new array
    pub fn new(ele_type: BasicType, length: u32, fixed: bool) -> Self {
        let content = vec!["".to_string()];
        Array {
            ele_type,
            length,
            fixed,
            content,
        }
    }

    /// whether the array's length is fixed
    pub fn is_fixed(&self) -> bool {
        self.fixed
    }

    /// whether the array is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// modify the content
    pub fn update_content(&mut self, new_content: Vec<String>) {
        self.content = new_content;
    }

    /// get the content
    pub fn get_content(&self) -> &Vec<String> {
        &self.content
    }

    /// get the mutate content
    pub fn get_mut_content(&mut self) -> &mut Vec<String> {
        &mut self.content
    }

    /// set the content
    pub fn set_content(&mut self, new_content: Vec<String>) {
        self.content = new_content;
    }

    /// get the element type
    pub fn get_ele_ty(&self) -> BasicType {
        self.ele_type.clone()
    }

    /// set the fixed
    pub fn set_fixed(&mut self, fixed: bool) {
        self.fixed = fixed;
    }

    /// get the length
    pub fn get_length(&self) -> u32 {
        self.length
    }

    /// set the length
    pub fn set_length(&mut self, new_len: u32) {
        self.length = new_len;
    }
}

#[cfg(test)]
mod tests {
    use crate::loki_type::*;
    #[test]
    fn test_set_current_lan() {
        set_current_language("RUST".to_string());
        assert_eq!(get_current_language(), "RUST".to_string());
        set_current_language("GO".to_string());
        assert_eq!(get_current_language(), "GO".to_string());
    }
}
