/*
use ed25519_dalek::{KEYPAIR_LENGTH, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SigningKey};
use rand::rngs::OsRng;

fn gen_keys() {
    let mut csprng = OsRng {};
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);

    let verifying_key_bytes: [u8; PUBLIC_KEY_LENGTH] = signing_key.verifying_key().to_bytes();
    let secret_key_bytes: [u8; SECRET_KEY_LENGTH] = signing_key.to_bytes();
    let signing_key_bytes: [u8; KEYPAIR_LENGTH] = signing_key.to_keypair_bytes();

    println!("{:?}", verifying_key_bytes);
}
*/
