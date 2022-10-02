//! all the hash functions used in LOKI
use hex;
use sha3::*;
use sm3::{Digest, Sm3};

/// the implementation of keccak256 hash function
pub fn keccak256(data: Vec<u8>) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(data.clone());
    let res = hasher.finalize();
    hex::encode(res)
}

/// the implementation of sha3 hash function
pub fn sha3_256(data: Vec<u8>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let res = hasher.finalize();
    hex::encode(res)
}

/// the implementation of sm3 function
pub fn sm3(data: Vec<u8>) -> String {
    let mut hasher = Sm3::new();
    hasher.update(data);
    let res = hasher.finalize();
    hex::encode(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Test Hash function's implementation
     */

    #[test]
    fn test_keccak256() {
        let input = "abc";
        let bytes = input.as_bytes().to_vec();
        let res = keccak256(bytes.clone());
        assert_eq!(
            res,
            "4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45".to_string()
        );
    }

    #[test]
    fn test_sm3() {
        let input = "abc";
        let bytes = input.as_bytes().to_vec();
        let res = sm3(bytes.clone());
        assert_eq!(
            res,
            "66C7F0F462EEEDD9D1F2D46BDC10E4E24167C4875CF2F7A2297DA02B8F4BA8E0"
                .to_string()
                .to_ascii_lowercase()
        );
    }

    #[test]
    fn test_sha3_256() {
        let input = "abc";
        let bytes = input.as_bytes().to_vec();
        let res = sha3_256(bytes.clone());
        assert_eq!(
            res,
            "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532".to_string()
        );
    }
}
