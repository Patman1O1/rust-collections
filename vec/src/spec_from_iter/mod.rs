// ── Core Aliases ─────────────────────────────────────────────────────────────
use core::{
    mem::{ManuallyDrop},
    ptr::{self}
};

// ── Super Aliases ────────────────────────────────────────────────────────────
use super::{
    IntoIter,
    SpecExtend,
    SpecFromIterNested,
    Vec
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `trait SpecFromIter<T, I>` Definition ────────────────────────────────────
pub(super) trait SpecFromIter<T, I> {
    // ── Functions ───────────────────────────────────────────────────────────
    fn from_iter(iter: I) -> Self;
}

// ── `SpecFromIter<T, I> for Vec<T>` Implementation ───────────────────────────
// where
//      I: Iterator<Item = T>
impl<T, I> SpecFromIter<T, I> for Vec<T>
where
    I: Iterator<Item = T>,
{
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    default fn from_iter(iterator: I) -> Self { todo!(); }
}

// ── `SpecFromIter<T, IntoIter<T>> for Vec<T>` Implementation ─────────────────
impl<T> SpecFromIter<T, IntoIter<T>> for Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from_iter(iterator: IntoIter<T>) -> Self { todo!(); }
}
