use crate::type_conversion::{FromExtendr, IntoExtendr};
use crate::utils;
use extendr_api::prelude::*;
use yrs::updates::{decoder::Decode as YDecode, encoder::Encode as YEncode};

utils::extendr_struct!(#[extendr] pub StateVector(yrs::StateVector));

#[extendr]
impl StateVector {
    fn decode_v1(data: &[u8]) -> Result<Self, Error> {
        Ok(Self(yrs::StateVector::decode_v1(data).extendr()?))
    }

    fn decode_v2(data: &[u8]) -> Result<Self, Error> {
        Ok(Self(yrs::StateVector::decode_v2(data).extendr()?))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn contains_client(&self, client_id: yrs::block::ClientID) -> bool {
        self.0.contains_client(&client_id)
    }

    fn equal(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn not_equal(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }

    fn less_than(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    fn less_than_equal(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    fn greater_than(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    fn greater_than_equal(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }

    fn encode_v1(&self) -> Vec<u8> {
        self.0.encode_v1()
    }

    fn encode_v2(&self) -> Vec<u8> {
        self.0.encode_v2()
    }
}

utils::extendr_struct!(#[extendr] pub DeleteSet(yrs::DeleteSet));

#[extendr]
impl DeleteSet {
    fn decode_v1(data: &[u8]) -> Result<Self, Error> {
        yrs::DeleteSet::decode_v1(data).extendr().map(Self)
    }

    fn decode_v2(data: &[u8]) -> Result<Self, Error> {
        yrs::DeleteSet::decode_v2(data).extendr().map(Self)
    }

    fn new() -> Self {
        Self(yrs::DeleteSet::new())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_deleted(&self, id: Robj) -> Result<bool, Error> {
        let id = yrs::block::ID::from_extendr(id)?;
        Ok(self.0.is_deleted(&id))
    }

    fn equal(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn not_equal(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }

    fn encode_v1(&self) -> Vec<u8> {
        self.0.encode_v1()
    }

    fn encode_v2(&self) -> Vec<u8> {
        self.0.encode_v2()
    }

    fn squash(&mut self) {
        self.0.squash()
    }

    fn merge(&mut self, other: &Self) {
        self.0.merge(other.0.clone())
    }
}

utils::extendr_struct!(#[extendr] pub Snapshot(yrs::Snapshot));

#[extendr]
impl Snapshot {
    fn new(state_map: &StateVector, delete_set: &DeleteSet) -> Self {
        yrs::Snapshot::new(state_map.0.clone(), delete_set.0.clone()).into()
    }

    fn decode_v1(data: &[u8]) -> Result<Self, Error> {
        yrs::Snapshot::decode_v1(data).extendr().map(Self)
    }

    fn decode_v2(data: &[u8]) -> Result<Self, Error> {
        yrs::Snapshot::decode_v2(data).extendr().map(Self)
    }

    fn encode_v1(&self) -> Vec<u8> {
        self.0.encode_v1()
    }

    fn encode_v2(&self) -> Vec<u8> {
        self.0.encode_v2()
    }

    fn equal(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn not_equal(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }
}

extendr_module! {
    mod state;
    impl StateVector;
    impl DeleteSet;
    impl Snapshot;
}
