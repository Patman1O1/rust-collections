// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    alloc::{
        Layout
    },
    hint::cold_path,
    marker::PhantomData,
    mem,
    ptr::NonNull
};

use std::{
    alloc::handle_alloc_error
};

use allocator_api2::{
    alloc::{Allocator, Global}, 
    collections::{
        TryReserveError,
        TryReserveErrorKind::{
            CapacityOverflow,
            AllocError
        }
    }
};

// ── Modules ─────────────────────────────────────────────────────────────────
// Public
pub mod drain;
pub mod extract_if;
pub mod into_iter;
pub mod peek_mut;
pub mod splice;

// Private
#[cfg(test)]
mod tests;

// ── `struct Vec<T, A>` Definition ───────────────────────────────────────────
pub struct Vec<T, A: Allocator = Global> {
    ptr: NonNull::<T>,
    cap: usize,
    len: usize,
    alloc: A,
    marker: PhantomData::<T>
}

// ── Functions ───────────────────────────────────────────────────────────────
#[cold]
#[inline(never)]
const fn capacity_overflow() -> ! { panic!("capacity overflow"); }

#[cold]
fn handle_error(e: TryReserveError) -> ! {
    match e.kind() {
        CapacityOverflow => capacity_overflow(),
        AllocError { layout, .. } => handle_alloc_error(layout)
    }
}

// ── `Vec<T>` Implementations ────────────────────────────────────────────────
impl<T> Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    pub fn new() -> Self { Self::new_in(Global) }


    // ── Methods ─────────────────────────────────────────────────────────────
    #[inline]
    pub const fn ptr(&self) -> *mut T { self.ptr.as_ptr() }
}

// ── `Vec<T, A: Allocator>` Implementations ──────────────────────────────────
impl<T, A: Allocator> Vec<T, A> {
    // ── Constants ───────────────────────────────────────────────────────────
    const DEFAULT_CAP: usize = 16;

    const IS_ZST: bool = size_of::<T>() == 0;

    // ── Functions ───────────────────────────────────────────────────────────
    pub fn new_in(alloc: A) -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: if Self::IS_ZST { isize::MAX as usize } else { 0 },
            len: 0,
            alloc,
            marker: PhantomData
        }
    }
    
    // ── Methods ─────────────────────────────────────────────────────────────
}
