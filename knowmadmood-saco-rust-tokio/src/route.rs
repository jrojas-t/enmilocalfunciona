use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        health_handler, post_handler, put_handler, delete_handler, get_handler , get_all_handler, get_api_yes_no,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::db();

    Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/books",get(get_all_handler))
        .route("/api/book/create", post(post_handler))
        .route("/api/book/:id",get(get_handler)
                                        .put(put_handler)
                                        .delete(delete_handler),
        )
        .route("/api/apiexternal", get(get_api_yes_no))
        .with_state(db)
}