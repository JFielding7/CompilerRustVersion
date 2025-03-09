use crate::compiler_error::CompilerError;
use crate::compiler_error::CompilerError::IndentError;

#[derive(Copy, Clone, Debug)]
pub struct Line<'a> {
    pub line_num: usize,
    pub start: usize,
    pub end: usize,
    pub indent: usize,
    pub file_name: &'a String,
    pub tokens: &'a [String]
}

impl<'a> Line<'a> {
    pub fn get_token(&self, i: usize) -> &'a String {
        &self.tokens[i - self.start]
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    fn set_indent_level(&mut self) -> Result<(), CompilerError> {
        const TAB_WIDTH: usize = 4;

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

pub struct LineIterator<'a> {
    curr_line: Line<'a>,
    tokens: Vec<String>
}

impl<'a> LineIterator<'a> {
    pub fn new(filename: &'a String, tokens: Vec<String>) -> Self {
        LineIterator {
            curr_line: Line {
                line_num: 0,
                start: 0,
                end: 0,
                indent: 0,
                file_name: filename,
                tokens: &[],
            },
            tokens
        }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Box<Line<'a>>;

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
                line.tokens = &line.tokens[line.start..line.end];
                return Some(Box::new(*line));
            }
            line.end += 1;
        }

        None
    }
}
