// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    borrow::{ToOwned},
    clone::TrivialClone,
    cmp::{
        self,
        Ordering
    },
    fmt::{self},
    hash::{
        Hash,
        Hasher
    },
    hint::{self},
    intrinsics::{self},
    iter::{self},
    marker::{
        Destruct,
        Freeze,
        PhantomData
    },
    mem::{
        self,
        Assume,
        ManuallyDrop,
        MaybeUninit,
        SizedTypeProperties,
        TransmuteFrom
    },
    ops::{
        self,
        Index,
        IndexMut,
        Range,
        RangeBounds
    },
    ptr::{
        self,
        NonNull
    },
    slice::{
        self,
        SliceIndex
    },
    ub_checks::{self}
};

// ── Standard Libary Aliases ─────────────────────────────────────────────────
use std::{
    borrow::ToOwned,
    boxed::Box,
    collections::TryReserveError
};

// ── Self Aliases ────────────────────────────────────────────────────────────
// Public
pub use self::{
    drain::Drain,
    extract_if::ExtractIf,
    splice::Splice,
    into_iter::IntoIter,
    peek_mut::PeekMut
};

// Public (Crate Only)
pub(crate) use self::{
    in_place_collect::AsVecIntoIter,
    is_zero::IsZero
};

// Private
use self::{
    in_place_drop::{
        InPlaceDrop,
        InPlaceDstDataSrcBufDrop
    },
    set_len_on_drop::SetLenOnDrop,
    spec_from_iter_nested::SpecFromIterNested,
    spec_from_iter::SpecFromIter,
    spec_extend::SpecExtend
};

// ── Allocator API 2 Aliases ─────────────────────────────────────────────────
use allocator_api2::{
    alloc::{
        Allocator,
        Global
    }
};


use crate::borrow::{Cow}; // Need to resolve
//use crate::raw_vec::RawVec;


// ── Modules ─────────────────────────────────────────────────────────────────
mod extract_if;
mod splice;
mod drain;
mod cow;
mod into_iter;
mod is_zero;
mod in_place_collect;
mod partial_eq;
mod peek_mut;
mod spec_from_elem;
mod set_len_on_drop;
mod in_place_drop;
mod spec_from_iter_nested;
mod spec_from_iter;
mod spec_extend;
mod sve_retain;

// ── `struct Vec<T, A>` Definition ───────────────────────────────────────────
pub struct Vec<T,  A: Allocator = Global> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize
}

// ── `trait Recyclable<From>` Definition ─────────────────────────────────────
unsafe trait Recyclable<From: Sized>: Sized {}


// ── `Vec<T>` Implementation ─────────────────────────────────────────────────
impl<T> Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    pub fn new() -> Self { todo!(); }

    // TODO
    pub fn with_capacity(capacity: usize) -> Self { todo!(); }

    // TODO
    pub fn try_with_capacity(
        capacity: usize
    ) -> Result<Self, TryReserveError> { todo!(); }

    // TODO
    pub unsafe fn from_raw_parts(
        ptr: *mut T,
        length: usize,
        capacity: usize
    ) -> Self { todo!(); }

    // TODO
    pub const unsafe fn from_parts(
        ptr: NonNull<T>,
        length: usize,
        capacity: usize
    ) -> Self { todo!(); }

    // TODO
    pub fn from_fn<F>(length: usize, f: F) -> Self
    where
        F: FnMut(usize) -> T,
    { todo!(); }

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub const fn into_raw_parts(self) -> (*mut T, usize, usize) {
        todo!();
    }

    // TODO
    pub const fn into_parts(self) -> (NonNull<T>, usize, usize) {
        todo!();
    }

    // TODO
    pub const fn const_make_global(mut self) -> &'static [T]
    where
        T: Freeze,
    { todo!(); }
}

