pub mod shard;

use crate::codec::{decode_gamepackets, encode_gamepackets};
use crate::compression::Compression;
use crate::encryption::Encryption;
use crate::error::ConnectionError;
use crate::helper::ProtoHelper;
use crate::transport::TransportLayerConnection;

// Add generic parameter <T> with ProtoHelper trait bound to the struct
pub struct Connection<T: ProtoHelper> {
    transport_layer: TransportLayerConnection,
    pub compression: Option<Compression>,
    pub encryption: Option<Encryption>,
    // PhantomData to mark the T type as used
    _marker: std::marker::PhantomData<T>,
}

// Implement methods for Connection<T> where T has ProtoHelper
impl<T: ProtoHelper> Connection<T> {
    pub(crate) fn from_transport_conn(transport_layer: TransportLayerConnection) -> Self {
        Self {
            transport_layer,
            compression: None,
            encryption: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub async fn send(
        &mut self,
        gamepackets: &[T::GamePacketType],
    ) -> Result<(), ConnectionError> {
        let gamepacket_stream = encode_gamepackets::<T>(
            gamepackets,
            self.compression.as_ref(),
            self.encryption.as_mut(),
        )?;

        self.transport_layer.send(&gamepacket_stream).await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Vec<T::GamePacketType>, ConnectionError> {
        let gamepacket_stream = self.transport_layer.recv().await?;
        let gamepackets = decode_gamepackets::<T>(
            gamepacket_stream,
            self.compression.as_ref(),
            self.encryption.as_mut(),
        )?;
        Ok(gamepackets)
    }
}

// Implement non-generic methods for Connection
impl<T: ProtoHelper> Connection<T> {
    pub async fn send_raw(&mut self, data: &[u8]) -> Result<(), ConnectionError> {
        self.transport_layer.send(data).await?;
        Ok(())
    }

    pub async fn recv_raw(&mut self) -> Result<Vec<u8>, ConnectionError> {
        let stream = self.transport_layer.recv().await?;
        Ok(stream)
    }

    pub async fn close(self) {
        self.transport_layer.close().await;
    }
}