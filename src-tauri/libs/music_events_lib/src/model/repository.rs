use std::sync::Arc;

use sqlx::{Error, PgPool};

pub use repository_derive::Repository;

pub trait PgPoolContainer {
    fn pool(&self) -> Arc<PgPool>;
}

pub trait RepositoryParamsProvider {
    type Model;
    type Entity: Into<Self::Model>;
    fn pool(&self) -> Arc<PgPool>;
    fn get_table_name() -> String;
    fn get_params() -> Vec<String>;

    fn form_params_group_and_placeholders() -> (String, String) {
        let params = Self::get_params();
        let groups = format!("({})", params.join(","));
        let placeholders = format!(
            "({})",
            params
                .iter()
                .enumerate()
                .map(|(_, i)| format!("${}", i))
                .collect::<Vec<_>>()
                .join(",")
        );
        (groups, placeholders)
    }
}

pub trait Repository {
    type Model;
    type Entity: Into<Self::Model>;
    async fn add(&self, value: &Self::Model) -> Result<u64, Error>;
    async fn get_all(&self) -> Result<Vec<Self::Model>, Error>;
    async fn get_count(&self) -> Result<u64, Error>;
    async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Error>;
    async fn update(&self, value: &Self::Model) -> Result<(), Error>;
    async fn remove(&self, id: u64) -> Result<u64, Error>;

    async fn get_ids(&self) -> Result<Vec<i64>, Error> {
        unimplemented!();
    }
    async fn get_paginated(&self, _count: i64, _offset: i64) -> Result<Vec<Self::Model>, Error> {
        unimplemented!();
    }
}