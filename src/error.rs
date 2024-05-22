use std::sync::Arc;

use poem::{error::ResponseError, http::StatusCode};
use sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    DbErr(#[from] DbErr),

    #[error("data source `{name}` has more than one strong reference to its transaction")]
    TransactionReferencesError { name: Arc<str> },

    #[error("not found a data source `{name}`")]
    NotFoundDataSourceError { name: Box<str> },

    #[error("not set data sources in the current context")]
    NotSetDataSourcesError,

    #[error("data source `{name}` no transactions to commit")]
    NoTransactionsToCommit { name: Arc<str> },

    #[error("data source `{name}` no transactions to rollback")]
    NoTransactionsToRollback { name: Arc<str> },
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
