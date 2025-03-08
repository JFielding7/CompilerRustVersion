use crate::compiler_error::CompilerError;
use crate::compiler_error::CompilerError::IndentError;

const TAB_WIDTH: usize = 4;

pub struct LineIterator<'a> {
    curr_line: Line<'a>
}

impl<'a> LineIterator<'a> {
    pub fn new(filename: &'a String, tokens: &'a Vec<String>) -> Self {
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

#[derive(Copy, Clone, Debug)]
pub struct Line<'a> {
    pub line_num: usize,
    pub start: usize,
    pub end: usize,
    pub indent: usize,
    pub file_name: &'a String,
    pub tokens: &'a Vec<String>
}

impl<'a> Line<'a> {
    pub fn get_token(&self, i: usize) -> &'a String {
        &self.tokens[i - self.start]
    }

    fn set_indent_level(&mut self) -> Result<(), CompilerError> {
        let mut count = 0;
        for char in self.tokens[self.start].chars() {
            match char {
                '\t' => count += TAB_WIDTH,
                ' ' => count += 1,
                _ => {}
            }
        }

        if count & (TAB_WIDTH - 1) == 0 {
            Ok(())
        } else {
            Err(IndentError(self.line_num))
        }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = &mut self.curr_line;
        line.line_num += 1;
        line.start = line.end;

        line.set_indent_level()
            .unwrap_or_else(|e| panic!("Incorrect Indentation"));

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
