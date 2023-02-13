use std::sync::Arc;

use poem::{
    error::InternalServerError, http::StatusCode, Endpoint, Error, IntoResponse, Middleware,
    Request, Response, Result,
};
use sea_orm::{DatabaseConnection, TransactionTrait};

use crate::ArcTxn;

pub struct ExplicitDbMiddleware {
    conn: DatabaseConnection,
}

impl ExplicitDbMiddleware {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl<E: Endpoint> Middleware<E> for ExplicitDbMiddleware {
    type Output = ExplicitDbEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ExplicitDbEndpoint {
            ep,
            conn: self.conn.clone(),
        }
    }
}

pub struct ExplicitDbEndpoint<E> {
    ep: E,
    conn: DatabaseConnection,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for ExplicitDbEndpoint<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let txn: ArcTxn = Arc::new(self.conn.begin().await.map_err(InternalServerError)?);

        req.extensions_mut().insert(txn.clone());

        let result = self.ep.call(req).await;

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
