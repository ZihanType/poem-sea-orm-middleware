use std::sync::Arc;

use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};

use crate::{DataSources, DATA_SOURCES};

pub struct SeaOrmMiddleware {
    data_sources: DataSources,
}

impl SeaOrmMiddleware {
    pub fn new(data_sources: DataSources) -> Self {
        Self { data_sources }
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
    data_sources: DataSources,
    inner: E,
}

impl<E: Endpoint> Endpoint for SeaOrmEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let data_sources = Arc::new(self.data_sources.clone());

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
