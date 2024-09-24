use std::{collections::HashMap, sync::Arc};

use sea_orm::DatabaseConnection;

use crate::{DataSource, Error};

#[derive(Debug)]
pub struct DataSources(HashMap<Arc<str>, DataSource>);

impl DataSources {
    pub fn new(map: &HashMap<Arc<str>, DatabaseConnection>) -> Self {
        let map = map
            .iter()
            .map(|(name, conn)| (name.clone(), DataSource::new(name.clone(), conn.clone())))
            .collect();

        Self(map)
    }

    pub fn get(&self, name: &str) -> Option<&DataSource> {
        self.0.get(name)
    }

    pub fn standalone(&self) -> Self {
        let map = self
            .0
            .iter()
            .map(|(name, conn)| (name.clone(), conn.standalone()))
            .collect();

        Self(map)
    }

    pub async fn commit_all(&self) -> Result<(), Error> {
        for source in self.0.values() {
            source.commit_all().await?;
        }

        Ok(())
    }

    pub async fn rollback_all(&self) -> Result<(), Error> {
        for source in self.0.values() {
            source.rollback_all().await?;
        }

        Ok(())
    }
}
