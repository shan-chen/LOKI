//! Utils contain some general helper functions
use crate::global_definition::*;

/// get max/min value of unsigned number for different languages
pub fn get_unsigned_edge_value(language: String, size: usize) -> (u128, u128) {
    let max_val: u128;
    let min_val: u128 = MIN_UINT;
    match language.to_ascii_lowercase().as_str() {
        "rust" => {
            match size {
                8 => {
                    max_val = RUST_MAX_U8;
                }
                16 => {
                    max_val = RUST_MAX_U16;
                }
                32 => {
                    max_val = RUST_MAX_U32;
                }
                64 => {
                    max_val = RUST_MAX_U64;
                }
                128 => {
                    max_val = RUST_MAX_U128;
                }
                _ => {
                    max_val = RUST_MAX_U128;
                }
            }
            (max_val, min_val)
        }
        "c" | "cpp" | "c++" => {
            match size {
                8 => {
                    max_val = C_MAX_UINT8;
                }
                16 => {
                    max_val = C_MAX_USHORT_UINT16;
                }
                32 => {
                    max_val = C_MAX_UINT_ULONG_UINT32;
                }
                64 => {
                    max_val = C_MAX_ULONGLONG_UINT64;
                }
                _ => {
                    max_val = C_MAX_ULONGLONG_UINT64;
                }
            }
            (max_val, min_val)
        }
        "go" => {
            match size {
                8 => {
                    max_val = GO_MAX_UINT8;
                }
                16 => {
                    max_val = GO_MAX_UINT16;
                }
                32 => {
                    max_val = GO_MAX_UINT32;
                }
                64 => {
                    max_val = GO_MAX_UINT64;
                }
                _ => {
                    max_val = GO_MAX_UINT64;
                }
            }
            (max_val, min_val)
        }
        _ => {
            panic!("unsupported language type: {}", language);
        }
    }
}

/// get max/min value of signed number for different languages
pub fn get_signed_edge_value(language: String, size: usize) -> (i128, i128) {
    let max_val: i128;
    let min_val: i128;
    match language.to_ascii_lowercase().as_str() {
        "rust" => {
            match size {
                8 => {
                    max_val = RUST_MAX_I8;
                    min_val = RUST_MIN_I8;
                }
                16 => {
                    max_val = RUST_MAX_I16;
                    min_val = RUST_MIN_I16;
                }
                32 => {
                    max_val = RUST_MAX_I32;
                    min_val = RUST_MIN_I32;
                }
                64 => {
                    max_val = RUST_MAX_I64;
                    min_val = RUST_MIN_I64;
                }
                128 => {
                    max_val = RUST_MAX_I128;
                    min_val = RUST_MIN_I128;
                }
                _ => {
                    max_val = RUST_MAX_I128;
                    min_val = RUST_MIN_I128;
                }
            }
            (max_val, min_val)
        }
        "c" | "cpp" | "c++" => {
            match size {
                8 => {
                    max_val = C_MAX_INT8;
                    min_val = C_MIN_INT8;
                }
                16 => {
                    max_val = C_MAX_SHORT_INT16;
                    min_val = C_MIN_SHORT_INT16;
                }
                32 => {
                    max_val = C_MAX_INT_LONG_INT32;
                    min_val = C_MIN_INT_LONG_INT32;
                }
                64 => {
                    max_val = C_MAX_LONGLONG_INT64;
                    min_val = C_MIN_LONGLONG_INT64;
                }
                _ => {
                    max_val = C_MAX_LONGLONG_INT64;
                    min_val = C_MIN_LONGLONG_INT64;
                }
            }
            (max_val, min_val)
        }
        "go" => {
            match size {
                8 => {
                    max_val = GO_MAX_INT8;
                    min_val = GO_MIN_INT8;
                }
                16 => {
                    max_val = GO_MAX_INT16;
                    min_val = GO_MIN_INT16;
                }
                32 => {
                    max_val = GO_MAX_INT32;
                    min_val = GO_MIN_INT32;
                }
                64 => {
                    max_val = GO_MAX_INT64;
                    min_val = GO_MIN_INT64;
                }
                _ => {
                    max_val = GO_MAX_INT64;
                    min_val = GO_MIN_INT64;
                }
            }
            (max_val, min_val)
        }
        _ => {
            panic!("unsupported language type: {}", language);
        }
    }
}

/// addition of large numbers in Rust
pub fn add_string_by_bits(num1: String, num2: String) -> String {
    let mut res = Vec::new();
    let mut i = num1.len();
    let mut j = num2.len();
    let mut carry = 0;

    while i > 0 || j > 0 {
        let n1 = {
            if i > 0 {
                num1.as_bytes()[i - 1] - '0' as u8
            } else {
                0
            }
        };
        let n2 = {
            if j > 0 {
                num2.as_bytes()[j - 1] - '0' as u8
            } else {
                0
            }
        };

        let tmp = n1 + n2 + carry;
        carry = tmp / 10;
        res.push(tmp % 10 + '0' as u8);
        if i > 0 {
            i -= 1;
        }
        if j > 0 {
            j -= 1;
        }
    }
    if carry == 1 {
        res.push('1' as u8);
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

/// subtraction of large numbers in Rust
pub fn sub_string_by_bits(num1: String, num2: String) -> String {
    let mut res = Vec::new();
    let mut i = num1.len();
    let mut j = num2.len();
    let mut bit = 0;

    while i > 0 || j > 0 {
        let mut n1 = {
            if i > 0 {
                num1.as_bytes()[i - 1] - '0' as u8
            } else {
                0
            }
        };
        let n2 = {
            if j > 0 {
                num2.as_bytes()[j - 1] - '0' as u8
            } else {
                0
            }
        };

        let tmp;
        if n1 < n2 + bit {
            n1 += 10;
            tmp = n1 - bit - n2;
            bit = 1;
        } else {
            tmp = n1 - bit - n2;
            bit = 0;
        }
        res.push(tmp % 10 + '0' as u8);
        if i > 0 {
            i -= 1;
        }
        if j > 0 {
            j -= 1;
        }
    }
    // simulate underflow
    if bit == 1 {
        res.push('1' as u8);
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}
