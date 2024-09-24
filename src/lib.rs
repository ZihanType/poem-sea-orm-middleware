#![doc = include_str!("../README.md")]

pub const DEFAULT_DATA_SOURCE: &str = "default";

tokio::task_local! {
    pub static DATA_SOURCES: std::sync::Arc<DataSources>;
}

mod data_source;
mod data_sources;
mod error;
mod function;
mod middleware;
mod transaction;

pub use self::{
    data_source::*, data_sources::*, error::*, function::*, middleware::*, transaction::*,
};
