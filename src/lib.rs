//! The 'ov' crate provides a collection of traits that allow you to chain off of anything.
//! Each trait has a single method with the same name as the trait (but in snake case).
//!
//! [`Over`](trait.Over.html), [`OverRef`](trait.OverRef.html), and [`OverMut`](trait.OverMut.html)
//! are each of `self`, `&self`, and `&mut self`, and the callback receives that same value.
//! They are implemented for all types.
//!
//! [`OverDeref`](trait.OverDeref.html) and [`OverDerefMut`](trait.OverDerefMut.html) are implemented
//! for types which have `Deref` and `DerefMut` implementations. They both borrow the receiver,
//! and pass a reference of the `Deref::target` to the callback.
//!
//! # Examples
//!
//! ```
//! use ov::*;
//! let mut n = 5;
//! assert_eq!(n.over(|n| n * 2), 10);
//! n.over_mut(|n| {
//!   *n *= 3
//! });
//! assert_eq!(n, 15);
//!
//! let s = String::from("Hello, world!");
//! // Note: this would fail if `s` is `String` or `&String`
//! let len = s.over_deref(|s| str::len(s));
//! assert_eq!(len, 13);
//! ```

use std::ops::Deref;
use std::ops::DerefMut;

pub trait Over: Sized {
    fn over<F, Ret>(self, f: F) -> Ret
    where
        F: FnOnce(Self) -> Ret,
    {
        f(self)
    }
}

impl<T> Over for T {}

pub trait OverRef {
    fn over_ref<F, Ret>(&self, f: F) -> Ret
    where
        F: FnOnce(&Self) -> Ret,
    {
        f(self)
    }
}

impl<T> OverRef for T {}

pub trait OverMut {
    fn over_mut<F, Ret>(&mut self, f: F) -> Ret
    where
        F: FnOnce(&mut Self) -> Ret,
    {
        f(self)
    }
}

impl<T> OverMut for T {}

pub trait OverDeref: Deref {
    fn over_deref<F, Ret>(&self, f: F) -> Ret
    where
        F: FnOnce(&<Self as Deref>::Target) -> Ret;
}

impl<T> OverDeref for T
where
    T: Deref,
{
    fn over_deref<F, Ret>(&self, f: F) -> Ret
    where
        F: FnOnce(&Self::Target) -> Ret,
    {
        f(Deref::deref(self))
    }
}

pub trait OverDerefMut: DerefMut {
    fn over_deref_mut<F, Ret>(&mut self, f: F) -> Ret
    where
        F: FnOnce(&mut Self::Target) -> Ret;
}

impl<T, DerefTarget> OverDerefMut for T
where
    T: DerefMut,
    T: Deref<Target = DerefTarget>,
{
    fn over_deref_mut<F, Ret>(&mut self, f: F) -> Ret
    where
        F: FnOnce(&mut DerefTarget) -> Ret,
    {
        f(DerefMut::deref_mut(self))
    }
}
