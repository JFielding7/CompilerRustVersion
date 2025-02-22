#[derive(thiserror::Error, Debug)]
pub(crate) enum CompilerError {
    #[error("Incorrect Indentation")]
    IndentError,
}