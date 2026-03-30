use extendr_api::prelude::*;
use yrs::types::{text::TextEvent as YTextEvent, PathSegment as YPathSegment};
use yrs::{GetString as YGetString, Observable as YObservable, Text as YText};

use crate::{try_read, Origin, Transaction};

#[extendr]
pub struct TextRef(yrs::TextRef);

impl From<yrs::TextRef> for TextRef {
    fn from(value: yrs::TextRef) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for TextRef {
    type Target = yrs::TextRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[extendr]
impl TextRef {
    pub fn len(&self, transaction: &Transaction) -> Result<u32, Error> {
        try_read!(transaction, t => self.0.len(t))
    }

    pub fn insert(
        &self,
        transaction: &mut Transaction,
        index: u32,
        chunk: &str,
    ) -> Result<(), Error> {
        transaction
            .try_write_mut()
            .map(|trans| self.0.insert(trans, index, chunk))
    }

    pub fn push(&self, transaction: &mut Transaction, chunk: &str) -> Result<(), Error> {
        transaction
            .try_write_mut()
            .map(|trans| self.0.push(trans, chunk))
    }

    pub fn remove_range(
        &self,
        transaction: &mut Transaction,
        index: u32,
        len: u32,
    ) -> Result<(), Error> {
        transaction
            .try_write_mut()
            .map(|trans| self.0.remove_range(trans, index, len))
    }

    pub fn get_string(&self, transaction: &Transaction) -> Result<String, Error> {
        try_read!(transaction, t => self.0.get_string(t))
    }

    pub fn observe(&self, f: Function, key: &Robj) -> Result<(), Error> {
        if f.formals().map(|g| g.len()).unwrap_or(0) != 2 {
            return Err(Error::Other(
                "Callback expect exactly two parameters: transaction and event".into(),
            ));
        }
        self.0.observe_with(
            Origin::new(key)?,
            move |trans: &yrs::TransactionMut<'_>, event: &YTextEvent| {
                // Converting to Robj first as the converter will set the class symbol attribute,
                // otherwise it will only be seen as an `externalptr` from R.
                let mut event: Robj = TextEvent::from_ref(event).into();
                let mut trans: Robj = Transaction::from_ref(trans).into();
                let result = f.call(pairlist!(trans.clone(), event.clone()));
                // TODO make sound pattern
                TryInto::<&mut TextEvent>::try_into(&mut event)
                    .unwrap()
                    .make_unusable();
                TryInto::<&mut Transaction>::try_into(&mut trans)
                    .unwrap()
                    .unlock();
                // TODO Either take an on_error, or store it somewhere
                result.unwrap();
            },
        );
        Ok(())
    }
}

#[extendr]
struct TextEvent {
    event: Option<&'static YTextEvent>,
}

impl TextEvent {
    fn from_ref(event: &YTextEvent) -> Self {
        // TODO unsafe
        let event = unsafe { std::mem::transmute::<&YTextEvent, &YTextEvent>(event) };
        Self { event: Some(event) }
    }

    fn make_unusable(&mut self) {
        self.event = None
    }

    fn try_get(&self) -> Result<&YTextEvent, Error> {
        match self.event {
            Some(e) => Ok(e),
            None => Err(Error::Other(
                concat!(
                    "Event was is invalid.",
                    " This happened because you tried to capture an event in an `observe`",
                    " callback and use it afterwards."
                )
                .into(),
            )),
        }
    }
}

#[extendr]
impl TextEvent {
    fn path(&self) -> Result<List, Error> {
        Ok(self
            .try_get()?
            .path()
            .into_iter()
            .map(|seg| match seg {
                YPathSegment::Key(k) => Strings::from_values([k]).into_robj(),
                YPathSegment::Index(i) => IntoRobj::into_robj(i),
            })
            .collect())
    }
}

extendr_module! {
    mod text;
    impl TextRef;
    impl TextEvent;
}
