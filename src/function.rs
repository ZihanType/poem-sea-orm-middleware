use std::sync::Arc;

use crate::{transaction::Transaction, Error, DATA_SOURCES, DEFAULT_DATA_SOURCE_NAME};

pub async fn default_txn() -> Result<Transaction, Error> {
    _current_txn(DEFAULT_DATA_SOURCE_NAME).await
}

pub async fn current_txn<N: AsRef<str>>(name: N) -> Result<Transaction, Error> {
    _current_txn(name.as_ref()).await
}

pub async fn new_txn<N: AsRef<str>>(name: N) -> Result<Transaction, Error> {
    _new_txn(name.as_ref()).await
}

async fn _current_txn(name: &str) -> Result<Transaction, Error> {
    DATA_SOURCES
        .try_with(Arc::clone)
        .map_err(|_| Error::NotSetDataSourcesError)?
        .current_txn(name)
        .await
}

async fn _new_txn(name: &str) -> Result<Transaction, Error> {
    DATA_SOURCES
        .try_with(Arc::clone)
        .map_err(|_| Error::NotSetDataSourcesError)?
        .new_txn(name)
        .await
}
