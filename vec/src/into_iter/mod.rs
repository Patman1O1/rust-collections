// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    array::{self},
    fmt::{self},
    iter::{
        FusedIterator,
        InPlaceIterable,
        SourceIter,
        TrustedFused,
        TrustedLen,
        TrustedRandomAccessNoCoerce
    },
    marker::{PhantomData},
    mem::{
        ManuallyDrop,
        MaybeUninit,
        SizedTypeProperties
    },
    num::{NonZero},
    ops::{Deref},
    panic::{UnwindSafe},
    ptr::{
        self,
        NonNull
    },
    slice::{self}
};

// ── Super Aliases ───────────────────────────────────────────────────────────
use super::{
    AsVecIntoIter
};

// ── Crate Aliases ───────────────────────────────────────────────────────────
use crate::{
    alloc::{
        Allocator,
        Global
    },
    collections::{VecDeque},
    raw_vec::{RawVec}
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── Macros ──────────────────────────────────────────────────────────────────
macro non_null {
    (mut $place:expr, $t:ident) => {{
        #![allow(unused_unsafe)]
        unsafe { &mut *((&raw mut $place) as *mut NonNull<$t>) }
    }},
    ($place:expr, $t:ident) => {{
        #![allow(unused_unsafe)]
        unsafe { *((&raw const $place) as *const NonNull<$t>) }
    }}
}

// ── `struct IntoIter<T, A>` Definition ──────────────────────────────────────
pub struct IntoIter<T, A: Allocator = Global> {
    pub(super) buf: NonNull<T>,
    pub(super) phantom: PhantomData<T>,
    pub(super) cap: usize,
    pub(super) alloc: ManuallyDrop<A>,
    pub(super) ptr: NonNull<T>,
    pub(super) end: *const T
}

// ── `trait NonDrop` Definition ──────────────────────────────────────────────
#[unsafe(rustc_allow_lifetime_dependent_specialization)]
trait NonDrop {}

// ── `IntoIter<T, A>` Implementation ─────────────────────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> IntoIter<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub fn as_slice(&self) -> &[T] { todo!(); }

    // TODO
    pub fn as_mut_slice(&mut self) -> &mut [T] { todo!(); }

    // TODO
    pub fn allocator(&self) -> &A { todo!(); }

    // TODO
    fn as_raw_mut_slice(&mut self) -> *mut [T] { todo!(); }

    // TODO
    pub(super) fn forget_allocation_drop_remaining(&mut self) { todo!(); }

    // TODO
    pub(crate) fn forget_remaining_elements(&mut self) { todo!(); }

    // TODO
    pub(crate) fn forget_remaining_elements_and_dealloc(self) { todo!(); }

    // TODO
    unsafe fn dealloc_only(&mut self) { todo!(); }

    // TODO
    pub(crate) fn into_vecdeque(self) -> VecDeque<T, A> { todo!(); }
}

// ── `UnwindSafe for IntoIter<T, A>` Implementation ──────────────────────────
// where
//      T: UnwindSafe
//      A: Allocator + UnwindSafe
impl<T: UnwindSafe, A: Allocator + UnwindSafe> UnwindSafe for IntoIter<T, A> {}

// ── `Debug for IntoIter<T, A>` Implementation ───────────────────────────────
// where
//      T: Debug
//      A: Allocator
impl<T: fmt::Debug, A: Allocator> fmt::Debug for IntoIter<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

// ── `AsRef<[T]> for IntoIter<T, A>` Implementation ──────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> AsRef<[T]> for IntoIter<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn as_ref(&self) -> &[T] { todo!(); }
}

// ── `Send for IntoIter<T, A>` Implementation ────────────────────────────────
// where
//      T: Send
//      A: Allocator + Send
unsafe impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A> {}

// ── `Sync for IntoIter<T, A>` Implementation ────────────────────────────────
// where
//      T: Sync
//      A: Allocator + Sync
unsafe impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A> {}

