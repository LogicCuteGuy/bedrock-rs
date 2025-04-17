
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecVAR, ProtoCodecBE};
use std::io::{Cursor, Read};
use std::mem::size_of;
use base64::Engine;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use serde_json::{Map, Value};
use uuid::Uuid;
use bedrockrs_proto_core::error::ProtoCodecError;
use crate::client_chain_data::ClientChainData;
use crate::v662::types::SerializedSkin;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use base64::{engine::general_purpose, Engine as _};
use p256::{
    ecdsa::{VerifyingKey, signature::Verifier, Signature},
    pkcs8::DecodePublicKey,
};

#[gamepacket(id = 1)] // Replace with the actual packet ID used for LoginPacket
#[derive(Clone, Debug)]
pub struct LoginPacket {
    pub username: String,
    pub title_id: String,
    pub protocol: i32,
    pub client_uuid: Uuid,
    pub client_id: i64,
    pub skin: SerializedSkin,
    pub issue_unix_time: i64,

    pub client_chain_data: ClientChainData
}

impl ProtoCodec for LoginPacket {
    fn proto_serialize(&self, _stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {

        let protocol = <i32 as ProtoCodecBE>::proto_deserialize(stream)?;

        // --- JWT Chain Parsing ---
        let chain_len = <u32 as ProtoCodecVAR>::proto_deserialize(stream)? as usize;
        let mut chain_data = vec![0u8; chain_len];
        stream.read_exact(&mut chain_data)?;

        let raw_jwt_json_str = String::from_utf8_lossy(&chain_data[4..]);
        let mut jwt_json_str = raw_jwt_json_str.split("\n");
        let jwt_json: Value = serde_json::from_str(jwt_json_str.next().unwrap())?;

        let mut username = String::new();
        let mut client_uuid = Uuid::nil();
        let mut issue_unix_time = 0i64;
        let mut title_id = String::new();

        let binding = vec![];
        let chains = jwt_json.get("chain").and_then(|v| v.as_array()).unwrap_or(&binding);

        for token in chains {
            let token_str = token.as_str().unwrap_or("");
            if let Some(mid) = decode_token(token_str) {
                if let Some(extra_data) = mid.get("extraData").and_then(|v| v.as_object()) {
                    if let Some(name) = extra_data.get("displayName").and_then(|v| v.as_str()) {
                        username = name.to_string();
                    }
                    if let Some(titleId) = extra_data.get("titleId").and_then(|v| v.as_str()) {
                        title_id = titleId.to_string();
                    }
                    if let Some(uuid_str) = extra_data.get("identity").and_then(|v| v.as_str()) {
                        client_uuid = Uuid::parse_str(uuid_str)?;
                    }
                    if let Some(iat) = mid.get("iat").and_then(|v| v.as_i64()) {
                        issue_unix_time = iat * 1000;
                    }
                }
            }
        }

        let skin_json = decode_token(jwt_json_str.next().unwrap()).unwrap();

        let client_id = skin_json.get("ClientRandomId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        let skin = SerializedSkin::decode(skin_json)?;

        Ok(Self {
            protocol,
            username,
            title_id,
            client_uuid,
            client_id,
            skin,
            issue_unix_time
        })
    }

    fn get_size_prediction(&self) -> usize {
        ProtoCodecVAR::get_size_prediction(&self.protocol)
            + self.username.get_size_prediction()
            + self.title_id.get_size_prediction()
            + self.client_uuid.get_size_prediction()
            + size_of::<i64>() // client_id
            + self.skin.get_size_prediction()
            + size_of::<i64>() // issue_unix_time
    }
}

fn decode_chain_data(buffer: &mut Cursor<&[u8]>) -> Result<(), ProtoCodecError> {
    let len = read_lint(buffer)? as usize;
    let mut chain_json = vec![0u8; len];
    buffer.read_exact(&mut chain_json)?;

    let chain_str = match std::str::from_utf8(&chain_json) {
        Ok(v) => v,
        Err(e) => {
            return Err(ProtoCodecError::FormatMismatch("Invalid UTF-8 string"));
        }
    };
    let parsed: HashMap<String, Vec<String>> = serde_json::from_str(chain_str)?;

    if let Some(chains) = parsed.get("chain") {
        for token in chains {
            if let Some(json) = decode_token(token) {
                if let Some(extra) = json.get("extraData") {
                    if let Some(iat) = json.get("iat").and_then(|v| v.as_i64()) {
                        println!("Issued at: {}", iat * 1000);
                    }
                    if let Some(display_name) = extra.get("displayName") {
                        println!("Username: {}", display_name);
                    }
                    if let Some(identity) = extra.get("identity") {
                        let uuid = Uuid::parse_str(identity.as_str().unwrap())?;
                        println!("Client UUID: {}", uuid);
                    }
                }
            }
        }
    }

    Ok(())
}

fn decode_token(token: &str) -> Option<Map<String, Value>> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return None;
    }

