// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    marker::PhantomData,
    ptr::NonNull
};

// ── Standard Libary Aliases ─────────────────────────────────────────────────

// ── Self Aliases ────────────────────────────────────────────────────────────

// ── Allocator API 2 Aliases ─────────────────────────────────────────────────
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

// ── `struct Vec<T, A> Definition` ───────────────────────────────────────────
pub struct Vec<T, A: Allocator = Global> {
    ptr: NonNull::<T>,
    cap: usize,
    len: usize,
    alloc: A,
    marker: PhantomData::<T>
}
