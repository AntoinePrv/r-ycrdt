use extendr_api::prelude::*;
use yrs::types::array::ArrayEvent as YArrayEvent;
use yrs::{
    Array as YArray, ArrayPrelim, MapPrelim as YMapPrelim, Observable as _,
    TextPrelim as YTextPrelim,
};

use crate::type_conversion::{FromExtendr, IntoExtendr};
use crate::utils::{self, lifetime, ExtendrRef};
use crate::{event, Prelim};
use crate::{try_read, ExtendrTransaction, MapRef, TextRef, Transaction};

utils::extendr_struct!(#[extendr] pub ArrayRef(yrs::ArrayRef));

#[extendr]
impl ArrayRef {
    pub fn len_ec(&self, transaction: &Transaction) -> Result<u32, Error> {
        try_read!(transaction, t => self.0.len(t))
    }

    pub fn insert_ec(
        &self,
        transaction: &mut Transaction,
        index: u32,
        prelim: &Prelim,
    ) -> Result<(), Error> {
        let array = self.0.clone(); // Cheap ptr copy
        let input = prelim.to_in()?;
        transaction.with_write_mut(move |trans| {
            array.insert(trans, index, input);
        })
    }

    pub fn insert_any_ec(
        &self,
        transaction: &mut Transaction,
        index: u32,
        obj: Robj,
    ) -> Result<(), Error> {
        let any = yrs::Any::from_extendr(obj)?;
        let array = self.0.clone(); // Cheap ptr copy
        transaction.with_write_mut(move |trans| {
            array.insert(trans, index, any);
        })
    }

    pub fn insert_text_ec(
        &self,
        transaction: &mut Transaction,
        index: u32,
    ) -> Result<TextRef, Error> {
        let array = self.0.clone(); // Cheap ptr copy
        transaction.with_write_mut(move |trans| {
            TextRef::from(array.insert(trans, index, YTextPrelim::default()))
        })
    }

    pub fn insert_array_ec(
        &self,
        transaction: &mut Transaction,
        index: u32,
    ) -> Result<ArrayRef, Error> {
        let array = self.0.clone(); // Cheap ptr copy
        transaction.with_write_mut(move |trans| {
            ArrayRef::from(array.insert(trans, index, ArrayPrelim::default()))
        })
    }

    pub fn insert_map_ec(
        &self,
        transaction: &mut Transaction,
        index: u32,
    ) -> Result<MapRef, Error> {
        let array = self.0.clone(); // Cheap ptr copy
        transaction.with_write_mut(move |trans| {
            MapRef::from(array.insert(trans, index, YMapPrelim::default()))
        })
    }

    pub fn get_ec(&self, transaction: &mut Transaction, index: u32) -> Result<Robj, Error> {
        try_read!(transaction, t => self.0.get(t, index).as_ref().extendr()).and_then(|r| r)
    }

    pub fn remove_ec(&self, transaction: &mut Transaction, index: u32) -> Result<(), Error> {
        let array = self.0.clone(); // Cheap ptr copy
        transaction.with_write_mut(move |trans| {
            array.remove(trans, index);
        })
    }

    pub fn observe_ec(&self, f: Function, key: &Robj) -> Result<(), Error> {
        event::observe_with!(self.as_ref(), observe_with, ArrayEvent, f, key);
        Ok(())
    }

    pub fn unobserve_ec(&self, key: &Robj) -> Result<(), Error> {
        event::unobserve_with!(self.as_ref(), unobserve, key);
        Ok(())
    }
}

utils::extendr_struct!(#[extendr] pub ArrayEvent(lifetime::CheckedRef<YArrayEvent>));

#[extendr]
impl ArrayEvent {
    pub fn target_ec(&self) -> Result<ArrayRef, Error> {
        // Cloning is shallow BranchPtr copy pointing to same data.
        self.try_map(|event| event.target().clone().into())
    }

    pub fn delta_ec(&self, transaction: &Transaction) -> Result<Robj, Error> {
        self.try_map(|event| {
            transaction
                .try_write()
                .map(|trans| event.delta(trans).extendr())
        })
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }

    pub fn path_ec(&self) -> Result<Robj, Error> {
        self.try_map(|event| event.path().extendr()).and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }

    pub fn inserts_ec(&self, transaction: &Transaction) -> Result<Robj, Error> {
        self.try_map(|event| {
            transaction
                .try_write()
                .map(|trans| event.inserts(trans).extendr())
        })
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }

    pub fn removes_ec(&self, transaction: &Transaction) -> Result<Robj, Error> {
        self.try_map(|event| {
            transaction
                .try_write()
                .map(|trans| event.removes(trans).extendr())
        })
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }
}

extendr_module! {
    mod array;
    impl ArrayRef;
    impl ArrayEvent;
}
