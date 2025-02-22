use crate::util::CompilerError;
use crate::util::CompilerError::IndentError;

const TAB_WIDTH: usize = 4;

pub(crate) struct LineIterator<'a> {
    curr_line: Line<'a>
}

impl<'a> LineIterator<'a> {
    pub(crate) fn new(filename: &'a String, tokens: &'a Vec<String>) -> Self {
        LineIterator {
            curr_line: Line {
                line_num: 0,
                start: 0,
                end: 0,
                indent: 0,
                file_name: filename,
                tokens,
            }
        }
    }
}

#[derive(Copy, Clone)]
pub(crate)  struct Line<'a> {
    pub(crate) line_num: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) indent: usize,
    pub(crate) file_name: &'a String,
    pub(crate) tokens: &'a Vec<String>
}

fn get_indent_level(indent_token: &String) -> Result<usize, CompilerError> {
    let mut count = 0;
    for char in indent_token.chars() {
        match char {
            '\t' => count += TAB_WIDTH,
            ' ' => count += 1,
            _ => {}
        }
    }

    if count & (TAB_WIDTH - 1) == 0 {
        Ok(count)
    } else {
        Err(IndentError)
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = &mut self.curr_line;
        line.line_num += 1;

        line.indent = get_indent_level(&line.tokens[line.end])
            .unwrap_or_else(|e| panic!("Your indent sucks"));

        line.start = line.end;
        line.end += 1;

        while line.end < line.tokens.len() {
            let token = &line.tokens[line.end];
            if token.chars().next().unwrap() == '\n' {
                return Some(*line);
            }
            line.end += 1;
        }

        None
    }
}