    let decoded = BASE64_STANDARD_NO_PAD.decode(parts[1]).ok()?;
    let json_str = std::str::from_utf8(&decoded).ok()?;
    serde_json::from_str::<Map<String, Value>>(json_str).ok()
}

fn read_lint(cursor: &mut Cursor<&[u8]>) -> Result<i32, ProtoCodecError> {
    <i32 as ProtoCodecVAR>::proto_deserialize(cursor)
}

pub fn verify_chain(chains: &[String]) -> Result<bool, Box<dyn std::error::Error>> {
    let mut last_key: Option<VerifyingKey> = None;
    let mut mojang_key_verified = false;
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    for (i, token) in chains.iter().enumerate() {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Ok(false);
        }

        let header_json = String::from_utf8(general_purpose::URL_SAFE_NO_PAD.decode(parts[0])?)?;
        let payload_json = String::from_utf8(general_purpose::URL_SAFE_NO_PAD.decode(parts[1])?)?;

        let header: Value = serde_json::from_str(&header_json)?;
        let payload: Value = serde_json::from_str(&payload_json)?;

        let x5u = header.get("x5u").and_then(|v| v.as_str()).ok_or("Missing x5u")?;
        let expected_key = load_public_key(x5u)?;

        // First key is self-signed
        if last_key.is_none() {
            last_key = Some(expected_key.clone());
        } else if last_key.as_ref() != Some(&expected_key) {
            return Ok(false);
        }

        if !verify_signature(&token, &expected_key)? {
            return Ok(false);
        }

        if mojang_key_verified {
            return Ok(i == chains.len() - 1); // Must be last
        }

        if expected_key == get_mojang_public_key()? {
            mojang_key_verified = true;
        }

        let exp = payload.get("exp").and_then(|v| v.as_u64()).ok_or("Invalid exp")?;
        if exp < now {
            return Ok(false);
        }

        let identity_key = payload.get("identityPublicKey").and_then(|v| v.as_str()).ok_or("Missing identityPublicKey")?;
        last_key = Some(load_public_key(identity_key)?);
    }

    Ok(mojang_key_verified)
}

fn load_public_key(base64_str: &str) -> Result<VerifyingKey, Box<dyn std::error::Error>> {
    let der = general_purpose::STANDARD.decode(base64_str)?;
    Ok(VerifyingKey::from_public_key_der(&der)?)
}

fn verify_signature(token: &str, key: &VerifyingKey) -> Result<bool, Box<dyn std::error::Error>> {
    use p256::ecdsa::{Signature, signature::Verifier};
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Ok(false);
    }

    let signing_input = format!("{}.{}", parts[0], parts[1]);
    let signature = general_purpose::URL_SAFE_NO_PAD.decode(parts[2])?;
    let signature = Signature::from_der(&signature)?;

    Ok(key.verify(signing_input.as_bytes(), &signature).is_ok())
}