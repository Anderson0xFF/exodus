#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

use self::state::State;

pub mod expr;
pub mod state;
pub mod types;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    state: Vec<State>,
}

impl Program {
    pub fn new(state: Vec<State>) -> Self {
        Self { state }
    }

    /// Get a reference to the program's state.
    pub fn state(&self) -> &[State] {
        self.state.as_ref()
    }

    /// Get a mutable reference to the program's state.
    pub fn state_mut(&mut self) -> &mut Vec<State> {
        &mut self.state
    }

    pub fn compile(lexer : logos::Lexer<lexer::Token>){
        
    }
}
