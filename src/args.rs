use std::env;
use std::ops::{Deref, DerefMut};

pub trait ArgsComparator {
    fn cmp(&self, idx: usize, expected: &str) -> bool;
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

impl ArgsComparator for Args {
    fn cmp(&self, idx: usize, expected: &str) -> bool {
        let check_some = self.args.get(idx);

        if check_some.is_some() {
            if let Some(args) = check_some {
                if args == expected {
                    return true;
                }

                return false;
            }
        }

        return false;
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
}
