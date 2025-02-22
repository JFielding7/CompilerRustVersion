pub struct Type {
    pub name: String,
    pub size: usize,
    validate_literal: fn(&str) -> bool,
}

