use std::fmt::Display;

#[derive(Debug)]
pub struct SyntaxError {
    msg: String,
    line: usize,
    col: usize,
}

impl SyntaxError {
    pub fn new(msg: String, line: usize, col: usize) -> Self {
        Self { msg, line, col }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} at [{}:{}]", &self.msg, self.line, self.col)
    }
}
