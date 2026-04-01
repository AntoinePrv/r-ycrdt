use extendr_api::prelude::*;
use yrs::types::map::MapEvent as YMapEvent;
use yrs::{
    ArrayPrelim as YArrayPrelim, Map as YMap, MapPrelim as YMapPrelim, TextPrelim as YTextPrelim,
};

use crate::event::ExtendrObservable;
use crate::type_conversion::{FromExtendr, IntoExtendr};
use crate::utils::{self, lifetime, ExtendrRef};
use crate::{try_read, ArrayRef, TextRef, Transaction};

utils::extendr_struct!(#[extendr] pub MapRef(yrs::MapRef));

#[extendr]
impl MapRef {
    pub fn len(&self, transaction: &Transaction) -> Result<u32, Error> {
        try_read!(transaction, t => self.0.len(t))
    }

    pub fn contains_key(&self, transaction: &Transaction, key: &str) -> Result<bool, Error> {
        try_read!(transaction, t => self.0.contains_key(t, key))
    }

    pub fn insert_any(
        &self,
        transaction: &mut Transaction,
        key: &str,
        obj: Robj,
    ) -> Result<(), Error> {
        let trans = transaction.try_write_mut()?;
        let any = yrs::Any::from_extendr(obj)?;
        self.0.insert(trans, key, any);
        Ok(())
    }

    pub fn insert_text(&self, transaction: &mut Transaction, key: &str) -> Result<TextRef, Error> {
        transaction
            .try_write_mut()
            .map(|trans| TextRef::from(self.0.insert(trans, key, YTextPrelim::default())))
    }

    pub fn insert_array(
        &self,
        transaction: &mut Transaction,
        key: &str,
    ) -> Result<ArrayRef, Error> {
        transaction
            .try_write_mut()
            .map(|trans| ArrayRef::from(self.0.insert(trans, key, YArrayPrelim::default())))
    }

    pub fn insert_map(&self, transaction: &mut Transaction, key: &str) -> Result<MapRef, Error> {
        transaction
            .try_write_mut()
            .map(|trans| MapRef::from(self.0.insert(trans, key, YMapPrelim::default())))
    }

    pub fn get(&self, transaction: &mut Transaction, key: &str) -> Result<Robj, Error> {
        try_read!(transaction, t => self.0.get(t, key).as_ref().extendr()).and_then(|r| r)
    }

    pub fn keys(&self, transaction: &mut Transaction) -> Result<Strings, Error> {
        try_read!(transaction, t => Strings::from_iter(self.0.keys(t)))
    }

    pub fn items(&self, transaction: &mut Transaction) -> Result<List, Error> {
        try_read!(transaction, t => {
            let n = self.0.len(t) as usize;
            let mut keys = Strings::new(n);
            let mut values = List::new(n);
            for (i, (k, v)) in self.0.iter(t).enumerate() {
                keys.set_elt(i, k.into());
                values.set_elt(i, v.extendr()?)?;
            }
            if n > 0 {
                values.set_names(keys.as_slice())?;
            }
            Ok::<List, Error>(values)
        })
        .and_then(|r| r)
    }

    pub fn remove(&self, transaction: &mut Transaction, key: &str) -> Result<(), Error> {
        transaction.try_write_mut().map(|trans| {
            self.0.remove(trans, key);
        })
    }

    pub fn clear(&self, transaction: &mut Transaction) -> Result<(), Error> {
        transaction.try_write_mut().map(|trans| self.0.clear(trans))
    }

    pub fn observe(&self, f: Function, key: &Robj) -> Result<(), Error> {
        ExtendrObservable::<MapEvent>::observe(self, f, key)
    }

    pub fn unobserve(&self, key: &Robj) -> Result<(), Error> {
        ExtendrObservable::<MapEvent>::unobserve(self, key)
    }
}

utils::extendr_struct!(#[extendr] pub MapEvent(lifetime::CheckedRef<YMapEvent>));

#[extendr]
impl MapEvent {
    fn target(&self) -> Result<MapRef, Error> {
        // Cloning is shallow BranchPtr copy pointing to same data.
        self.try_map(|event| event.target().clone().into())
    }

    fn keys(&self, transaction: &Transaction) -> Result<Robj, Error> {
        self.try_map(|event| {
            transaction
                .try_write()
                .map(|trans| event.keys(trans).extendr())
        })
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }

    fn path(&self) -> Result<Robj, Error> {
        self.try_map(|event| event.path().extendr()).and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }
}

extendr_module! {
    mod map;
    impl MapRef;
    impl MapEvent;
}
