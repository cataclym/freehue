use rand::RngCore;
use rustc_serialize::hex::ToHex;

pub fn random_color() -> String {
    // compile-time length, use `vec![0;len]` for runtime
    let mut bytes: [u8; 3] = [0; 3];
    rand::thread_rng().fill_bytes(&mut bytes);
    // demo-ing both crates, either works
    bytes.to_hex()
}