//! all the signature functions used in LOKI
use secp256k1::{Secp256k1, SecretKey, XOnlyPublicKey};
use std::fs;

/// SECP256K1: get the private key object from the private key file
pub fn get_private_key_secp256k1(private_key_file: &str) -> SecretKey {
    let data = &fs::read(private_key_file).unwrap();
    let sk = SecretKey::from_slice(data).expect("32 bytes, within curve order");
    sk
}

/// SECP256K1: get the public key object from the public key file
pub fn get_public_key_secp256k1(public_key_file: &str) -> XOnlyPublicKey {
    let data = &fs::read(public_key_file).unwrap();
    let sk = XOnlyPublicKey::from_slice(data).expect("32 bytes, within curve order");
    sk
}

/// SECP256K1: sign a message with a secret key
/// This function returns a signature object, the user can change it to :
/// 1) bytes: serialize_compact(&self) -> [u8; 64]
/// 2) DER format signature: serialize_der(&self) -> SerializedSignature
pub fn sign_secp256k1(private_key: SecretKey, msg: Vec<u8>) -> Vec<u8> {
    let data = secp256k1::Message::from_slice(&msg).unwrap();
    let ctx = Secp256k1::new();
    let res = ctx.sign_ecdsa(&data, &private_key);
    return res.serialize_compact().to_vec();
}
