use std::collections::BTreeMap;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::Aead;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use base64::prelude::BASE64_STANDARD;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use p384::ecdh::EphemeralSecret;
use p384::ecdsa::SigningKey;
use p384::elliptic_curve::rand_core::OsRng;
use p384::pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use p384::PublicKey;
use serde::Serialize;
use serde_json::json;
use bedrockrs_proto_core::error::EncryptionError;
use aes_gcm::KeyInit;
use p384::SecretKey;
use p384::ecdh::diffie_hellman;
use p384::elliptic_curve::ecdh::SharedSecret;
use p384::NistP384;
use aes::Aes256;
use ctr::cipher::{KeyIvInit, StreamCipher};
use ctr::Ctr128BE;
use sha2::{Digest, Sha256};
type Aes256Ctr = Ctr128BE<Aes256>;

#[derive(Clone)]
pub struct Encryption {
    key: Vec<u8>,
    decrypt_cipher: Aes256Ctr,
    decrypt_counter: u64,
    encrypt_cipher: Aes256Ctr,
    encrypt_counter: u64,
}

#[derive(Debug, Serialize)]
struct Claims {
    salt: String,
}

#[derive(Debug, Serialize)]
struct CustomHeader {
    alg: String,
    x5u: String,
}

#[derive(Debug, Serialize)]
struct JwtPayload {
    salt: String,
}

impl Encryption {
    pub fn fake_gcm(encryption_key: &[u8]) -> Self {
        let iv = [&encryption_key[..12], &[0, 0, 0, 2]].concat();
        Self::new(encryption_key, &iv)
    }

    pub fn cfb8(_encryption_key: &[u8]) -> Self {
        // AES-CFB8 is not directly supported in popular Rust crates, so you'd
        // need to implement it yourself or use bindings to OpenSSL.
        // For simplicity, this is left unimplemented.
        unimplemented!("AES-256-CFB8 is not implemented in this example");
    }

    pub fn new(encryption_key: &[u8], iv: &[u8]) -> Self {
        let key = encryption_key.to_vec();
        let decrypt_cipher = Aes256Ctr::new_from_slices(&key, iv).unwrap();
        let encrypt_cipher = Aes256Ctr::new_from_slices(&key, iv).unwrap();

        Self {
            key,
            decrypt_cipher,
            decrypt_counter: 0,
            encrypt_cipher,
            encrypt_counter: 0,
        }
    }

    pub fn decrypt(&mut self, encrypted: Vec<u8>) -> Result<Vec<u8>, String> {
        if encrypted.len() < 9 {
            return Err("Payload is too short".to_string());
        }

        // Decrypt the data
        let mut decrypted = encrypted.clone();
        self.decrypt_cipher.apply_keystream(&mut decrypted);

        // Calculate the length of the payload (subtracting 8 for checksum)
        let payload_len = decrypted.len() - 8;

        // Split the decrypted data into payload and checksum
        let (mut payload, actual_checksum) = decrypted.split_at_mut(payload_len);

        // Increment the packet counter
        let counter = self.decrypt_counter;
        self.decrypt_counter += 1;

        // Calculate the expected checksum
        let expected_checksum = self.calculate_checksum(counter, payload);

        // Compare the actual and expected checksums
        if actual_checksum != expected_checksum {
            return Err(format!(
                "Encrypted packet {} has invalid checksum (expected {:02x?}, got {:02x?})",
                counter, expected_checksum, actual_checksum
            ));
        }

        // Return the decrypted payload as a Vec<u8>
        Ok(payload.to_vec())
    }

    pub fn encrypt(&mut self, mut payload: Vec<u8>) -> Vec<u8> {
        // Calculate the checksum
        let checksum = self.calculate_checksum(self.encrypt_counter, &payload);
        self.encrypt_counter += 1;

        // Append checksum to the payload
        payload.extend_from_slice(&checksum);

        // Encrypt the payload
        self.encrypt_cipher.apply_keystream(&mut payload);

        payload // Return the encrypted data
    }

    fn calculate_checksum(&self, counter: u64, payload: &[u8]) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(&counter.to_le_bytes());
        hasher.update(payload);
        hasher.update(&self.key);
        let hash = hasher.finalize();

        let mut checksum = [0u8; 8];
        checksum.copy_from_slice(&hash[..8]);
        checksum
    }


    pub fn verify(&mut self, _src: &[u8]) -> Result<(), EncryptionError> {
        unimplemented!()
    }

    pub fn generate_random_token() -> [u8; 16] {
        let mut token = [0u8; 16];
        getrandom::fill(&mut token).unwrap();
        token
    }

    pub fn parse_der_public_key(base64_key: &str) -> Result<PublicKey, Box<dyn std::error::Error>> {
        let der = BASE64_STANDARD.decode(base64_key)?;
        let public_key = PublicKey::from_public_key_der(&der)?;
        Ok(public_key)
    }

    pub fn create_key_pair() -> (SigningKey, PublicKey) {
        // This generates a keypair using the P-384 (secp384r1) curve
        let signing_key = SigningKey::random(&mut OsRng);
        let public_key = signing_key.verifying_key().into();
        (signing_key, public_key)
    }

    pub fn get_secret_key(
        local_private_key: &SigningKey,
        remote_public_key: &PublicKey,
        token: &[u8],
    ) -> [u8; 32] {
        // Specify curve type explicitly: SharedSecret<NistP384>
        let shared_secret: SharedSecret<NistP384> = diffie_hellman(
            local_private_key.as_nonzero_scalar(),
            remote_public_key.as_affine(),
        );

        // Hash(token || shared_secret)
        let mut hasher = Sha256::new();
        hasher.update(token);
        hasher.update(shared_secret.raw_secret_bytes());
        let result = hasher.finalize();

        let mut secret = [0u8; 32];
        secret.copy_from_slice(&result);
        secret
    }

    pub fn create_handshake_jwt(signing_key: &SigningKey, token: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        // Create salt claim
        // let claims = Claims {
        //     salt: STANDARD.encode(token),
        // };

        // Encode private key to DER for jwt crate
        // let binding = signing_key.to_pkcs8_der()?;
        // let der = binding.as_bytes();

        // Create a custom header
        // Set "x5u" header with base64-encoded public key (not standard, but matching your Java intent)
        let public_key = signing_key.verifying_key().to_public_key_der().unwrap();
        let x5u = STANDARD.encode(public_key);
        // header.x5u = Some(STANDARD.encode(public_key));

        let header = CustomHeader {
            alg: "ES384".to_string(),
            x5u
        };

        // Payload
        let payload = JwtPayload {
            salt: STANDARD.encode(token),
        };

        let priv_der = signing_key.to_pkcs8_der().unwrap();
        let encoding_key = EncodingKey::from_ec_der(priv_der.as_bytes());

        // Encode manually: `encode` accepts a raw JSON string as header if needed
        let header_json = serde_json::to_string(&header).unwrap();
        let jwt = encode(&serde_json::from_str(&header_json).unwrap(), &payload, &encoding_key)
            .expect("JWT generation failed");
        // Encode the JWT
        // let jwt = encode(
        //     &header,
        //     &claims,
        //     &EncodingKey::from_ec_der(der),
        // )?;

        Ok(jwt)
    }
}