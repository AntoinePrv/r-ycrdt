use extendr_api::prelude::*;

use yrs::sync::{Message as YMessage, SyncMessage as YSyncMessage};
use yrs::updates::{decoder::Decode as YDecode, encoder::Encode as YEncode};

use crate::type_conversion::IntoExtendr;
use crate::utils;
use crate::StateVector;

utils::extendr_struct!(#[extendr] pub SyncMessage(YSyncMessage));

#[extendr]
impl SyncMessage {
    fn decode_v1_ec(data: &[u8]) -> Result<Self, Error> {
        YSyncMessage::decode_v1(data).extendr().map(From::from)
    }

    fn decode_v2_ec(data: &[u8]) -> Result<Self, Error> {
        YSyncMessage::decode_v2(data).extendr().map(From::from)
    }

    fn from_sync_step1_ec(state_vector: &StateVector) -> Result<Self, Error> {
        Ok(Self::from(YSyncMessage::SyncStep1(
            state_vector.as_ref().clone(),
        )))
    }

    fn from_sync_step2_ec(data: &[u8]) -> Result<Self, Error> {
        Ok(Self::from(YSyncMessage::SyncStep2(data.to_vec())))
    }

    fn from_update_ec(data: &[u8]) -> Result<Self, Error> {
        Ok(Self::from(YSyncMessage::Update(data.to_vec())))
    }

    fn equal(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }

    fn not_equal(&self, other: &Self) -> bool {
        self.as_ref().ne(other.as_ref())
    }

    fn encode_v1(&self) -> Vec<u8> {
        self.as_ref().encode_v1()
    }

    fn encode_v2(&self) -> Vec<u8> {
        self.as_ref().encode_v2()
    }

    fn step(&self) -> &str {
        match self.as_ref() {
            YSyncMessage::SyncStep1(_) => "sync_step1",
            YSyncMessage::SyncStep2(_) => "sync_step2",
            YSyncMessage::Update(_) => "update",
        }
    }

    fn is_sync_step1(&self) -> bool {
        matches!(self.as_ref(), YSyncMessage::SyncStep1(_))
    }

    fn is_sync_step2(&self) -> bool {
        matches!(self.as_ref(), YSyncMessage::SyncStep2(_))
    }

    fn is_update(&self) -> bool {
        matches!(self.as_ref(), YSyncMessage::Update(_))
    }

    fn state_vector_ec(&self) -> Result<StateVector, Error> {
        match self.as_ref() {
            YSyncMessage::SyncStep1(sv) => Ok(StateVector::from(sv.clone())),
            _ => Err(Error::Other(format!(
                "Expected step to be 'sync_step1', got {}",
                self.step()
            ))),
        }
    }

    fn data_ec(&self) -> Result<Raw, Error> {
        match self.as_ref() {
            YSyncMessage::SyncStep2(data) | YSyncMessage::Update(data) => Ok(Raw::from_bytes(data)),
            _ => Err(Error::Other(format!(
                "Expected step to be 'sync_step2' or 'update`, got {}",
                self.step()
            ))),
        }
    }
}

#[extendr]
#[derive(Default)]
struct Unsupported {}

#[extendr]
impl Unsupported {
    fn new() -> Self {
        Self {}
    }
}

#[extendr]
struct Message(Robj);

impl Message {
    fn from_ymessage(msg: YMessage) -> Self {
        match msg {
            YMessage::Sync(s) => Self(SyncMessage(s).into_robj()),
            _ => Self(Unsupported {}.into_robj()),
        }
    }
}

#[extendr]
impl Message {
    fn decode_v1_ec(data: &[u8]) -> Result<Self, Error> {
        YMessage::decode_v1(data)
            .map(Self::from_ymessage)
            .map_err(|err| Error::Other(err.to_string()))
    }

    fn decode_v2_ec(data: &[u8]) -> Result<Self, Error> {
        YMessage::decode_v2(data)
            .map(Self::from_ymessage)
            .map_err(|err| Error::Other(err.to_string()))
    }

    fn new_ec(obj: Robj) -> Result<Self, Error> {
        if let Ok(m) = TryInto::<ExternalPtr<SyncMessage>>::try_into(obj.clone()) {
            Ok(Self(m.into_robj()))
        } else {
            Err(utils::make_type_error(obj, "SyncMessage"))
        }
    }

    fn inner(&self) -> Robj {
        self.0.clone()
    }

    fn is_sync_message(&self) -> bool {
        TryInto::<ExternalPtr<SyncMessage>>::try_into(self.0.clone()).is_ok()
    }

    fn encode_v1(&self) -> Vec<u8> {
        // Only one variant that we currently store inside a Message.
        let s = TryInto::<ExternalPtr<SyncMessage>>::try_into(self.0.clone()).unwrap();
        // YMessage::Sync requires ownership, so we clone the inner YSyncMessage.
        // To avoid the clone, one could swap a dummy value in, encode, then swap back.
        YMessage::Sync(s.as_ref().as_ref().clone()).encode_v1()
    }

    fn encode_v2(&self) -> Vec<u8> {
        // Only one variant that we currently store inside a Message.
        let s = TryInto::<ExternalPtr<SyncMessage>>::try_into(self.0.clone()).unwrap();
        // YMessage::Sync requires ownership, so we clone the inner YSyncMessage.
        // To avoid the clone, one could swap a dummy value in, encode, then swap back.
        YMessage::Sync(s.as_ref().as_ref().clone()).encode_v2()
    }
}

extendr_module! {
    mod message;
    impl SyncMessage;
    impl Message;
    impl Unsupported;
}
