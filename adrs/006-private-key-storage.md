# Storage of Private Keys

## Context

We need to securely store private keys for the wallets such that they can only be accessed using a user-supplied password/pin.

## Decision

We will encrypt the private key with ChaCha20-Poly1305, using argon2 to derive a symmetric key from the user-supplied password. The encrypted private key will be stored along with other data required to decrypt the key, such as the salt and nonce used, and a `version` number which will allow us to support different methods of encrypting the private key in the future if needed. The version described in this ADR (and only version currently) is `version` 1.

We will the the [cryptoxide](https://github.com/typed-io/cryptoxide) library for the cryptographic operations. We will use the [zeroize](https://docs.rs/zeroize/latest/zeroize/) library to handle safe memory-wiping.

The steps of encrypting the private key will look like:

1. Generate a random `salt` of length 16 to use in the argon2 KDF
2. Derive the symmetric key that will be used to encrypt the private key by calling argon2 on the user-supplied password and the salt, using argon2d mode with 1_000_000 iterations and with no optional key or AAD
3. Generate a random `nonce` of length 12 to use in the ChaCha20-Poly1305 encryption
4. Use the derived key and generated nonce to ChaCha20-Poly1305 encrypt the private key we wish to secure, which will return the `ciphertext` and an AEAD `tag`
5. Output and store the `data` bytestring `(version || salt || nonce || tag || ciphertext)`

Then, the steps of decrypting the private key from the `data` bytestring will look like:

1. Split the `data` bytestring into the individual parts using the constant sizes of the version, salt, nonce, and tag
2. Receive the password from the user
3. As before, derive the symmetric key from the user-supplied password using argon2 and the salt decoded from the `data` bytestring
4. Use the derived symmetric key to decrypt the ciphertext using ChaCha20-Poly1305 and the tag decoded from the `data` bytestring, retreiving the private key
