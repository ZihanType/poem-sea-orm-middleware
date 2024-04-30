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
    function::{current_txn, default_txn, new_txn},
    middleware::{SeaOrmEndpoint, SeaOrmMiddleware},
    transaction::Transaction,
};
