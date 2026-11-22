//! Alex-XP: macros that keep Rust's checks and hide the decisions Python does
//! not ask for (local types, move-vs-copy, and lifetimes).
//!
//! Start with the [`auto_let!`] / [`auto_own!`] / [`auto_edit!`] family in
//! [`crate::auto`]. The explicit `declare_*` / `*_lend!` macros remain when you
//! want to name the decision yourself.
//!
//! Code stays in `.rs` files. **rustc** and **rust-analyzer** remain the checker.
//! Class fields and method signatures still name their types and use `&self` /
//! `&mut self`, because a struct cannot infer those.

mod auto;
mod ownership;
mod oop;
mod drill;

#[doc(hidden)]
pub use auto::{auto_dup, auto_own};
#[doc(hidden)]
pub use ownership::__alex_copy;
