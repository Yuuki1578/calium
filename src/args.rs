use std::env;
use std::ops::{Deref, DerefMut};

macro_rules! string {
    ($slice:expr) => {{
        String::from($slice)
    }};

    ($($slice:expr),*) => {{
        let mut vec_string: Vec<String> = Vec::new();

        $(
            vec_string.push(string!($slice));
        )*

        vec_string
    }};
}

#[derive(Debug, Clone)]
pub struct Args {
    args: Vec<String>,
}

impl Deref for Args {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl DerefMut for Args {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.args
    }
}

impl Args {
    pub fn new(lim: usize) -> Self {
        Self {
            args: env::args()
                .enumerate()
                .filter(|(idx, _)| *idx != 0 && *idx <= lim)
                .map(|(_, string)| string)
                .collect(),
        }
    }

    #[inline]
    pub fn iter_cmp(&self, operand: Vec<String>) -> Vec<bool> {
        let final_result: Vec<bool> = Vec::new();
        let mut expected_queue = operand.iter();

        final_result
    }
}
