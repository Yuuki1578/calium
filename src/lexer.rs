use crate::syntax::SyntaxError;
use std::ops::{Deref, DerefMut};
use std::str::Chars;

macro_rules! scan {
    ($str:expr) => {{
        crate::lexer::Scanner::new($str).scan_move()
    }};

    ($($str:expr),*) => {{
        let mut eval_vec = vec![];

        $(
            eval_vec.push($str.to_string());
        )*

        crate::runtime::eval(&eval_vec)
    }};
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Number(i64),
    Addition,
    Substraction,
    Multiplication,
    Division,
    Remainder,
    Power,
    EndStatement,
}

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    chars: Chars<'a>,
    result: Vec<TokenKind>,
    pos_line: usize,
    pos_col: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars(),
            result: Vec::new(),
            pos_line: 1,
            pos_col: 1,
        }
    }

    fn add_line(&mut self) {
        self.pos_line += 1;
    }

    fn add_column(&mut self) {
        self.pos_col += 1;
    }

    fn reset_column(&mut self) {
        self.pos_col = 1;
    }

    fn add_token(&mut self, tok: TokenKind) {
        self.result.push(tok);
    }

    #[allow(unused_assignments)]
    pub fn scan(&mut self) -> Result<&mut Self, SyntaxError> {
        while let Some(chr) = self.next() {
            let mut is_number = 0_i64;

            match chr {
                '0'..='9' => {
                    is_number = chr.to_digit(10).unwrap_or(0) as i64;

                    'inner: while let Some(inside) = self.clone().next() {
                        self.add_column();

                        if let Some(digit) = inside.to_digit(10) {
                            is_number = is_number * 10 + digit as i64;
                            self.next();
                        } else {
                            break 'inner;
                        }
                    }

                    self.result.push(TokenKind::Number(is_number));
                    is_number = 0;
                }

                '+' => {
                    self.add_token(TokenKind::Addition);
                    self.add_column();
                }

                '-' => {
                    self.add_token(TokenKind::Substraction);
                    self.add_column();
                }

                '*' => {
                    self.add_token(TokenKind::Multiplication);
                    self.add_column();
                }

                '/' => {
                    self.add_token(TokenKind::Division);
                    self.add_column();
                }

                '%' => {
                    self.add_token(TokenKind::Remainder);
                    self.add_column();
                }

                '^' => {
                    self.add_token(TokenKind::Power);
                    self.add_column();
                }

                ';' => {
                    self.add_token(TokenKind::EndStatement);
                    self.add_column();
                }

                '\n' => {
                    self.add_line();
                    self.reset_column();
                }

                ' ' | '\t' => {
                    self.add_column();
                }

                _ => {
                    let error = SyntaxError::new(
                        format!("unexpected symbol \'{chr}\'"),
                        self.pos_line,
                        self.pos_col,
                    );
                    return Err(error);
                }
            }
        }

        Ok(self)
    }

    pub fn scan_move(mut self) -> Result<Self, SyntaxError> {
        let result = Self::clone(self.scan()?);

        Ok(result)
    }
}

impl<'a> Deref for Scanner<'a> {
    type Target = Chars<'a>;

    fn deref(&self) -> &Self::Target {
        &self.chars
    }
}

impl<'a> DerefMut for Scanner<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chars
    }
}
