#![allow(unused)]

use crate::syntax::SyntaxError;
use crate::HAS_ERROR;

use std::fmt::Display;
use std::iter::Peekable;
use std::ops::{Deref, DerefMut};
use std::str::Chars;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub type ErrorList = Vec<SyntaxError>;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Number(i64),
    Addition,
    Substraction,
    Multiplication,
    Divvision,
    Remainder,
    Power,
    EndStatement,
}

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    peek: Peekable<Chars<'a>>,
    tokens: Vec<TokenKind>,
    line: usize,
    col: usize,
}

impl<'a> Deref for Scanner<'a> {
    type Target = Peekable<Chars<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.peek
    }
}

impl<'a> DerefMut for Scanner<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.peek
    }
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            peek: src.chars().peekable(),
            tokens: Vec::new(),
            line: 1,
            col: 1,
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(kind);
    }

    fn add_line(&mut self) {
        self.line += 1;
    }

    fn add_column(&mut self) {
        self.col += 1;
    }

    fn reset_column(&mut self) {
        self.col = 0;
    }

    pub fn scan(&mut self) -> Result<&mut Self, ErrorList> {
        let mut base_number: i64 = 0;
        let mut error_list: ErrorList = Vec::new();

        use TokenKind::*;

        while let Some(chr) = self.next() {
            match chr {
                '0'..='9' => {
                    base_number = base_number * 10 + chr.to_digit(10).unwrap_or_default() as i64;

                    'inner: while let Some(peeked) = self.peek() {
                        self.add_column();

                        if let Some(digit) = peeked.to_digit(10) {
                            base_number = base_number * 10 + digit as i64;
                            self.next();
                        } else {
                            break 'inner;
                        }
                    }

                    self.add_token(Number(base_number));
                    base_number = i64::default();
                }

                '+' => {
                    self.add_token(Addition);
                    self.add_column();
                }

                '-' => {
                    self.add_token(Substraction);
                    self.add_column();
                }

                '*' => {
                    self.add_token(Multiplication);
                    self.add_column();
                }

                '/' => {
                    self.add_token(Divvision);
                    self.add_column();
                }

                '%' => {
                    self.add_token(Remainder);
                    self.add_column();
                }

                '^' => {
                    self.add_token(Power);
                    self.add_column();
                }

                ';' => {
                    self.add_token(EndStatement);
                    self.add_column();
                }

                ' ' | '\t' => {
                    self.add_column();
                }

                '\n' => {
                    self.add_line();
                    self.reset_column();
                }

                _ => {
                    unsafe {
                        HAS_ERROR = true;
                    }

                    let error = SyntaxError::new(
                        format!(
                            "Syntax error:\nunexpected symbol '{}' at line {} : column {}",
                            chr, self.line, self.col
                        ),
                        self.line,
                        self.col,
                    );
                }
            }
        }

        if unsafe { HAS_ERROR } {
            Err(error_list)
        } else {
            Ok(self)
        }
    }
}
