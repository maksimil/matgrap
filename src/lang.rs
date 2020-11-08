use crate::canvas::*;
use std::collections::HashMap;
use std::fmt;

type Call = Vec<String>;

#[derive(Clone)]
pub struct Function {
    validate: fn(&Call, fn(&str)) -> bool,
    run: fn(&Call, &mut Canvas),
}

impl fmt::Debug for Function {
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result {
        f.write_str("Function")
    }
}

pub struct Language<'a> {
    pub funcs: HashMap<&'static str, Function>,
    canvas: &'a mut Canvas<'a>,
}

impl<'a> Language<'a> {
    pub fn new<'b>(canvas: &'b mut Canvas<'b>) -> Language {
        Language {
            canvas,
            funcs: [
                // Functions
                (
                    "a",
                    Function {
                        validate: |call, f| true,
                        run: |call, c| {},
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}
