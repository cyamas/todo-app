pub mod templates;
pub mod handlers;

use askama_axum::IntoResponse;
use axum::http::StatusCode;
use serde::Deserialize;
use chrono::{Utc, DateTime};



#[derive(Deserialize)]
pub struct AddTodo {
    pub project: String,
    pub task: String,
    pub priority: String,
}

#[derive(Debug)]
pub struct PendingTodo {
    pub todo_id: i32,
    pub project: String,
    pub task: String,
    pub task_priority: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct EditTodo {
    pub id: String,
    pub project: String,
    pub task: String,
    pub priority: String,
}

#[derive(Deserialize, Debug)]
pub struct TodoId {
    pub id: String,
}

pub struct CompletedTodo {
    pub todo_id: i32,
    pub project: String,
    pub task: String,
    pub task_priority: i32,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Date {
    pub month: u32,
    pub day: u32,
    pub year: i32,
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> askama_axum::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", self.0)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err:E) -> Self {
        Self(err.into())
    }
}
