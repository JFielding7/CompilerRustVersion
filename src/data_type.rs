use std::cell::RefCell;

pub struct Type {
    pub name: String,
    pub size: usize,
    validate_literal: fn(&str) -> bool,
}

thread_local! {
    static TYPES: RefCell<Vec<Type>> = RefCell::new(Vec::new());
}

fn valid_i64_literal(literal: &str) -> bool {
    literal.parse::<i64>().is_ok()
}

fn valid_str_literal(literal: &str) -> bool {
    let len = literal.len();
    let literal_bytes = literal.as_bytes();
    len > 1 && literal_bytes[0] == b'"' && literal_bytes[len - 1] == b'"'
}

impl Type {
    pub fn new(name: String, size: usize, validate_literal: fn(&str) -> bool) -> Type {
        Self {name, size, validate_literal }
    }

    pub fn compile_native_types() {
        const NATIVE_TYPES: [(&str, usize, fn(&str) -> bool); 2] = [
            ("i64", 8, valid_i64_literal),
            ("str", 8, valid_str_literal)
        ];

        for (name, size, value) in NATIVE_TYPES {
            TYPES.with(|types| types.borrow_mut().push(Type::new(name.to_string(), size, value)));
        }
    }
}