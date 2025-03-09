use std::collections::HashMap;

#[derive(Debug)]
pub struct Type {
    name: String,
    size: usize,
    validate_literal: fn(&str) -> bool,
}

fn valid_i64_literal(literal: &str) -> bool {
    literal.parse::<i64>().is_ok()
}

fn valid_str_literal(literal: &str) -> bool {
    let len = literal.len();
    let literal_bytes = literal.as_bytes();
    len > 1 && literal_bytes[0] == b'"' && literal_bytes[len - 1] == b'"'
}

pub fn compile_native_types() -> HashMap<String, Type> {
    const NATIVE_TYPE_COUNT: usize = 2;
    const NATIVE_TYPES: [(&str, usize, fn(&str) -> bool); NATIVE_TYPE_COUNT] = [
        ("i64", 8, valid_i64_literal),
        ("str", 8, valid_str_literal)
    ];

    let mut types = HashMap::with_capacity(NATIVE_TYPE_COUNT);
    NATIVE_TYPES.iter().for_each(|&(name, size, value)| {
        types.insert(name.to_string(), Type::new(name.to_string(), size, value));
    });

    types
}

pub fn get_literal_type<'a>(types: &'a HashMap<String, Type>, literal: &str) -> Option<&'a Type> {
    for data_type in types.values() {
        if (data_type.validate_literal)(literal) {
            return Some(data_type);
        }
    }

    None
}

impl Type {
    pub fn new(name: String, size: usize, validate_literal: fn(&str) -> bool) -> Self {
        Self { name, size, validate_literal }
    }
}
