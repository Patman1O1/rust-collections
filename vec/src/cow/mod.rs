// ── Standard Library Aliases ────────────────────────────────────────────────
use std::{
    borrow::Cow
};

// ── Super Aliases ───────────────────────────────────────────────────────────
use super::{
    Vec
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `From<&'a [T]> for Cow<'a, [T]>` Implementation ─────────────────────────
// where
//      T: Clone
impl<'a, T: Clone> From<&'a [T]> for Cow<'a, [T]> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: &'a [T]) -> Cow<'a, [T]> { todo!(); }
}

// ── `From<&'a [T; N]> for Cow<'a, [T]>` Implementation ──────────────────────
// where
//      T: Clone
//      N: usize
impl<'a, T: Clone, const N: usize> From<&'a [T; N]> for Cow<'a, [T]> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: &'a [T; N]) -> Cow<'a, [T]> { todo!(); }
}

// ── `From<Vec<T>> for Cow<'a, [T]>` Implementation ──────────────────────────
// where
//      T: Clone
impl<'a, T: Clone> From<Vec<T>> for Cow<'a, [T]> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(v: Vec<T>) -> Cow<'a, [T]> { todo!(); }
}

// ── `From<&'a Vec<T>> for Cow<'a, [T]>` Implementation ──────────────────────
// where
//      T: Clone
impl<'a, T: Clone> From<&'a Vec<T>> for Cow<'a, [T]> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(v: &'a Vec<T>) -> Cow<'a, [T]> { todo!(); }
}

// ── `FromIterator<T> for Cow<'a, [T]>` Implementation ───────────────────────
// where
//      T: Clone
impl<'a, T> FromIterator<T> for Cow<'a, [T]>
where
    T: Clone,
{
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Cow<'a, [T]> { todo!(); }
}
