use std::sync::Arc;

use poem::{
    error::InternalServerError, http::StatusCode, Endpoint, Error, IntoResponse, Middleware,
    Request, Response, Result,
};
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

tokio::task_local! {
    pub static TXN: Arc<DatabaseTransaction>;
}

pub struct TxnMiddleware {
    conn: DatabaseConnection,
}

impl TxnMiddleware {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl<E: Endpoint> Middleware<E> for TxnMiddleware {
    type Output = TxnEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        TxnEndpoint {
            ep,
            conn: self.conn.clone(),
        }
    }
}

pub struct TxnEndpoint<E> {
    ep: E,
    conn: DatabaseConnection,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for TxnEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let txn = Arc::new(self.conn.begin().await.map_err(InternalServerError)?);

        let result = TXN
            .scope(txn.clone(), async move { self.ep.call(req).await })
            .await;

        let Ok(txn) = Arc::try_unwrap(txn) else {
            return Err(Error::from_string("TXN has more than one strong reference", StatusCode::INTERNAL_SERVER_ERROR));
        };

        match result {
            Ok(output) => {
                let response = output.into_response();
                if response.is_success() {
                    txn.commit().await.map_err(InternalServerError)?;
                } else {
                    txn.rollback().await.map_err(InternalServerError)?;
                }
                Ok(response)
            }
            Err(e) => {
                txn.rollback().await.map_err(InternalServerError)?;
                Err(e)
            }
        }
    }
}
