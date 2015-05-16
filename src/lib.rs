#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # downcast
//!
//! An extension trait providing downcasting on all traits which
//! implement `unsafe_any::UnsafeAnyExt`.
//!

extern crate typeable;
extern crate unsafe_any as uany;

use typeable::Typeable;
use uany::UnsafeAnyExt;

/// An extension trait providing checked downcasting on trait objects.
pub trait Downcast: Typeable + UnsafeAnyExt {
    /// Returns true if the boxed type is the same as `T`.
    fn is<T: Typeable>(&self) -> bool {
        self.get_type() == TypeId::of::<T>()
    }

    /// Returns a reference to the boxed value if it is of type `T` or `None`
    /// if it isn't.
    fn downcast_ref<T: Typeable>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(unsafe { self.downcast_ref_unchecked() })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the boxed value if it is of type `T` or
    /// `None` if it isn't.
    fn downcast_mut<T: Typeable>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            Some(unsafe { self.downcast_mut_unchecked() })
        } else {
            None
        }
    }

    /// Returns the underlying boxed value if it is of type `T` or the
    /// trait object itself if it isn't.
    fn downcast<T: Typeable>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.is::<T>() {
            Some(unsafe { self.downcast_unchecked() })
        } else {
            None
        }
    }
}

impl<T: ?Sized + UnsafeAnyExt, Typeable> Downcast for T { }

