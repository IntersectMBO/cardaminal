use cryptoxide::chacha20poly1305::ChaCha20Poly1305;
use cryptoxide::kdf::argon2;
use pallas::crypto::key::ed25519::SecretKey;
use pallas::ledger::traverse::ComputeHash;
use rand::RngCore;

pub type PubKeyHash = [u8; 28];

const ITERATIONS: u32 = 2500;
const VERSION_SIZE: usize = 1;
const SALT_SIZE: usize = 16;
const NONCE_SIZE: usize = 12;
const TAG_SIZE: usize = 16;
const DATA_SIZE: usize = 32;

pub fn keygen() -> (SecretKey, PubKeyHash) {
    let privkey = SecretKey::new(rand::thread_rng());
    let pubkey = privkey.public_key();
    let pubkeyhash = pubkey.compute_hash();

    (privkey, *pubkeyhash)
}

// TODO: add checks/errors
pub fn encrypt_privkey(password: &String, data: [u8; 32]) -> Vec<u8> {
    let salt = {
        let mut salt = [0u8; SALT_SIZE];
        rand::thread_rng().fill_bytes(&mut salt);
        salt
    };

    let sym_key: [u8; 32] = argon2::argon2(
        &argon2::Params::argon2d().iterations(ITERATIONS).unwrap(),
        password.as_bytes(),
        &salt,
        &[],
        &[],
    );

    let nonce = {
        let mut nonce = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    };

    let mut chacha20 = ChaCha20Poly1305::new(&sym_key, &nonce, &[]);

    let (ciphertext, ct_tag) = {
        let mut ciphertext = vec![0u8; data.len()];
        let mut ct_tag = [0u8; 16];
        chacha20.encrypt(&data, &mut ciphertext, &mut ct_tag);

        (ciphertext, ct_tag)
    };

    // (version || salt || nonce || tag || ciphertext)
    let mut out = Vec::with_capacity(VERSION_SIZE + SALT_SIZE + NONCE_SIZE + TAG_SIZE + DATA_SIZE);

    out.push(1);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ct_tag);
    out.extend_from_slice(&ciphertext);

    out
}

#[allow(unused)]
pub fn decrypt_privkey(password: &String, data: Vec<u8>) -> Result<SecretKey, ()> {
    assert_eq!(
        data.len(),
        VERSION_SIZE + SALT_SIZE + NONCE_SIZE + TAG_SIZE + DATA_SIZE
    );

    let mut cursor = 0;

    let _version = &data[cursor];
    cursor += VERSION_SIZE;

    let salt = &data[cursor..cursor + SALT_SIZE];
    cursor += SALT_SIZE;

    let nonce = &data[cursor..cursor + NONCE_SIZE];
    cursor += NONCE_SIZE;

    let tag = &data[cursor..cursor + TAG_SIZE];
    cursor += TAG_SIZE;

    let ciphertext = &data[cursor..cursor + DATA_SIZE];

    let sym_key: [u8; 32] = argon2::argon2(
        &argon2::Params::argon2d().iterations(ITERATIONS).unwrap(),
        password.as_bytes(),
        salt,
        &[],
        &[],
    );

    let mut chacha20 = ChaCha20Poly1305::new(&sym_key, nonce, &[]);

    let mut plaintext = [0u8; DATA_SIZE];

    if chacha20.decrypt(ciphertext, &mut plaintext, tag) {
        Ok(plaintext.into())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::keys;

    #[test]
    fn privkey_encryption_roundtrip() {
        let password = "hunter123";

        let (priv_key, _) = keys::keygen();

        let priv_key: [u8; 32] = priv_key.into();

        let encrypted_priv_key = keys::encrypt_privkey(&password.into(), priv_key);

        let decrypted_privkey =
            keys::decrypt_privkey(&password.into(), encrypted_priv_key).unwrap();

        assert_eq!(priv_key, Into::<[u8; 32]>::into(decrypted_privkey))
    }
}
