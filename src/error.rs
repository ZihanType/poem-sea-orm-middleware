use std::sync::Arc;

use poem::{error::ResponseError, http::StatusCode};
use sea_orm::DbErr;

use crate::Transaction;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    DbErr(#[from] DbErr),

    #[error("not found a data source `{name}`")]
    NotFoundDataSource { name: Box<str> },

    #[error("not set data sources in the current context")]
    NotSetDataSources,

    #[error("inconsistent data source and transaction, data source name: `{data_source_name}`, transaction name : `{transaction_name}`")]
    InconsistentDataSourceAndTransaction {
        data_source_name: Arc<str>,
        transaction_name: Arc<str>,
        txn: Transaction,
    },

    #[error("transaction have more than one reference, data source name: `{data_source_name}`, transaction hierarchy: `{transaction_hierarchy}`")]
    TransactionHaveMoreThanOneReference {
        data_source_name: Arc<str>,
        transaction_hierarchy: usize,
        txn: Transaction,
    },

    #[error("nested transaction have more than one reference, data source name: `{data_source_name}`, current transaction hierarchy: `{current_transaction_hierarchy}`, nested transaction hierarchy: `{nested_transaction_hierarchy}`")]
    NestedTransactionHaveMoreThanOneReference {
        data_source_name: Arc<str>,
        current_transaction_hierarchy: usize,
        nested_transaction_hierarchy: usize,
        txn: Transaction,
    },
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