// ── `Vec<T, A>` Implementations ─────────────────────────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> Vec<T, A> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    pub fn new_in(alloc: A) -> Self { todo!(); }

    // TODO
    pub fn try_with_capacity_in(
        capacity: usize,
        alloc: A
    ) -> Result<Self, TryReserveError> { todo!(); }

    // TODO
    pub unsafe fn from_raw_parts_in(
        ptr: *mut T,
        length: usize,
        capacity: usize,
        alloc: A,
    ) -> Self { todo!(); }

    // TODO
    pub unsafe fn from_parts_in(
        ptr: NonNull<T>,
        length: usize,
        capacity: usize,
        alloc: A,
    ) -> Self { todo!(); }

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub fn into_raw_parts_with_allocator(
        self
    ) -> (*mut T, usize, usize, A) { todo!(); }

    // TODO
    pub fn into_parts_with_allocator(self) -> (NonNull<T>, usize, usize, A) {
        todo!();
    }

    // TODO
    pub const fn capacity(&self) -> usize { todo!(); }

    // TODO
    pub fn reserve(&mut self, additional: usize) { todo!(); }

    // TODO
    pub fn reserve_exact(&mut self, additional: usize) { todo!(); }

    // TODO
    pub fn try_reserve(
        &mut self,
        additional: usize
    ) -> Result<(), TryReserveError> { todo!(); }

    // TODO
    pub fn try_reserve_exact(
        &mut self,
        additional: usize
    ) -> Result<(), TryReserveError> { todo!(); }

    // TODO
    pub fn shrink_to_fit(&mut self) { todo!(); }

    // TODO
    pub fn shrink_to(&mut self, min_capacity: usize) { todo!(); }

    // TODO
    pub fn try_shrink_to_fit(&mut self) -> Result<(), TryReserveError> {
        todo!();
    }

    // TODO
    pub fn try_shrink_to(
        &mut self,
        min_capacity: usize
    ) -> Result<(), TryReserveError> { todo!(); }

    // TODO
    pub fn into_boxed_slice(mut self) -> Box<[T], A> { todo!(); }

    // TODO
    pub fn into_array<const N: usize>(self) -> Result<Box<[T; N], A>, Self> {
        todo!();
    }

    // TODO
    pub fn truncate(&mut self, len: usize) { todo!(); }

    // TODO
    pub fn as_slice(&self) -> &[T] { todo!(); }

    // TODO
    pub fn as_mut_slice(&mut self) -> &mut [T] { todo!(); }

    // TODO
    pub fn as_ptr(&self) -> *const T { todo!(); }

    // TODO
    pub const fn as_mut_ptr(&mut self) -> *mut T { todo!(); }

    // TODO
    pub const fn as_non_null(&mut self) -> NonNull<T> { todo!(); }
    
    // TODO
    pub const fn allocator(&self) -> &A { todo!(); }
    
    // TODO
    pub const unsafe fn set_len(&mut self, new_len: usize) {
        todo!();
    }

    // TODO
    pub fn swap_remove(&mut self, index: usize) -> T { todo!(); }

    // TODO
    pub fn insert(&mut self, index: usize, element: T) { todo!(); }

    // TODO
    pub fn insert_mut(&mut self, index: usize, element: T) -> &mut T {
        todo!();
    }

    // TODO
    pub fn remove(&mut self, index: usize) -> T { todo!(); }

    // TODO
    pub fn try_remove(&mut self, index: usize) -> Option<T> { todo!(); }

    // TODO
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    { todo!(); }

    // TODO
    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    { todo!(); }

    // TODO
    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    { todo!(); }

    // TODO
    pub fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    { todo!(); }

    // TODO
    pub fn push_within_capacity(&mut self, value: T) -> Result<&mut T, T> {
        todo!();
    }

    // TODO
    pub fn pop(&mut self) -> Option<T> { todo!(); }

    // TODO
    pub fn pop_if(
        &mut self,
        predicate: impl FnOnce(&mut T) -> bool
    ) -> Option<T> { todo!(); }

    // TODO
    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, T, A>> { todo!(); }

    // TODO
    pub fn append(&mut self, other: &mut Self) { todo!(); }

    // TODO
    unsafe fn append_elements(&mut self, other: *const [T]) { todo!(); }

    // TODO
    unsafe fn try_append_elements(
        &mut self,
        other: *const [T]
    ) -> Result<(), TryReserveError> { todo!(); }

    // TODO
    unsafe fn append_elements_unreserved(&mut self, other: *const [T]) {
        todo!();
    }

    // TODO
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    where
        R: RangeBounds<usize>,
    { todo!(); }

    // TODO
    pub fn clear(&mut self) { todo!(); }

    // TODO
    pub const fn len(&self) -> usize { todo!(); }

    // TODO
    pub const fn is_empty(&self) -> bool { todo!(); }

    // TODO
    pub fn split_off(&mut self, at: usize) -> Self
    where
        A: Clone,
    { todo!(); }

    // TODO
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    { todo!(); }

    // TODO
    pub fn leak<'a>(self) -> &'a mut [T]
    where
        A: 'a,
    { todo!(); }

    // TODO
    pub const fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        todo!();
    }

    // TODO
    pub const fn split_at_spare_mut(
        &mut self
    ) -> (&mut [T], &mut [MaybeUninit<T>]) { todo!(); }

    // TODO
    const unsafe fn split_at_spare_mut_with_len(
        &mut self,
    ) -> (&mut [T], &mut [MaybeUninit<T>], &mut usize) { todo!(); }

    // TODO
    pub fn into_chunks<const N: usize>(mut self) -> Vec<[T; N], A> { todo!(); }

    // TODO
    pub fn recycle<U>(mut self) -> Vec<U, A>
    where
        U: Recyclable<T>,
    { todo!(); }

    // TODO
    fn extend_desugared<I: Iterator<Item = T>>(&mut self, mut iterator: I) {
        todo!();
    }

    // TODO
    fn extend_trusted(&mut self, iterator: impl iter::TrustedLen<Item = T>) {
        todo!();
    }

    // TODO
    pub fn splice<R, I>(
        &mut self,
        range: R,
        replace_with: I
    ) -> Splice<'_, I::IntoIter, A>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    { todo!(); }

    // TODO
    pub fn extract_if<F, R>(
        &mut self,
        range: R,
        filter: F
    ) -> ExtractIf<'_, T, F, A>
    where
        F: FnMut(&mut T) -> bool,
        R: RangeBounds<usize>,
    { todo!(); }

}

