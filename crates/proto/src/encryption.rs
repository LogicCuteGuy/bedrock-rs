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
use sha2::{Digest, Sha256};
use bedrockrs_proto_core::error::EncryptionError;
use aes_gcm::KeyInit;

#[derive(Debug, Clone)]
pub struct Encryption {
    recv_counter: u64,
    send_counter: u64,
    buf: [u8; 8],
    key: Vec<u8>,
    iv: Vec<u8>
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
    pub fn new() -> Self {
        let key = vec![0u8; 32]; // Initialize with 32 zero bytes (can be set to your desired key)

        let cipher = Aes256Gcm::new(&Key::<Aes256Gcm>::from_slice(&key));

        Encryption{
            recv_counter: 0,
            send_counter: 0,
            buf: [0; 8],
            key: Vec::new(),
            iv: Vec::new()
        }
    }

    pub fn decrypt(&mut self, ciphertext: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
        // In bedrock dedicated server, there are serveral encryption method, but it turned out they are likely to always use
        // AES-256-GCM
        // let nonce = Nonce::from_slice(&self.iv);

        // self.cipher.decrypt(nonce, ciphertext.as_slice())
        println!("de");
        unimplemented!()
    }

    pub fn encrypt(&mut self, plaintext: Vec<u8>) -> Result<Vec<u8>, EncryptionError> {
        // let nonce = Nonce::from_slice(&self.iv);

        // self.cipher.encrypt(nonce, plaintext.as_slice())
        println!("en");
        unimplemented!()
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

    pub fn create_key_pair() -> (EphemeralSecret, PublicKey) {
        let secret = EphemeralSecret::random(&mut OsRng);
        let public = PublicKey::from(&secret);
        (secret, public)
    }

    pub fn get_secret_key(
        local_secret: &EphemeralSecret,
        remote_public: &PublicKey,
        token: &[u8],
    ) -> [u8; 32] {
        let binding = local_secret.diffie_hellman(remote_public);
        let shared_secret = binding.raw_secret_bytes();
        let mut hasher = Sha256::new();
        hasher.update(token);
        hasher.update(shared_secret);
        let result = hasher.finalize();
        let mut secret = [0u8; 32];
        secret.copy_from_slice(&result);
        secret
    }

    pub fn create_handshake_jwt(signing_key: &SigningKey, token: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        // Create salt claim
        let claims = Claims {
            salt: STANDARD.encode(token),
        };

        // Encode private key to DER for jwt crate
        let binding = signing_key.to_pkcs8_der()?;
        let der = binding.as_bytes();

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