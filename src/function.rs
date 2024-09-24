use std::sync::Arc;

use crate::{DataSources, Error, Transaction, DATA_SOURCES, DEFAULT_DATA_SOURCE};

#[inline(always)]
pub async fn default_txn() -> Result<Transaction, Error> {
    current_txn(DEFAULT_DATA_SOURCE).await
}

pub async fn current_txn(name: &str) -> Result<Transaction, Error> {
    let txn = data_sources()?
        .get(name)
        .ok_or_else(|| Error::NotFoundDataSource { name: name.into() })?
        .current_txn()
        .await?;

    Ok(txn)
}

pub async fn create_txn(name: &str) -> Result<Transaction, Error> {
    let txn = data_sources()?
        .get(name)
        .ok_or_else(|| Error::NotFoundDataSource { name: name.into() })?
        .create_txn()
        .await?;

    Ok(txn)
}

pub async fn commit(txn: Transaction) -> Result<(), Error> {
    let data_sources = data_sources()?;

    let source = data_sources
        .get(&txn.name)
        .ok_or_else(|| Error::NotFoundDataSource {
            name: txn.name.as_ref().into(),
        })?;

    source.commit(txn).await
}

pub async fn rollback(txn: Transaction) -> Result<(), Error> {
    let data_sources = data_sources()?;

    let source = data_sources
        .get(&txn.name)
        .ok_or_else(|| Error::NotFoundDataSource {
            name: txn.name.as_ref().into(),
        })?;

    source.rollback(txn).await
}

pub fn data_sources() -> Result<Arc<DataSources>, Error> {
    DATA_SOURCES
        .try_with(Arc::clone)
        .map_err(|_| Error::NotSetDataSources)
}
