#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    Other(anyhow::Error),
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> ApiError {
        ApiError::Other(err)
    }
}