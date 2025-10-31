use axum::response::IntoResponse;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),

    #[error("an error occured")]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        todo!(":)")
    }
}
