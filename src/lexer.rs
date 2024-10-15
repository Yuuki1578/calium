use crate::syntax::SyntaxError;
use std::ops::{Deref, DerefMut};
use std::str::Chars;

#[derive(Debug)]
pub enum TokenKind {
    Number(u128),
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    EOL,
}

#[derive(Debug)]
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

    fn inc_line(&mut self) {
        self.pos_line += 1;
    }

    fn inc_col(&mut self) {
        self.pos_col += 1;
    }

    fn reset_col(&mut self) {
        self.pos_col = 1;
    }

    fn push_token(&mut self, tok: TokenKind) {
        self.result.push(tok);
    }

    #[allow(unused_assignments)]
    pub fn scan(&mut self) -> Result<&mut Self, SyntaxError> {
        use TokenKind::*;

        while let Some(chr) = self.next() {
            let mut is_number = 0_u128;

            match chr {
                '0'..='9' => {
                    is_number = chr.to_digit(10).unwrap_or(0) as u128;

                    'inner: while let Some(inside) = self.clone().next() {
                        self.inc_col();

                        if let Some(digit) = inside.to_digit(10) {
                            is_number = is_number * 10 + digit as u128;
                            self.next();
                        } else {
                            break 'inner;
                        }
                    }

                    self.result.push(Number(is_number));
                    is_number = 0;
                }

                '+' => {
                    self.push_token(Add);
                    self.inc_col();
                }

                '-' => {
                    self.push_token(Sub);
                    self.inc_col();
                }

                '*' => {
                    self.push_token(Mul);
                    self.inc_col();
                }

                '/' => {
                    self.push_token(Div);
                    self.inc_col();
                }

                '%' => {
                    self.push_token(Rem);
                    self.inc_col();
                }

                '^' => {
                    self.push_token(Pow);
                    self.inc_col();
                }

                ';' => {
                    self.push_token(EOL);
                    self.inc_col();
                }

                '\n' => {
                    self.inc_line();
                    self.reset_col();
                }

                ' ' | '\t' => {
                    self.inc_col();
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

    pub fn parser_dump(&self) -> String {
        format!("{:#?}", &self.result)
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
