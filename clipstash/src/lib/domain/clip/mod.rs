pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Invalid title: {0}")]
    InvalidTitle(String),
    #[error("Empty content")]
    EmptyContent,
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("Failed to parse date: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("Invalid ID: {0}")]
    Id(#[from] uuid::Error),
    #[error("Failed to parse hits: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    #[serde(skip)]
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}
