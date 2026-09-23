// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    alloc::{
        Allocator,
        Layout
    },
    iter::{
        InPlaceIterable,
        SourceIter,
        TrustedRandomAccessNoCoerce
    },
    marker::{PhantomData},
    mem::{
        self,
        ManuallyDrop,
        SizedTypeProperties
    },
    num::{NonZero},
    ptr::{self}
};

// ── Super Aliases ───────────────────────────────────────────────────────────
use super::{
    InPlaceDrop,
    InPlaceDstDataSrcBufDrop,
    SpecFromIter,
    SpecFromIterNested,
    Vec
};

// ── Crate Aliases ───────────────────────────────────────────────────────────
use crate::{
    alloc::{
        Global,
        handle_alloc_error
    }
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `trait InPlaceCollect` Definition ───────────────────────────────────────
#[rustc_specialization_trait]
trait InPlaceCollect: SourceIter<Source: AsVecIntoIter> + InPlaceIterable {
    // ── Types ───────────────────────────────────────────────────────────────
    type Src;
}

// ── `trait SpecInPlaceCollect<T, I>` Definition ─────────────────────────────
trait SpecInPlaceCollect<T, I>: Iterator<Item = T> {
    // ── Methods ─────────────────────────────────────────────────────────────
    unsafe fn collect_in_place(&mut self, dst: *mut T, end: *const T) -> usize;
}

// ── `trait AsVecIntoIter` Definition ────────────────────────────────────────
#[rustc_specialization_trait]
pub(crate) unsafe trait AsVecIntoIter {
    // ── Types ───────────────────────────────────────────────────────────────
    type Item;

    // ── Methods ─────────────────────────────────────────────────────────────
    fn as_into_iter(&mut self) -> &mut super::IntoIter<Self::Item>;
}

// ── Functions ───────────────────────────────────────────────────────────────
// TODO
const fn in_place_collectible<DEST, SRC>(
    step_merge: Option<NonZero<usize>>,
    step_expand: Option<NonZero<usize>>
) -> bool { todo!(); }

// TODO
const fn needs_realloc<SRC, DEST>(src_cap: usize, dst_cap: usize) -> bool {
    todo!();
}

// TODO
fn from_iter_in_place<I, T>(mut iterator: I) -> Vec<T>
where
    I: Iterator<Item = T> + InPlaceCollect,
    <I as SourceIter>::Source: AsVecIntoIter,
{ todo!(); }

// TODO
fn write_in_place_with_drop<T>(
    src_end: *const T
) -> impl FnMut(InPlaceDrop<T>, T) -> Result<InPlaceDrop<T>, !> { todo!(); }

// ── `InPlaceCollect for T` Implementation ───────────────────────────────────
// where
//      T: SourceIter<Source: AsVecIntoIter> + InPlaceIterable
impl<T> InPlaceCollect for T
where
    T: SourceIter<Source: AsVecIntoIter> + InPlaceIterable,
{
    // ── Types ───────────────────────────────────────────────────────────────
    type Src = <<T as SourceIter>::Source as AsVecIntoIter>::Item;
}

// ── `SpecFromIter<T, I> for Vec<T>` Implementation ──────────────────────────
// where
//      I: Iterator<Item = T> + InPlaceCollect
//      <I as SourceIter>::Source: AsVecIntoIter
impl<T, I> SpecFromIter<T, I> for Vec<T>
where
    I: Iterator<Item = T> + InPlaceCollect,
    <I as SourceIter>::Source: AsVecIntoIter,
{
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    default fn from_iter(iterator: I) -> Self { todo!(); }
}

// ── `SpecInPlaceCollect<T, I> for I` Implementation ─────────────────────────
// where
//      I: Iterator<Item = T>
impl<T, I> SpecInPlaceCollect<T, I> for I
where
    I: Iterator<Item = T>,
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    default unsafe fn collect_in_place(
        &mut self,
        dst_buf: *mut T,
        end: *const T
    ) -> usize { todo!(); }
}

// ── `SpecInPlaceCollect<T, I> for I` Implementation ─────────────────────────
// where
//      I: Iterator<Item = T> + TrustedRandomAccessNoCoerce
impl<T, I> SpecInPlaceCollect<T, I> for I
where
    I: Iterator<Item = T> + TrustedRandomAccessNoCoerce,
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    unsafe fn collect_in_place(
        &mut self,
        dst_buf: *mut T,
        end: *const T
    ) -> usize { todo!(); }
}