// ── `Vec<T, A>` Implementations ─────────────────────────────────────────────
// where
//      A: Allocator + Destruct
impl<T, A: Allocator + Destruct> Vec<T, A> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self { todo!(); }

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub fn push(&mut self, value: T) { todo!(); }

    // TODO
    pub fn push_mut(&mut self, value: T) -> &mut T { todo!(); }
}

// ── `Vec<T: Clone, A: Allocator>` Implementation ────────────────────────────
// where
//      T: Clone
//      A: Allocator
impl<T: Clone, A: Allocator> Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub fn resize(&mut self, new_len: usize, value: T) { todo!(); }

    // TODO
    pub fn extend_from_slice(&mut self, other: &[T]) { todo!(); }

    // TODO
    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: RangeBounds<usize>,
    { todo!(); }

     // TODO
    fn extend_with(&mut self, n: usize, value: T) { todo!(); }
}

// ── `Vec<u8, A: Allocator>` Implementation ──────────────────────────────────
// where
//      A: Allocator
impl<A: Allocator> Vec<u8, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub(crate) fn try_extend_from_slice_of_bytes(
        &mut self,
        other: &[u8],
    ) -> Result<(), TryReserveError> { todo!(); }
}

// ── `Vec<[T; N], A>` Implementation ─────────────────────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator, const N: usize> Vec<[T; N], A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub fn into_flattened(self) -> Vec<T, A> { todo!(); }
}

// ── `Vec<T: PartialEq, A: Allocator>` Implementation ────────────────────────
// where
//      T: PartialEq
//      A: Allocator
impl<T: PartialEq, A: Allocator> Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub fn dedup(&mut self) { todo!(); }
}

// ── `Recyclable<From>` Implementations ──────────────────────────────────────
unsafe impl<From, To> Recyclable<From> for To
where
    for<'a> &'a MaybeUninit<To>: TransmuteFrom<&'a MaybeUninit<From>, {
        Assume::SAFETY
    }>,

    for<'a> &'a MaybeUninit<From>: TransmuteFrom<&'a MaybeUninit<To>, {
        Assume::SAFETY
    }>,
{}

// ── `Clone for Vec<T, A>` Implementation ────────────────────────────────────
// where 
//      T: Clone
//      A: Allocator + Clone
impl<T: Clone, A: Allocator + Clone> Clone for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn clone(&self) -> Self { todo!(); }

    // TODO
    fn clone_from(&mut self, source: &Self) { todo!(); }
}


// TODO
pub fn from_elem<T: Clone>(elem: T, n: usize) -> Vec<T> {
    todo!();
}

// TODO
pub fn from_elem_in<T: Clone, A: Allocator>(
    elem: T,
    n: usize,
    alloc: A
) -> Vec<T, A> { todo!(); }

// ── `ExtendFromWithinSpec for Vec<T, A>` Implementation ─────────────────────
// where
//      T:Clone
//      A: Allocator
impl<T: Clone, A: Allocator> ExtendFromWithinSpec for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    default unsafe fn spec_extend_from_within(&mut self, src: Range<usize>) {
        todo!();
    }
}

