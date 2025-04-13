use std::collections::HashMap;
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecVAR, ProtoCodecLE, ProtoCodecBE};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read};
use std::mem::size_of;
use std::string::FromUtf8Error;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json::{Map, Value};
use uuid::Uuid;
use bedrockrs_proto_core::error::ProtoCodecError;
use crate::v662::types::SerializedSkin;

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
    pub buffer: Cursor<Vec<u8>>
}

impl ProtoCodec for LoginPacket {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        Ok(())
    }


    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {

        let protocol = <i32 as ProtoCodecBE>::proto_deserialize(stream)?;

        // --- JWT Chain Parsing ---
        let chain_len = <i32 as ProtoCodecLE>::proto_deserialize(stream)? as usize;
        let mut chain_data = vec![0u8; chain_len];
        stream.read_exact(&mut chain_data)?;

        let jwt_json_str = std::str::from_utf8(&chain_data).expect("JWT JSON was not valid UTF-8");
        let jwt_json: Value = serde_json::from_str(jwt_json_str)?;

        let mut username = String::new();
        let mut client_uuid = Uuid::nil();
        let mut issue_unix_time = 0i64;

        let binding = vec![];
        let chains = jwt_json.get("chain").and_then(|v| v.as_array()).unwrap_or(&binding);
        for token in chains {
            let token_str = token.as_str().unwrap_or("");
            if let Some(mid) = decode_token(token_str) {
                if let Some(extra_data) = mid.get("extraData").and_then(|v| v.as_object()) {
                    if let Some(name) = extra_data.get("displayName").and_then(|v| v.as_str()) {
                        username = name.to_string();
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

        // --- Skin Data Parsing ---
        let skin_data_len = <i32 as ProtoCodecLE>::proto_deserialize(stream)? as usize;
        let mut skin_data_buf = vec![0u8; skin_data_len];
        stream.read_exact(&mut skin_data_buf)?;
        let skin_data_str = std::str::from_utf8(&skin_data_buf).expect("JWT JSON was not valid UTF-8");
        let skin_json: Value = serde_json::from_str(skin_data_str)?;

        let client_id = skin_json.get("ClientRandomId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        let title_id = skin_json.get("TitleId")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let skin = SerializedSkin::decode(&skin_json)?; // You'll define this

        // Remaining raw buffer (if you want to keep it)
        let mut raw_buf = [0u8; 4096];
        let len = stream.read(&mut raw_buf)?;
        let buffer = Cursor::new(raw_buf[..len].try_into()?);

        Ok(Self {
            protocol,
            username,
            title_id,
            client_uuid,
            client_id,
            skin,
            issue_unix_time,
            buffer,
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
    }; ;
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

    let decoded = STANDARD.decode(parts[1]).ok()?;
    let json_str = std::str::from_utf8(&decoded).ok()?;
    serde_json::from_str::<Map<String, Value>>(json_str).ok()
}

fn read_lint(cursor: &mut Cursor<&[u8]>) -> Result<i32, ProtoCodecError> {
    <i32 as ProtoCodecVAR>::proto_deserialize(cursor)
}
