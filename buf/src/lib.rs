// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    alloc::{
        Layout
    },
    hint::cold_path,
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

// ── `struct Buf<A>` Definition ──────────────────────────────────────────────
pub struct Buf<A: Allocator = Global> {
    ptr: NonNull::<[u8]>,
    cap: usize,
    len: usize,
    alloc: A
}

// ── `Buf<A: Allocator>` Implementations ─────────────────────────────────────
impl<A: Allocator> Buf<A> {
    // ── Constants ───────────────────────────────────────────────────────────
    const DEFAULT_CAP: usize = 16;

    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    pub fn new() -> Self { todo!() }

    // ── Methods ─────────────────────────────────────────────────────────────
    
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

