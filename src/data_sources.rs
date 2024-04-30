use std::sync::Arc;

use scc::HashMap;
use sea_orm::DatabaseConnection;

use crate::{
    connection::Connection, error::Error, transaction::Transaction, DEFAULT_DATA_SOURCE_NAME,
};

#[derive(Debug, Default)]
pub struct DataSources(HashMap<Arc<str>, Connection>);

impl DataSources {
    pub async fn with_default(conn: DatabaseConnection) -> Self {
        let name = Arc::<str>::from(DEFAULT_DATA_SOURCE_NAME);

        let map = HashMap::new();

        let _ = map
            .insert_async(name.clone(), Connection::new(name, conn))
            .await;

        Self(map)
    }

    pub async fn insert<N: Into<Arc<str>>>(
        &self,
        name: N,
        conn: DatabaseConnection,
    ) -> Result<(), (Arc<str>, Connection)> {
        self._insert(name.into(), conn).await
    }

    pub async fn current_txn<N: AsRef<str>>(&self, name: N) -> Result<Transaction, Error> {
        self._current_txn(name.as_ref()).await
    }

    pub async fn new_txn<N: AsRef<str>>(&self, name: N) -> Result<Transaction, Error> {
        self._new_txn(name.as_ref()).await
    }

    pub async fn commit<N: AsRef<str>>(&self, name: N) -> Result<(), Error> {
        self._commit(name.as_ref()).await
    }

    pub(crate) async fn commit_all(&self) -> Result<(), Error> {
        let mut option_entry = self.0.first_entry_async().await;

        while let Some(mut entry) = option_entry {
            entry.get_mut().commit_all().await?;
            option_entry = entry.next_async().await;
        }

        Ok(())
    }

    pub async fn rollback<N: AsRef<str>>(&self, name: N) -> Result<(), Error> {
        self._rollback(name.as_ref()).await
    }

    pub(crate) async fn rollback_all(&self) -> Result<(), Error> {
        let mut option_entry = self.0.first_entry_async().await;

        while let Some(mut entry) = option_entry {
            entry.get_mut().rollback_all().await?;
            option_entry = entry.next_async().await;
        }

        Ok(())
    }
}

impl DataSources {
    async fn _insert(
        &self,
        name: Arc<str>,
        conn: DatabaseConnection,
    ) -> Result<(), (Arc<str>, Connection)> {
        self.0
            .insert_async(name.clone(), Connection::new(name, conn))
            .await
    }

    async fn _current_txn(&self, name: &str) -> Result<Transaction, Error> {
        match self.0.get_async(name).await {
            Some(mut entry) => Ok(entry.get_mut().current_txn().await?),
            None => Err(Error::NotFoundDataSourceError { name: name.into() }),
        }
    }

    async fn _new_txn(&self, name: &str) -> Result<Transaction, Error> {
        match self.0.get_async(name).await {
            Some(mut entry) => Ok(entry.get_mut().new_txn().await?),
            None => Err(Error::NotFoundDataSourceError { name: name.into() }),
        }
    }

    async fn _commit(&self, name: &str) -> Result<(), Error> {
        match self.0.get_async(name).await {
            Some(mut entry) => entry.get_mut().commit().await,
            None => Err(Error::NotFoundDataSourceError { name: name.into() }),
        }
    }

    async fn _rollback(&self, name: &str) -> Result<(), Error> {
        match self.0.get_async(name).await {
            Some(mut entry) => entry.get_mut().rollback().await,
            None => Err(Error::NotFoundDataSourceError { name: name.into() }),
        }
    }
}
