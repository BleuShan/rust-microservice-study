#![forbid(future_incompatible)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]
#![feature(never_type, trait_alias, backtrace)]
#![doc = include_str!("../README.md")]

mod manager;
pub mod prelude;

pub use manager::DatabaseManager;
