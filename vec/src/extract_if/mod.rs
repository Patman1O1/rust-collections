// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    fmt::{self},
    ops::{
        Range,
        RangeBounds
    },
    ptr::{self},
    slice::{self}
};

// ── Super Aliases ───────────────────────────────────────────────────────────
use super::{
    Vec
};

// ── Crate Aliases ───────────────────────────────────────────────────────────
use crate::{
    alloc::{
        Allocator,
        Global
    }
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `struct ExtractIf<'a, T, F, A>` Definition ──────────────────────────────
pub struct ExtractIf<'a, T, F, A: Allocator = Global> {
    vec: &'a mut Vec<T, A>,
    idx: usize,
    end: usize,
    del: usize,
    old_len: usize,
    pred: F
}

// ── `ExtractIf<'a, T, F, A>` Implementation ─────────────────────────────────
// where
//      A: Allocator
impl<'a, T, F, A: Allocator> ExtractIf<'a, T, F, A> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    pub(super) fn new<R: RangeBounds<usize>>(
        vec: &'a mut Vec<T, A>,
        pred: F,
        range: R
    ) -> Self { todo!(); }

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub fn allocator(&self) -> &A { todo!(); }
}

// ── `Iterator for ExtractIf<'_, T, F, A>` Implementation ────────────────────
// where
//      F: FnMut(&mut T) -> bool
//      A: Allocator
impl<T, F, A: Allocator> Iterator for ExtractIf<'_, T, F, A>
where
    F: FnMut(&mut T) -> bool,
{
    // ── Types ───────────────────────────────────────────────────────────────
    type Item = T;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn next(&mut self) -> Option<T> { todo!(); }

    // TODO
    fn size_hint(&self) -> (usize, Option<usize>) { todo!(); }
}

// ── `Drop for ExtractIf<'_, T, F, A>` Implementation ────────────────────────
// where
//      A: Allocator
impl<T, F, A: Allocator> Drop for ExtractIf<'_, T, F, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn drop(&mut self) { todo!(); }
}

// ── `Debug for ExtractIf<'_, T, F, A>` Implementation ───────────────────────
// where
//      T: Debug
//      A: Allocator
impl<T, F, A> fmt::Debug for ExtractIf<'_, T, F, A>
where
    T: fmt::Debug,
    A: Allocator,
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}
