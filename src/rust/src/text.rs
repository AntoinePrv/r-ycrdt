use extendr_api::prelude::*;
use yrs::types::{text::TextEvent as YTextEvent, PathSegment as YPathSegment};
use yrs::{GetString as YGetString, Observable as YObservable, Text as YText};

use crate::type_conversion::IntoExtendr;
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
                let event = TextEvent::from_ref(event);
                let mut trans: Robj = Transaction::from_ref(trans).into();
                let result = f.call(pairlist!(trans.clone(), event.get().clone()));
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
struct TextEvent(lifetime::CheckedRef<YTextEvent>);

impl TextEvent {
    fn from_ref<'a>(
        event: &'a YTextEvent,
    ) -> lifetime::Guard<'a, YTextEvent, ExternalPtr<TextEvent>> {
        lifetime::CheckedRef::new_guarded(event)
    }

    fn try_with<R>(&self, f: impl FnOnce(&YTextEvent) -> R) -> Result<R, Error> {
        self.0.map(f).ok_or_else(|| {
            Error::Other(
                concat!(
                    "Event is invalid.",
                    " This happened because you tried to capture an event in an `observe`",
                    " callback and use it afterwards."
                )
                .into(),
            )
        })
    }
}

impl lifetime::Owner<YTextEvent> for ExternalPtr<TextEvent> {
    fn wrap(r: lifetime::CheckedRef<YTextEvent>) -> Self {
        // Converting to Robj first as the converter will set the class symbol attribute,
        // otherwise it will only be seen as an `externalptr` from R.
        let robj = TextEvent(r).into_robj();
        // PANICS: Robj was just created with the proper type
        TryInto::<ExternalPtr<TextEvent>>::try_into(robj).unwrap()
    }

    fn inner(&self) -> &lifetime::CheckedRef<YTextEvent> {
        &self.as_ref().0
    }
}

#[extendr]
impl TextEvent {
    fn target(&self) -> Result<TextRef, Error> {
        // Cloning is shallow BranchPtr copy pinting to same data.
        self.try_with(|event| event.target().clone().into())
    }

    fn delta(&self, transaction: &Transaction) -> Result<List, Error> {
        self.try_with(|event| {
            transaction.try_write().map(|trans| {
                event
                    .delta(trans)
                    .iter()
                    .map(|delta| delta.extendr())
                    .collect::<Result<List, _>>()
            })
        })
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
        .and_then(|r| r) // TODO(MSRV 1.89) .flatten()
    }

    fn path(&self) -> Result<List, Error> {
        self.try_with(|event| {
            event
                .path()
                .into_iter()
                .map(|seg| match seg {
                    YPathSegment::Key(k) => Strings::from_values([k]).into_robj(),
                    YPathSegment::Index(i) => IntoRobj::into_robj(i),
                })
                .collect()
        })
    }
}

mod lifetime {
    use std::{cell::Cell, marker::PhantomData, ptr::NonNull};

    /// A reference-counted container that can hold a [`CheckedRef`].
    ///
    /// [`Owner`] abstracts over the reference-counting mechanism used to share
    /// a [`CheckedRef`] between a [`Guard`] and its callers. For example,
    /// an R [`Robj`][extendr_api::Robj] (which is internally reference-counted by R's GC)
    /// implements [`Owner`], while tests use [`Rc<CheckedRef<T>>`][std::rc::Rc].
    ///
    /// # Safety contract
    ///
    /// [`wrap`][Self::wrap] and [`inner`][Self::inner] must be inverses:
    /// [`inner`][Self::inner] **must** return a reference to the exact same [`CheckedRef`]
    /// that was passed to [`wrap`][Self::wrap]. This invariant is critical because
    /// [`Guard::drop`] calls `inner().clear()` to invalidate the pointer — if
    /// [`inner`][Self::inner] returns a different [`CheckedRef`], the real one is left
    /// dangling.
    ///
    /// This contract is verified by a `debug_assert` in [`CheckedRef::new_guarded`].
    pub(crate) trait Owner<T> {
        /// Store a [`CheckedRef`] in a new reference-counted container.
        fn wrap(r: CheckedRef<T>) -> Self;

        /// Retrieve the [`CheckedRef`] previously stored by [`wrap`][Self::wrap].
        ///
        /// Must return a reference to the same [`CheckedRef`] instance, not a copy or
        /// a different one.
        fn inner(&self) -> &CheckedRef<T>;
    }

    /// Lifetime erasure utility that converts a compile-time lifetime into a runtime check.
    ///
    /// A `CheckedRef<T>` stores a raw pointer to `T` without carrying the original lifetime.
    /// A [`Guard`] ties the pointer's validity to the original lifetime `'a`: when the guard
    /// drops (at the end of `'a` at the latest), the pointer is cleared.
    ///
    /// Access is only possible through [`map`](CheckedRef::map), whose higher-rank trait bound
    /// (`impl FnOnce(&T) -> R`) prevents the reference from escaping the closure.
    pub struct CheckedRef<T>(Cell<Option<NonNull<T>>>);

    impl<T> CheckedRef<T> {
        unsafe fn from_ref(r: &T) -> Self {
            Self(Some(NonNull::new_unchecked(r as *const T as *mut T)).into())
        }

        pub fn new_guarded<'a, O: Owner<T>>(r: &'a T) -> Guard<'a, T, O> {
            // SAFETY: The raw pointer is valid as long as 'a. The Guard is tied to 'a
            // and clears the pointer on drop. Access is only through `map()`, whose
            // HRTB (Higher-Rank Trait Bounds) prevents the reference from escaping the closure.
            unsafe {
                let reference: O = Owner::wrap(Self::from_ref(r));
                debug_assert!(
                    std::ptr::eq(&reference.inner().0, &reference.inner().0),
                    "Owner::inner() must return a stable reference (same address on repeated calls)"
                );
                debug_assert!(
                    reference.inner().0.get().is_some(),
                    "Owner::inner() must return the CheckedRef that was passed to Owner::wrap()"
                );
                Guard::<'a> {
                    reference,
                    _phantom: PhantomData,
                }
            }
        }

        pub fn map<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
            // SAFETY: The pointer is valid as long as the option contains a value.
            // The HRTB on F prevents the reference from escaping the closure.
            self.0.get().map(|ptr| f(unsafe { ptr.as_ref() }))
        }

        pub fn clear(&self) {
            self.0.set(None)
        }
    }

    #[must_use]
    pub struct Guard<'a, T, O: Owner<T>> {
        reference: O,
        _phantom: PhantomData<&'a T>,
    }

    impl<'a, T, O: Owner<T>> Drop for Guard<'a, T, O> {
        fn drop(&mut self) {
            self.reference.inner().clear();
        }
    }

    impl<'a, T, O: Owner<T>> Guard<'a, T, O> {
        pub fn get(&self) -> &O {
            &self.reference
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::rc::Rc;

        impl<T> Owner<T> for std::rc::Rc<CheckedRef<T>> {
            fn wrap(r: CheckedRef<T>) -> Self {
                Self::from(r)
            }

            fn inner(&self) -> &CheckedRef<T> {
                self.as_ref()
            }
        }

        #[test]
        fn guard_drop_invalidates_checked_ref() {
            let val = 42i32;
            let guard = CheckedRef::<i32>::new_guarded::<Rc<_>>(&val);
            let owner = guard.get().clone();
            assert_eq!(owner.map(|r| *r), Some(42));
            drop(guard);
            assert_eq!(owner.map(|r| *r), None);
        }
    }
}

extendr_module! {
    mod text;
    impl TextRef;
    impl TextEvent;
}
