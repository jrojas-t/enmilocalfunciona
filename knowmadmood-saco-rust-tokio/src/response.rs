use crate::model::Book;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct CRUDResponse {
    pub status: String,
    pub data: Book,
}

#[derive(Serialize, Debug)]
pub struct BookListResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<Book>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SingleApiResponse {
    pub status: String,
    pub data: ApiYesOrNoResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiYesOrNoResponse {
    answer: String,
    forced: bool,
    image: String,
}
