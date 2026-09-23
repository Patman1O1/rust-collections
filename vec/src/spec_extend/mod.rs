// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    clone::{TrivialClone},
    iter::{TrustedLen},
    slice::{self}
};

// ── Super Aliases ───────────────────────────────────────────────────────────
use super::{
    IntoIter,
    Vec
};

// ── Crate Aliases ───────────────────────────────────────────────────────────
use crate::{
    alloc::{Allocator}
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `trait SpecExtend<T, I>` Definition ─────────────────────────────────────
pub(super) trait SpecExtend<T, I> {
    // ── Methods ─────────────────────────────────────────────────────────────
    fn spec_extend(&mut self, iter: I);
}

// ── `SpecExtend<T, I> for Vec<T, A>` Implementation ─────────────────────────
// where
//      I: Iterator<Item = T>
//      A: Allocator
impl<T, I, A: Allocator> SpecExtend<T, I> for Vec<T, A>
where
    I: Iterator<Item = T>,
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    default fn spec_extend(&mut self, iter: I) { todo!(); }
}

// ── `SpecExtend<T, I> for Vec<T, A>` Implementation ─────────────────────────
// where
//      I: TrustedLen<Item = T>
//      A: Allocator
impl<T, I, A: Allocator> SpecExtend<T, I> for Vec<T, A>
where
    I: TrustedLen<Item = T>,
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    default fn spec_extend(&mut self, iterator: I) { todo!(); }
}

// ── `SpecExtend<T, IntoIter<T, A2>> for Vec<T, A1>` Implementation ──────────
// where
//      A1: Allocator
//      A2: Allocator
impl<
    T,
    A1: Allocator,
    A2: Allocator
> SpecExtend<T, IntoIter<T, A2>> for Vec<T, A1> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn spec_extend(&mut self, iterator: IntoIter<T, A2>) { todo!(); }
}

// ── `SpecExtend<&'a T, I> for Vec<T, A>` Implementation ─────────────────────
// where
//      T: 'a + Clone
//      I: Iterator<Item = &'a T>
//      A: Allocator
impl<'a, T: 'a, I, A: Allocator> SpecExtend<&'a T, I> for Vec<T, A>
where
    I: Iterator<Item = &'a T>,
    T: Clone,
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    default fn spec_extend(&mut self, iterator: I) { todo!(); }
}

// ── `SpecExtend<&'a T, slice::Iter<'a, T>> for Vec<T, A>` Implementation ────
// where
//      T: 'a + TrivialClone
//      A: Allocator
impl<
    'a,
    T: 'a,
    A: Allocator
> SpecExtend<&'a T, slice::Iter<'a, T>> for Vec<T, A>
where
    T: TrivialClone,
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn spec_extend(&mut self, iterator: slice::Iter<'a, T>) { todo!(); }
}
