use crate::ast::{ASTSpec, BinaryOperator, NodeExpr, UnaryOperator};
use crate::syntax::SyntaxError;
use std::ops::{Deref, DerefMut};
use std::str::Chars;

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub enum TokenKind {
//     Number(i64),
//     Addition,
//     Substraction,
//     Multiplication,
//     Division,
//     Remainder,
//     Power,
//     EndStatement,
// }

pub trait ParsingSupport: Sized {
    fn to_digit(&mut self, rhs: Self);
    fn to_default(&mut self);
}

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    chars: Chars<'a>,
    result: Vec<NodeExpr>,
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

    fn add_node(&mut self, node: NodeExpr) {
        self.result.push(node);
    }

    pub fn scan(&mut self) -> Result<&mut Self, SyntaxError> {
        let mut if_digit: i64 = 0;

        while let Some(char) = self.next() {
            match char {
                '0'..='9' => {
                    if_digit.to_digit(char.to_digit(10).unwrap_or_default() as i64);

                    'inner_loop: while let Some(inner) = self.clone().next() {
                        self.add_column();

                        if let Some(digit) = inner.to_digit(10) {
                            if_digit.to_digit(digit as i64);
                            self.next();
                        } else {
                            break 'inner_loop;
                        }
                    }

                    self.add_node(NodeExpr::IntExpr(if_digit));
                    if_digit.to_default();
                }

                '+' => {
                    self.add_node(NodeExpr::BinaryExpr {
                        binary_op: BinaryOperator::Addition,
                        left_node: Box::new(self.result.last().unwrap().clone()),
                        right_node: continue,
                    });

                    self.add_column();
                }
            }
        }

        Ok(self)
    }

    // fn add_token(&mut self, tok: TokenKind) {
    //     self.result.push(tok);
    // }

    // #[allow(unused_assignments)]
    // pub fn scan_vector(&mut self) -> Result<&mut Self, SyntaxError> {
    //     use TokenKind::*;
    //     let mut is_number = 0_i64;

    //     while let Some(chr) = self.next() {
    //         match chr {
    //             '0'..='9' => {
    //                 is_number = chr.to_digit(10).unwrap_or(0) as i64;

    //                 'inner: while let Some(inside) = self.clone().next() {
    //                     self.add_column();

    //                     if let Some(digit) = inside.to_digit(10) {
    //                         is_number = is_number * 10 + digit as i64;
    //                         self.next();
    //                     } else {
    //                         break 'inner;
    //                     }
    //                 }

    //                 self.result.push(Number(is_number));
    //                 is_number = 0;
    //             }

    //             '+' => {
    //                 self.add_token(Addition);
    //                 self.add_column();
    //             }

    //             '-' => {
    //                 self.add_token(Substraction);
    //                 self.add_column();
    //             }

    //             '*' => {
    //                 self.add_token(Multiplication);
    //                 self.add_column();
    //             }

    //             '/' => {
    //                 self.add_token(Division);
    //                 self.add_column();
    //             }

    //             '%' => {
    //                 self.add_token(Remainder);
    //                 self.add_column();
    //             }

    //             '^' => {
    //                 self.add_token(Power);
    //                 self.add_column();
    //             }

    //             ';' => {
    //                 self.add_token(EndStatement);
    //                 self.add_column();
    //             }

    //             '\n' => {
    //                 self.add_line();
    //                 self.reset_column();
    //             }

    //             ' ' | '\t' => {
    //                 self.add_column();
    //             }

    //             _ => {
    //                 let error = SyntaxError::new(
    //                     format!("unexpected symbol \'{chr}\'"),
    //                     self.pos_line,
    //                     self.pos_col,
    //                 );
    //                 return Err(error);
    //             }
    //         }
    //     }

    //     Ok(self)
    // }

    // pub fn scan_vector_move(&mut self) -> Result<Self, SyntaxError> {
    //     let result = Self::clone(self.scan_vector()?);
    //     Ok(result)
    // }

    // pub fn parser_dump(&self) -> String {
    //     format!("{:#?}", &self.result)
    // }
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

impl ParsingSupport for i64 {
    fn to_digit(&mut self, rhs: Self) {
        const RADIX: u8 = 10;

        *self = *self * RADIX as Self + rhs;
    }

    fn to_default(&mut self) {
        *self = Self::default();
    }
}
