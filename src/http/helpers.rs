use rand::Rng;

pub fn generate_transaction_hash() -> String {
    let charset = b"abcdefABCDEF0123456789";
    let mut rng = rand::rng();
    let hash: String = (0..98)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    format!("0x{hash}")
}
