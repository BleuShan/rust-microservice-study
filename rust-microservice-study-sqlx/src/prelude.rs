//! Prelude
pub(crate) use rust_microservice_study_core::prelude::*;

pub use sqlx::{
    self,
    migrate,
    prelude::*,
    query,
    query_as,
    query_file,
    query_file_as,
    query_scalar,
};
