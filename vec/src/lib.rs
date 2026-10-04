// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    alloc::{
        Layout
    },
    marker::PhantomData,
    mem,
    ptr::NonNull
};

use std::{
    alloc::handle_alloc_error
};

use allocator_api2::{
    alloc::{Allocator, Global}
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
fn capacity_overflow() -> ! { panic!("capacity overflow"); }

// ── `Vec<T>` Implementations ────────────────────────────────────────────────
impl<T> Vec<T> {
    // ── Functions ───────────────────────────────────────────────────────────
    pub fn new() -> Self { Self::new_in(Global) }
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
    fn grow_to(&mut self, new_cap: usize) {
        debug_assert!(!Self::IS_ZST && self.cap < new_cap);
        
        let new_layout: Layout = Layout::array::<T>(new_cap).unwrap_or_else(
            |_| { capacity_overflow() }
        );

        let result = if self.cap > 0 {
            // SAFETY: This layout was already checked during the creation
            // of the current buffer.
            let old_layout = unsafe {
                Layout::array::<T>(new_cap).unwrap_unchecked()
            };

            // SAFETY: `ptr` was allocated by `self.alloc` with `old_layout`,
            // and `new_layout` has the same alignment and a larger size.
            unsafe { self.alloc.grow(self.ptr.cast(), old_layout, new_layout) }
        } else {
            self.alloc.allocate(new_layout)
        };

        self.ptr = match result {
            Ok(buffer) => buffer.cast(),
            Err(_) => handle_alloc_error(new_layout)
        };

        self.cap = new_cap;
    }

}
