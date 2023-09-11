use crate::common::infrastructure::error::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum BoardPortError {
    #[error("Not Found")]
    NotFound(i32),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for BoardPortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}