// ── `ExtendFromWithinSpec for Vec<T, A>` Implementation ─────────────────────
// where
//      T: TrivialClone
impl<T: TrivialClone, A: Allocator> ExtendFromWithinSpec for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    unsafe fn spec_extend_from_within(&mut self, src: Range<usize>) {
        todo!();
    }
}

// ── `Deref for Vec<T, A>` Implementation ────────────────────────────────────
// where
//      T: Allocator
impl<T, A: Allocator> ops::Deref for Vec<T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Target = [T];
    
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn deref(&self) -> &[T] { todo!(); }
}

// ── `DerefMut for Vec<T, A>` Implementation ─────────────────────────────────
// where
//      T: Allocator
impl<T, A: Allocator> ops::DerefMut for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn deref_mut(&mut self) -> &mut [T] { todo!(); }
}

// ── `DerefPure for Vec<T, A>` Implementation ────────────────────────────────
// where
//      T: Allocator
unsafe impl<T, A: Allocator> ops::DerefPure for Vec<T, A> {}

// ── `Hash for Vec<T, A>` Implementation ─────────────────────────────────────
// where
//      T: Hash
//      A: Allocator
impl<T: Hash, A: Allocator> Hash for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn hash<H: Hasher>(&self, state: &mut H) { todo!(); }
}

// ── `Index<I> for Vec<T, A>` Implementation ─────────────────────────────────
// where
//      I: SliceIndex<[T]>
//      A: Allocator
impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for Vec<T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Output = I::Output;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn index(&self, index: I) -> &Self::Output { todo!(); }
}

// ── `IndexMut<I> for Vec<T, A>` Implementation ──────────────────────────────
// where
//      I: SliceIndex<[T]>
//      A: Allocator
impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut<I> for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn index_mut(&mut self, index: I) -> &mut Self::Output { todo!(); }
}

// ── `FromIterator<T> for Vec<T>` Implementation ─────────────────────────────
impl<T> FromIterator<T> for Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> { todo!(); }
}

// ── `IntoIterator for Vec<T, A>` Implementation ─────────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> IntoIterator for Vec<T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Item = T;

    type IntoIter = IntoIter<T, A>;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn into_iter(self) -> Self::IntoIter { todo!(); }
}

// ── `IntoIterator for &'a Vec<T, A>` Implementation ─────────────────────────
impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Item = &'a T;

    type IntoIter = slice::Iter<'a, T>;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn into_iter(self) -> Self::IntoIter { todo!(); }
}

// ── `IntoIterator for &'a mut Vec<T, A>` Implementation ─────────────────────
impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Item = &'a mut T;

    type IntoIter = slice::IterMut<'a, T>;

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn into_iter(self) -> Self::IntoIter { todo!(); }
}

// ── `Extend<T> for Vec<T, A>` Implementation ────────────────────────────────
impl<T, A: Allocator> Extend<T> for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) { todo!(); }

    // TODO
    fn extend_one(&mut self, item: T) { todo!(); }

    // TODO
    fn extend_reserve(&mut self, additional: usize) { todo!(); }

    // TODO
    unsafe fn extend_one_unchecked(&mut self, item: T) { todo!(); }
}

// ── `Extend<&'a T> for Vec<T, A>` Implementation ────────────────────────────
impl<'a, T: Copy + 'a, A: Allocator> Extend<&'a T> for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) { todo!(); }

    // TODO
    fn extend_one(&mut self, &item: &'a T) { todo!(); }

    // TODO
    fn extend_reserve(&mut self, additional: usize) { todo!(); }

    // TODO
    unsafe fn extend_one_unchecked(&mut self, &item: &'a T) { todo!(); }
}

// ── `PartialOrder<Vec<T, A2>> for Vec<T, A1>` Implementation ────────────────
// where
//      T: PartialOrd
//      A1: Allocator
//      A2: Allocator
impl<T, A1, A2> PartialOrd<Vec<T, A2>> for Vec<T, A1>
where
    T: PartialOrd,
    A1: Allocator,
    A2: Allocator
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn partial_cmp(&self, other: &Vec<T, A2>) -> Option<Ordering> {
        todo!();
    }
}

// ── `Eq for Vec<T, A>` Implementation ───────────────────────────────────────
// where
//      T: Eq
//      A: Allocator
impl<T: Eq, A: Allocator> Eq for Vec<T, A> {}

