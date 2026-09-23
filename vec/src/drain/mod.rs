// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    fmt::{self},
    iter::{
        FusedIterator,
        TrustedLen
    },
    mem::{
        self,
        ManuallyDrop,
        SizedTypeProperties
    },
    ptr::{
        self,
        NonNull
    },
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

// ── `struct Drain<'a, T, A>` Definition ─────────────────────────────────────
pub struct Drain<'a, T: 'a, A: Allocator + 'a = Global> {
    pub(super) tail_start: usize,
    pub(super) tail_len: usize,
    pub(super) iter: slice::Iter<'a, T>,
    pub(super) vec: NonNull<Vec<T, A>>
}

// ── `Drain<'a, T, A>` Implementation ────────────────────────────────────────
// where
//      A: Allocator
impl<'a, T, A: Allocator> Drain<'a, T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub fn as_slice(&self) -> &[T] { todo!(); }

    // TODO
    pub fn allocator(&self) -> &A { todo!(); }

    // TODO
    pub fn keep_rest(self) { todo!(); }
}

// ── `Debug for Drain<'_, T, A>` Implementation ──────────────────────────────
// where
//      T: Debug
//      A: Allocator
impl<T: fmt::Debug, A: Allocator> fmt::Debug for Drain<'_, T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

// ── `AsRef<[T]> for Drain<'a, T, A>` Implementation ─────────────────────────
// where
//      A: Allocator
impl<'a, T, A: Allocator> AsRef<[T]> for Drain<'a, T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn as_ref(&self) -> &[T] { todo!(); }
}

// ── `Sync for Drain<'_, T, A>` Implementation ───────────────────────────────
// where
//      T: Sync
//      A: Sync + Allocator
unsafe impl<T: Sync, A: Sync + Allocator> Sync for Drain<'_, T, A> {}

// ── `Send for Drain<'_, T, A>` Implementation ───────────────────────────────
// where
//      T: Send
//      A: Send + Allocator
unsafe impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A> {}

// ── `Iterator for Drain<'_, T, A>` Implementation ───────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> Iterator for Drain<'_, T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Item = T;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn next(&mut self) -> Option<T> { todo!(); }

    // TODO
    fn size_hint(&self) -> (usize, Option<usize>) { todo!(); }
}

// ── `DoubleEndedIterator for Drain<'_, T, A>` Implementation ────────────────
// where
//      A: Allocator
impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn next_back(&mut self) -> Option<T> { todo!(); }
}

// ── `Drop for Drain<'_, T, A>` Implementation ───────────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> Drop for Drain<'_, T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn drop(&mut self) { todo!(); }
}

// ── `ExactSizeIterator for Drain<'_, T, A>` Implementation ──────────────────
// where
//      A: Allocator
impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn is_empty(&self) -> bool { todo!(); }
}

// ── `TrustedLen for Drain<'_, T, A>` Implementation ─────────────────────────
// where
//      A: Allocator
unsafe impl<T, A: Allocator> TrustedLen for Drain<'_, T, A> {}

// ── `FusedIterator for Drain<'_, T, A>` Implementation ──────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> FusedIterator for Drain<'_, T, A> {}
