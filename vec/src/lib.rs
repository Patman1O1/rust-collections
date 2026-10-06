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
mod alloc_init;

// Tests
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
    pub const fn as_ptr(&self) -> *const T { self.ptr.as_ptr() }

    pub const fn as_mut_ptr(&mut self) -> *mut T { self.ptr.as_ptr() }

    pub const fn as_non_null(&self) -> NonNull<T> { self.ptr }

    pub const fn as_slice(&self) -> &[T] {
        // SAFETY: `self.ptr` is non-null and aligned.
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY `self.ptr` is non-null and aligned.
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }
}