// ── `Iterator for IntoIter<T, A>` Implementation ────────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> Iterator for IntoIter<T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Item = T;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn next(&mut self) -> Option<T> { todo!(); }

    // TODO
    fn size_hint(&self) -> (usize, Option<usize>) { todo!(); }

    // TODO
    fn advance_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        todo!();
    }

    // TODO
    fn count(self) -> usize { todo!(); }

    // TODO
    fn last(mut self) -> Option<T> { todo!(); }

    // TODO
    fn next_chunk<const N: usize>(
        &mut self
    ) -> Result<[T; N], core::array::IntoIter<T, N>> { todo!(); }

    // TODO
    fn fold<B, F>(mut self, mut accum: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    { todo!(); }

    // TODO
    fn try_fold<B, F, R>(&mut self, mut accum: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: core::ops::Try<Output = B>,
    { todo!(); }

    // TODO
    unsafe fn __iterator_get_unchecked(&mut self, i: usize) -> Self::Item
    where
        Self: TrustedRandomAccessNoCoerce,
    { todo!(); }
}

// ── `DoubleEndedIterator for IntoIter<T, A>` Implementation ─────────────────
// where
//      A: Allocator
impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn next_back(&mut self) -> Option<T> { todo!(); }

    // TODO
    fn next_chunk_back<const N: usize>(
        &mut self
    ) -> Result<[T; N], core::array::IntoIter<T, N>> { todo!(); }

    // TODO
    fn advance_back_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        todo!();
    }
}

// ── `ExactSizeIterator for IntoIter<T, A>` Implementation ───────────────────
// where
//      A: Allocator
impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn is_empty(&self) -> bool { todo!(); }
}

// ── `FusedIterator for IntoIter<T, A>` Implementation ───────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> FusedIterator for IntoIter<T, A> {}

// ── `TrustedFused for IntoIter<T, A>` Implementation ────────────────────────
// where
//      A: Allocator
unsafe impl<T, A: Allocator> TrustedFused for IntoIter<T, A> {}

// ── `TrustedLen for IntoIter<T, A>` Implementation ──────────────────────────
// where
//      A: Allocator
unsafe impl<T, A: Allocator> TrustedLen for IntoIter<T, A> {}

// ── `Default for IntoIter<T, A>` Implementation ─────────────────────────────
// where
//      A: Allocator + Default
impl<T, A> Default for IntoIter<T, A>
where
    A: Allocator + Default,
{
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn default() -> Self { todo!(); }
}

// ── `NonDrop for T` Implementation ──────────────────────────────────────────
// where
//      T: Copy
impl<T: Copy> NonDrop for T {}

// ── `TrustedRandomAccessNoCoerce for IntoIter<T, A>` Implementation ─────────
// where
//      T: NonDrop
//      A: Allocator
unsafe impl<T, A: Allocator> TrustedRandomAccessNoCoerce for IntoIter<T, A>
where
    T: NonDrop,
{
    // ── Constants ───────────────────────────────────────────────────────────
    const MAY_HAVE_SIDE_EFFECT: bool = false;
}

// ── `Clone for IntoIter<T, A>` Implementation ───────────────────────────────
// where
//      T: Clone
//      A: Allocator + Clone
impl<T: Clone, A: Allocator + Clone> Clone for IntoIter<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn clone(&self) -> Self { todo!(); }
}

// ── `Drop for IntoIter<T, A>` Implementation ────────────────────────────────
// where
//      A: Allocator
unsafe impl<#[may_dangle] T, A: Allocator> Drop for IntoIter<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn drop(&mut self) { todo!(); }
}

// ── `InPlaceIterable for IntoIter<T, A>` Implementation ─────────────────────
// where
//      A: Allocator
unsafe impl<T, A: Allocator> InPlaceIterable for IntoIter<T, A> {
    // ── Constants ───────────────────────────────────────────────────────────
    const EXPAND_BY: Option<NonZero<usize>> = NonZero::new(1);

    const MERGE_BY: Option<NonZero<usize>> = NonZero::new(1);
}

// ── `SourceIter for IntoIter<T, A>` Implementation ──────────────────────────
// where
//      A: Allocator
unsafe impl<T, A: Allocator> SourceIter for IntoIter<T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Source = Self;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    unsafe fn as_inner(&mut self) -> &mut Self::Source { todo!(); }
}

// ── `AsVecIntoIter for IntoIter<T>` Implementation ──────────────────────────
unsafe impl<T> AsVecIntoIter for IntoIter<T> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Item = T;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn as_into_iter(&mut self) -> &mut IntoIter<Self::Item> { todo!(); }
}
