use std::{collections::HashMap, sync::Arc};

use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use sea_orm::DatabaseConnection;

use crate::{DataSources, DATA_SOURCES, DEFAULT_DATA_SOURCE};

pub struct SeaOrmMiddleware {
    data_sources: HashMap<Arc<str>, DatabaseConnection>,
}

impl SeaOrmMiddleware {
    pub fn with_default(conn: DatabaseConnection) -> Self {
        let mut data_sources = HashMap::new();

        data_sources.insert(Arc::<str>::from(DEFAULT_DATA_SOURCE), conn);

        Self { data_sources }
    }

    pub fn new(map: HashMap<Arc<str>, DatabaseConnection>) -> Self {
        Self { data_sources: map }
    }
}

impl<E: Endpoint> Middleware<E> for SeaOrmMiddleware {
    type Output = SeaOrmEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        SeaOrmEndpoint {
            data_sources: self.data_sources.clone(),
            inner: ep,
        }
    }
}

pub struct SeaOrmEndpoint<E> {
    data_sources: HashMap<Arc<str>, DatabaseConnection>,
    inner: E,
}

impl<E: Endpoint> Endpoint for SeaOrmEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let data_sources = Arc::new(DataSources::new(&self.data_sources));

        let result = DATA_SOURCES
            .scope(data_sources.clone(), async { self.inner.call(req).await })
            .await;

        match result {
            Ok(output) => {
                let response = output.into_response();

                if response.is_success() {
                    data_sources.commit_all().await?;
                } else {
                    data_sources.rollback_all().await?;
                }

                Ok(response)
            }
            Err(e) => {
                data_sources.rollback_all().await?;
                Err(e)
            }
        }
    }
}
