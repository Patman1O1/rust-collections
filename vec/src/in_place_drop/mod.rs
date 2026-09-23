// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    marker::{PhantomData},
    ptr::{NonNull}
};

// ── Crate Aliases ───────────────────────────────────────────────────────────
use crate::{
    alloc::{Global},
    raw_vec::{RawVec}
};

// ── `struct InPlaceDrop<T>` Definition ──────────────────────────────────────
pub(super) struct InPlaceDrop<T> {
    pub(super) inner: *mut T,
    pub(super) dst: *mut T
}

// ── `struct InPlaceDstDataSrcBufDrop<Src, Dest>` Definition ─────────────────
pub(super) struct InPlaceDstDataSrcBufDrop<Src, Dest> {
    pub(super) ptr: NonNull<Dest>,
    pub(super) len: usize,
    pub(super) src_cap: usize,
    pub(super) src: PhantomData<Src>
}

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `InPlaceDrop<T>` Implementation ─────────────────────────────────────────
impl<T> InPlaceDrop<T> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn len(&self) -> usize { todo!(); }
}

// ── `Drop for InPlaceDrop<T>` Implementation ────────────────────────────────
impl<T> Drop for InPlaceDrop<T> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn drop(&mut self) { todo!(); }
}

// ── `Drop for InPlaceDstDataSrcBufDrop<Src, Dest>` Implementation ───────────
impl<Src, Dest> Drop for InPlaceDstDataSrcBufDrop<Src, Dest> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn drop(&mut self) { todo!(); }
}
