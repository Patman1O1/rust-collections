// ── Core Aliases ─────────────────────────────────────────────────────────────
use core::{
    ptr::{self}
};

// ── Super Aliases ────────────────────────────────────────────────────────────
use super::{
    IsZero,
    Vec
};

// ── Crate Aliases ────────────────────────────────────────────────────────────
use crate::{
    alloc::{Allocator},
    raw_vec::{RawVec}
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `trait SpecFromElem` Definition ──────────────────────────────────────────
pub(super) trait SpecFromElem: Sized {
    // ── Functions ───────────────────────────────────────────────────────────
    fn from_elem<A: Allocator>(elem: Self, n: usize, alloc: A) -> Vec<Self, A>;
}

// ── `SpecFromElem for T` Implementation ──────────────────────────────────────
// where
//      T: Clone
impl<T: Clone> SpecFromElem for T {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    default fn from_elem<A: Allocator>(
        elem: Self,
        n: usize,
        alloc: A
    ) -> Vec<Self, A> { todo!(); }
}

// ── `SpecFromElem for T` Implementation ──────────────────────────────────────
// where
//      T: Clone + IsZero
impl<T: Clone + IsZero> SpecFromElem for T {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    default fn from_elem<A: Allocator>(
        elem: T,
        n: usize,
        alloc: A
    ) -> Vec<T, A> { todo!(); }
}

// ── `SpecFromElem for i8` Implementation ─────────────────────────────────────
impl SpecFromElem for i8 {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from_elem<A: Allocator>(elem: i8, n: usize, alloc: A) -> Vec<i8, A> {
        todo!();
    }
}

// ── `SpecFromElem for u8` Implementation ─────────────────────────────────────
impl SpecFromElem for u8 {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from_elem<A: Allocator>(elem: u8, n: usize, alloc: A) -> Vec<u8, A> {
        todo!();
    }
}

// ── `SpecFromElem for ()` Implementation ─────────────────────────────────────
impl SpecFromElem for () {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from_elem<A: Allocator>(_elem: (), n: usize, alloc: A) -> Vec<(), A> {
        todo!();
    }
}
