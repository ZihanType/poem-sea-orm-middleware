#![doc = include_str!("../README.md")]

pub(crate) const DEFAULT_DATA_SOURCE_NAME: &str = "default";

tokio::task_local! {
    pub(crate) static DATA_SOURCES: std::sync::Arc<DataSources>;
}

mod connection;
mod data_sources;
mod error;
mod function;
mod middleware;
mod transaction;

pub use self::{
    connection::Connection,
    data_sources::DataSources,
    error::Error,
    function::{commit, current_txn, data_sources, default_txn, new_txn, rollback},
    middleware::{SeaOrmEndpoint, SeaOrmMiddleware},
    transaction::Transaction,
};
