#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error("Error: Line {0}: Incorrect Indentation")]
    IndentError(usize),
    #[error("Error: Line {0}: Incomplete Definition")]
    InvalidDefinition(usize),
    #[error("Error: Line {0}: Invalid Symbol `{1}`")]
    InvalidSymbol(usize, String),
    #[error("Error: Line {0}: Symbol `{1}` already defined")]
    SymbolAlreadyDefined(usize, String),
}

pub fn raise_compiler_error(e: CompilerError) {
    println!("{}", e);
    std::process::exit(1);
}
