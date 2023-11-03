use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    model::{QueryOptions, Book, UpdateSchema, DB},
    response::{BookListResponse, ApiYesOrNoResponse, CRUDResponse},
};

pub async fn health_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Up";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}


pub async fn get_all_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let data = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let data: Vec<Book> = data.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = BookListResponse {
        status: "success".to_string(),
        results: data.len(),
        data,
    };

    Json(json_response)
}

pub async fn post_handler(
    State(db): State<DB>,
    Json(mut body): Json<Book>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title: '{}' already exists", todo.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    vec.push(body);

    let json_response = CRUDResponse {
        status: "success".to_string(),
        data: { todo },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.id == Some(id.to_owned())) {
        let json_response = CRUDResponse {
            status: "success".to_string(),
            data: { todo.clone() },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn put_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(body): Json<UpdateSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter_mut().find(|todo| todo.id == Some(id.clone())) {
        let datetime = chrono::Utc::now();
        let title = body
            .title
            .to_owned()
            .unwrap_or_else(|| todo.title.to_owned());
        let content = body
            .content
            .to_owned()
            .unwrap_or_else(|| todo.content.to_owned());
        let completed = body.completed.unwrap_or(todo.completed.unwrap());
        let payload = Book {
            id: todo.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                todo.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                todo.content.to_owned()
            },
            completed: Some(completed),
            createdAt: todo.createdAt,
            updatedAt: Some(datetime),
        };
        *todo = payload;

        let json_response = CRUDResponse {
            status: "success".to_string(),
            data: { todo.clone() },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with ID: {} not found", id)
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

pub async fn delete_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(|todo| todo.id == Some(id.clone())) {
        vec.remove(pos);

        let json_response = serde_json::json!({
            "status": "success",
            "message": format!("The book with ID: {}, has been successfully deleted", id)
        });
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn get_api_yes_no() -> impl IntoResponse {

    let body: &str = "{\"answer\":\"yes\",\"forced\":false,\"image\":\"https://yesno.wtf/assets/yes/7-653c8ee5d3a6bbafd759142c9c18d76c.gif\"}";

    let resp: ApiYesOrNoResponse = serde_json::from_str(body).unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "message": resp
    });

    Json(json_response)
}