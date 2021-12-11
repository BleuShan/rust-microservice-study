#![forbid(future_incompatible)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms
)]
#![feature(never_type, trait_alias, backtrace)]
#![doc = include_str!("../README.md")]

mod application;
mod prelude;
use application::Application;
use prelude::*;

#[tokio::main]
#[instrument]
async fn main() -> Result<Unit> {
    let app = Application::init()?;
    Ok(())
}
