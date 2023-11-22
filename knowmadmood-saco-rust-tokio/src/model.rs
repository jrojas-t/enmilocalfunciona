use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]

pub struct Operation {
    pub operation: String,
    pub number_a: i32,
    pub number_b: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

pub type DB = Arc<Mutex<Vec<Book>>>;

pub fn db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
