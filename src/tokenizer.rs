use std::fs::File;
use std::io;
use std::io::Read;
use std::rc::Rc;
use regex::Regex;

fn read_source_file(name: &String) -> io::Result<String> {
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn tokenize_file(filename: &String) -> io::Result<Vec<Rc<String>>> {
    const TOKEN_REGEX: &str = "\n[ \t]*|[-+*/%|&~^()=,]|\\w+|\".*?\"";
    let regex = Regex::new(TOKEN_REGEX).unwrap();

    let code= read_source_file(filename)?;

    let mut tokens = vec![Rc::new("\n".to_string())];
    regex.captures_iter(&code).for_each(|token| tokens.push(Rc::new(token[0].to_string())));
    tokens.push(Rc::new("\n".to_string()));
    Ok(tokens)
}
