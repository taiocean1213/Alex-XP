//! Alex-XP: macros that keep Rust's checks and hide the decisions Python does
//! not ask for (local types, move-vs-copy, and lifetimes).
//!
//! Type [`val!`], [`var!`], [`add!`], [`s!`], and [`obj!`]. Longer names
//! (`auto_let!`, `class!`) are the same ideas, kept so existing code still builds.
//!
//! Code stays in `.rs` files. **rustc** and **rust-analyzer** remain the checker.
//! Class fields and method signatures still name their types and use `&self` /
//! `&mut self`, because a struct cannot infer those.

mod auto;
mod short;
mod ownership;
mod oop;
mod drill;

#[doc(hidden)]
pub use auto::{auto_dup, auto_own};
#[doc(hidden)]
pub use ownership::__alex_copy;
