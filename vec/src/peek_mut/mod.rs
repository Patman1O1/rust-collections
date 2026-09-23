// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    ops::{
        Deref,
        DerefMut
    }
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
    },
    fmt::{self}
};
``
// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `struct PeekMut<'a, T, A>` Definition ───────────────────────────────────
pub struct PeekMut<'a, T, A: Allocator = Global> {
    vec: &'a mut Vec<T, A>
}

// ── `PeekMut<'a, T, A>` Implementation ──────────────────────────────────────
// where
//      A: Allocator
impl<'a, T, A: Allocator> PeekMut<'a, T, A> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    pub(super) fn new(vec: &'a mut Vec<T, A>) -> Option<Self> { todo!(); }

    // TODO
    pub fn pop(this: Self) -> T { todo!(); }
}

// ── `Debug for PeekMut<'_, T, A>` Implementation ────────────────────────────
// where
//      T: Debug
//      A: Allocator
impl<T: fmt::Debug, A: Allocator> fmt::Debug for PeekMut<'_, T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

// ── `Deref for PeekMut<'a, T, A>` Implementation ────────────────────────────
// where
//      A: Allocator
impl<'a, T, A: Allocator> Deref for PeekMut<'a, T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Target = T;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn deref(&self) -> &Self::Target { todo!(); }
}

// ── `DerefMut for PeekMut<'a, T, A>` Implementation ─────────────────────────
// where
//      A: Allocator
impl<'a, T, A: Allocator> DerefMut for PeekMut<'a, T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn deref_mut(&mut self) -> &mut Self::Target { todo!(); }
}
