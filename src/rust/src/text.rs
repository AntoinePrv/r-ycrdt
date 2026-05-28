use extendr_api::prelude::*;
use yrs::types::text::TextEvent as YTextEvent;
use yrs::{GetString as YGetString, Observable as _, Text as YText};

use crate::event;
use crate::type_conversion::IntoExtendr;
use crate::utils::{self, lifetime, ExtendrRef};
use crate::{try_read, ExtendrTransaction, Transaction};

utils::extendr_struct!(#[extendr] pub TextRef(yrs::TextRef));

#[extendr]
impl TextRef {
    pub fn len_ec(&self, transaction: &Transaction) -> Result<u32, Error> {
        try_read!(transaction, t => self.0.len(t))
    }

    pub fn insert_ec(
        &self,
        transaction: &mut Transaction,
        index: u32,
        chunk: &str,
    ) -> Result<(), Error> {
        let text = self.0.clone(); // Cheap ptr copy
        let chunk = chunk.to_string();
        transaction.with_write_mut(move |trans| text.insert(trans, index, &chunk))
    }

    pub fn push_ec(&self, transaction: &mut Transaction, chunk: &str) -> Result<(), Error> {
        let text = self.0.clone(); // Cheap ptr copy
        let chunk = chunk.to_string();
        transaction.with_write_mut(move |trans| text.push(trans, &chunk))
    }

    pub fn remove_range_ec(
        &self,
        transaction: &mut Transaction,
        index: u32,
        len: u32,
    ) -> Result<(), Error> {
        let text = self.0.clone(); // Cheap ptr copy
        transaction.with_write_mut(move |trans| text.remove_range(trans, index, len))
    }

    pub fn get_string_ec(&self, transaction: &Transaction) -> Result<String, Error> {
        try_read!(transaction, t => self.0.get_string(t))
    }

    pub fn observe_ec(&self, f: Function, key: &Robj) -> Result<(), Error> {
        event::observe_with!(self.as_ref(), observe_with, TextEvent, f, key);
        Ok(())
    }

    pub fn unobserve_ec(&self, key: &Robj) -> Result<(), Error> {
        event::unobserve_with!(self.as_ref(), unobserve, key);
        Ok(())
    }
}

utils::extendr_struct!(#[extendr] pub TextEvent(lifetime::CheckedRef<YTextEvent>));

#[extendr]
impl TextEvent {
    fn target_ec(&self) -> Result<TextRef, Error> {
        // Cloning is shallow BranchPtr copy pinting to same data.
        self.try_map(|event| event.target().clone().into())
    }

    fn delta_ec(&self, transaction: &Transaction) -> Result<Robj, Error> {
        self.try_map(|event| {
            transaction
                .try_write()
                .map(|trans| event.delta(trans).extendr())
        })
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }

    fn path_ec(&self) -> Result<Robj, Error> {
        self.try_map(|event| event.path().extendr()).and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }
}

extendr_module! {
    mod text;
    impl TextRef;
    impl TextEvent;
}
