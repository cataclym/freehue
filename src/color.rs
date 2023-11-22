use rand::RngCore;

pub fn random_color() -> String {
    // compile-time length, use `vec![0;len]` for runtime
    let mut bytes: [u8; 3] = [0; 3];
    rand::thread_rng().fill_bytes(&mut bytes);
    // demo-ing both crates, either works
    format!("#{}", hex::encode(&bytes))
}