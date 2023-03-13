use serde::Deserialize;

pub mod domains;
pub mod message;
pub mod rules;
pub mod stats;


#[derive(Debug, Deserialize)]
pub struct ResponseStatus {
    pub status: String,
}