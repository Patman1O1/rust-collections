// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    mem::{SizedTypeProperties},
    num::{
        NonZero,
        Saturating,
        Wrapping
    }
};

// ── Crate Aliases ───────────────────────────────────────────────────────────
use crate::{
    boxed::{Box}
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `trait IsZero` Definition ───────────────────────────────────────────────
#[rustc_specialization_trait]
pub(super) unsafe trait IsZero {
    // ── Methods ─────────────────────────────────────────────────────────────
    fn is_zero(&self) -> bool;
}

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_is_zero {
    ($t:ty, $is_zero:expr) => {
        unsafe impl IsZero for $t {
            // ── Methods ─────────────────────────────────────────────────────
            // TODO
            fn is_zero(&self) -> bool { todo!(); }
        }
    };
}

// ── `macro_rules! impl_is_zero_tuples` Definition ───────────────────────────
macro_rules! impl_is_zero_tuples {
    () => {};
    ($first_arg:ident $(,$rest:ident)*) => {
        unsafe impl
            $first_arg: IsZero,
            $($rest: IsZero,)*> IsZero for ($first_arg, $($rest,)*)
        {
            // ── Methods ─────────────────────────────────────────────────────
            // TODO
            fn is_zero(&self) -> bool { todo!(); }
        }

        impl_is_zero_tuples!($($rest),*);
    }
}

// ── `macro_rules! impl_is_zero_option_of_nonzero_int` Definition ────────────
macro_rules! impl_is_zero_option_of_nonzero_int {
    ($($t:ty),+ $(,)?) => {$(
        unsafe impl IsZero for Option<NonZero<$t>> {
            // ── Methods ─────────────────────────────────────────────────────
            // TODO
            fn is_zero(&self) -> bool { todo!(); }
        }
    )+};
}

// ── `macro_rules! impl_is_zero_option_of_int` Definition ────────────────────
macro_rules! impl_is_zero_option_of_int {
    ($($t:ty),+ $(,)?) => {$(
        unsafe impl IsZero for Option<$t> {
            // ── Methods ─────────────────────────────────────────────────────
            // TODO
            fn is_zero(&self) -> bool { todo!(); }
        }
    )+};
}

// ── `macro_rules! impl_is_zero_option_of_bool` Definition ───────────────────
macro_rules! impl_is_zero_option_of_bool {
    ($($t:ty),+ $(,)?) => {$(
        unsafe impl IsZero for $t {
            // ── Methods ─────────────────────────────────────────────────────
            // TODO
            fn is_zero(&self) -> bool { todo!(); }
        }
    )+};
}

// ── `impl_is_zero!` Invocations ─────────────────────────────────────────────
impl_is_zero!((), |_: ()| true);

impl_is_zero!(i8, |x| x == 0);
impl_is_zero!(i16, |x| x == 0);
impl_is_zero!(i32, |x| x == 0);
impl_is_zero!(i64, |x| x == 0);
impl_is_zero!(i128, |x| x == 0);
impl_is_zero!(isize, |x| x == 0);

impl_is_zero!(u8, |x| x == 0);
impl_is_zero!(u16, |x| x == 0);
impl_is_zero!(u32, |x| x == 0);
impl_is_zero!(u64, |x| x == 0);
impl_is_zero!(u128, |x| x == 0);
impl_is_zero!(usize, |x| x == 0);

impl_is_zero!(bool, |x| x == false);
impl_is_zero!(char, |x| x == '\0');

impl_is_zero!(f32, |x: f32| x.to_bits() == 0);
impl_is_zero!(f64, |x: f64| x.to_bits() == 0);

// ── `IsZero for [T; N]` Implementation ──────────────────────────────────────
// where
//      N: usize
unsafe impl<T, const N: usize> IsZero for [T; N] {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    default fn is_zero(&self) -> bool { todo!(); }
}

// ── `IsZero for [T; N]` Implementation ──────────────────────────────────────
// where
//      T: IsZero
//      N: usize
unsafe impl<T: IsZero, const N: usize> IsZero for [T; N] {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn is_zero(&self) -> bool { todo!(); }
}

// ── `impl_is_zero_tuples!` Invocation ───────────────────────────────────────
impl_is_zero_tuples!(A, B, C, D, E, F, G, H);

// ── `IsZero for Option<&T>` Implementation ──────────────────────────────────
// where
//      T: ?Sized
unsafe impl<T: ?Sized> IsZero for Option<&T> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn is_zero(&self) -> bool { todo!(); }
}

// ── `IsZero for Option<Box<T>>` Implementation ──────────────────────────────
// where
//      T: ?Sized
unsafe impl<T: ?Sized> IsZero for Option<Box<T>> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn is_zero(&self) -> bool { todo!(); }
}

// ── `impl_is_zero_option_of_nonzero_int!` Invocation ────────────────────────
impl_is_zero_option_of_nonzero_int!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

// ── `impl_is_zero_option_of_int!` Invocation ────────────────────────────────
impl_is_zero_option_of_int!(
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    usize,
    isize
);

// ── `IsZero for Wrapping<T>` Implementation ─────────────────────────────────
// where
//      T: IsZero
unsafe impl<T: IsZero> IsZero for Wrapping<T> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn is_zero(&self) -> bool { todo!(); }
}

// ── `IsZero for Saturating<T>` Implementation ───────────────────────────────
// where
//      T: IsZero
unsafe impl<T: IsZero> IsZero for Saturating<T> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn is_zero(&self) -> bool { todo!(); }
}

// ── `impl_is_zero_option_of_bool!` Invocation ───────────────────────────────
impl_is_zero_option_of_bool!(
    Option<bool>,
    Option<Option<bool>>,
    Option<Option<Option<bool>>>
);
