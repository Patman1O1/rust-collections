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
    fn allocate(&mut self) {
        debug_assert!(!Self::IS_ZST);

        // SAFETY: Self::DEFAULT_CAP is always less than isize::MAX
        let layout = unsafe {
            Layout::array::<T>(Self::DEFAULT_CAP).unwrap_unchecked()
        };

        self.ptr = self.alloc.allocate(layout).unwrap_or_else(
            |_| { handle_alloc_error(layout) }
        ).cast();
        self.cap = Self::DEFAULT_CAP;
    }
    
    fn grow(&mut self) {
        debug_assert!(!Self::IS_ZST);
        
        // SAFETY: The old layout was already checked when the
        // current buffer was allocated.
        let old_layout = unsafe {
            Layout::array::<T>(self.cap).unwrap_unchecked()
        };

        let new_cap: usize = self.cap.checked_shl(1).unwrap_or_else(|| {
            capacity_overflow()
        });

        let new_layout = Layout::array::<T>(new_cap).unwrap_or_else(|_| {
            capacity_overflow()
        });

        // SAFETY: `new_layout` was just checked for overflow and has the same
        // layout as `old_layout` and is larger.
        self.ptr = match unsafe {
            self.alloc.grow(self.ptr.cast(), old_layout, new_layout)
        } {
            Ok(buffer) => buffer.cast(),
            Err(_) => handle_alloc_error(new_layout)
        };
        self.cap = new_cap;
    }

}