// ── `Ord for Vec<T, A>` Implementation ──────────────────────────────────────
// where
//      T: Ord
//      A: Allocator
impl<T: Ord, A: Allocator> Ord for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn cmp(&self, other: &Self) -> Ordering { todo!(); }
}

// ── `Drop for Vec<T, A>` Implementation ─────────────────────────────────────
// where
//      T: Destruct
//      A: Allocator + Destruct
unsafe impl<
    #[may_dangle] T: Destruct,
    A: Allocator + Destruct> Drop for Vec<T, A>
{
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn drop(&mut self) { todo!(); }
}

// ── `Default for Vec<T>` Implementation ─────────────────────────────────────
impl<T> Default for Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn default() -> Vec<T> { todo!(); }
}

// ── `Debug for Vec<T, A>` Implementation ────────────────────────────────────
// where
//      T: Debug,
//      A: Allocator
impl<T: fmt::Debug, A: Allocator> fmt::Debug for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

// ── `AsRef<Vec<T, A>> for Vec<T, A>` Implementation ─────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> AsRef<Vec<T, A>> for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn as_ref(&self) -> &Vec<T, A> { todo!(); }
}

// ── `AsMut<Vec<T, A>> for Vec<T, A>` Implementation ─────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> AsMut<Vec<T, A>> for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn as_mut(&mut self) -> &mut Vec<T, A> { todo!(); }
}

// ── `AsRef<[T]> for Vec<T, A>` Implementation ───────────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> AsRef<[T]> for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn as_ref(&self) -> &[T] { todo!(); }
}

// ── `AsMut<[T]> for Vec<T, A>` Implementation ───────────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> AsMut<[T]> for Vec<T, A> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn as_mut(&mut self) -> &mut [T] { todo!(); }
}

// ── `From<&[T]> for Vec<T>` Implementation ──────────────────────────────────
// where
//      T: Clone
impl<T: Clone> From<&[T]> for Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: &[T]) -> Vec<T> { todo!(); }
}

// ── `From<&mut [T]> for Vec<T>` Implementation ──────────────────────────────
// where
//      T: Clone
impl<T: Clone> From<&mut [T]> for Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: &mut [T]) -> Vec<T> { todo!(); }
}

// ── `From<&[T; N]> for Vec<T>` Implementation ───────────────────────────────
// where
//      T: Clone
//      N: usize
impl<T: Clone, const N: usize> From<&[T; N]> for Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: &[T; N]) -> Vec<T> { todo!(); }
}

// ── `From<&mut [T; N]> for Vec<T>` Implementation ───────────────────────────
// where
//      T: Clone
//      N: usize
impl<T: Clone, const N: usize> From<&mut [T; N]> for Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: &mut [T; N]) -> Vec<T> { todo!(); }
}

// ── `From<T; N> for Vec<T>` Implementation ──────────────────────────────────
// where
//      N: usize
impl<T, const N: usize> From<[T; N]> for Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: [T; N]) -> Vec<T> { todo!(); }
}

// ── `From<Cow<'a, [T]>> for Vec<T>` Implementation ──────────────────────────
impl<'a, T> From<Cow<'a, [T]>> for Vec<T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: Cow<'a, [T]>) -> Vec<T> { todo!(); }
}

// ── `From<Box<[T], A>> for Vec<T, A>` Implementation ────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> From<Box<[T], A>> for Vec<T, A> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: Box<[T], A>) -> Self { todo!(); }
}

// ── `From<Vec<T, A>> for Box<[T], A>` Implementation ────────────────────────
// where
//      A: Allocator
impl<T, A: Allocator> From<Vec<T, A>> for Box<[T], A> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(v: Vec<T, A>) -> Self { todo!(); }
}

// ── `From<&str> for Vec<u8>` Implementation ─────────────────────────────────
impl From<&str> for Vec<u8> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn from(s: &str) -> Vec<u8> { todo!(); }
}

// ── `TryFrom<Vec<T, A>> for [T; N]` Implementation ──────────────────────────
// where
//      T: Destruct
//      A: Allocator + Destruct
impl<
    T: Destruct,
    A: Allocator + Destruct,
    const N: usize
> TryFrom<Vec<T, A>> for [T; N] {
    // ── Types ───────────────────────────────────────────────────────────────
    type Error = Vec<T, A>;
    
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    fn try_from(mut vec: Vec<T, A>) -> Result<[T; N], Vec<T, A>> {
        todo!();
    }
}
