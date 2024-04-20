use crate::models::todo::Todo;
use serde::Serialize;

#[derive(Serialize)]
pub enum SendTodoParamResult {
    Str(String),
    Bool(bool),
}