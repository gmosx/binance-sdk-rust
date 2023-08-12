use serde::Serialize;

use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub struct Request<P> {
    pub method: String,
    pub params: P,
    pub id: u64,
}
