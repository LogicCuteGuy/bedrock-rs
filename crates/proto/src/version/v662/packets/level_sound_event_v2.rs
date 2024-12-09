use crate::version::v662::enums::Puv;
use bedrockrs_core::Vec3;
use bedrockrs_macros::gamepacket;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::{ProtoCodec, ProtoCodecLE, ProtoCodecVAR};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use varint_rs::{VarintReader, VarintWriter};

#[gamepacket(id = 120)]
pub struct LevelSoundEventPacketV2 {
    pub event_id: Puv::Legacy::LevelSoundEvent,
    #[endianness(le)]
    pub position: Vec3<f32>,
    #[endianness(var)]
    pub data: i32,
    pub actor_identifier: String,
    pub baby_mob: bool,
    pub global: bool,
}

impl ProtoCodec for LevelSoundEventPacketV2 {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let mut event_id_stream: Vec<u8> = Vec::new();
        <Puv::Legacy::LevelSoundEvent as ProtoCodec>::proto_serialize(
            &self.event_id,
            &mut event_id_stream,
        )?;
        let mut event_id_cursor = Cursor::new(event_id_stream.as_slice());

        stream.write_i8(event_id_cursor.read_u32_varint()? as i8)?;
        <Vec3<f32> as ProtoCodecLE>::proto_serialize(&self.position, stream)?;
        <i32 as ProtoCodecVAR>::proto_serialize(&self.data, stream)?;
        <String as ProtoCodec>::proto_serialize(&self.actor_identifier, stream)?;
        <bool as ProtoCodec>::proto_serialize(&self.baby_mob, stream)?;
        <bool as ProtoCodec>::proto_serialize(&self.global, stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let mut event_id_stream: Vec<u8> = Vec::new();
        event_id_stream.write_u32_varint(stream.read_i8()? as u32)?;
        let mut event_id_cursor = Cursor::new(event_id_stream.as_slice());

        let event_id =
            <Puv::Legacy::LevelSoundEvent as ProtoCodec>::proto_deserialize(&mut event_id_cursor)?;
        let position = <Vec3<f32> as ProtoCodecLE>::proto_deserialize(stream)?;
        let data = <i32 as ProtoCodecVAR>::proto_deserialize(stream)?;
        let actor_identifier = <String as ProtoCodec>::proto_deserialize(stream)?;
        let baby_mob = <bool as ProtoCodec>::proto_deserialize(stream)?;
        let global = <bool as ProtoCodec>::proto_deserialize(stream)?;

        Ok(Self {
            event_id,
            position,
            data,
            actor_identifier,
            baby_mob,
            global,
        })
    }

    fn get_size_prediction(&self) -> usize {
        self.event_id.get_size_prediction()
            + self.position.get_size_prediction()
            + self.data.get_size_prediction()
            + self.actor_identifier.get_size_prediction()
            + self.baby_mob.get_size_prediction()
            + self.global.get_size_prediction()
    }
}

// VERIFY: ProtoCodec impl
