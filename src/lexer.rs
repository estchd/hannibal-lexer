use std::io::Read;
use thiserror::Error;
use crate::lexer::definition::LexerDefinition;
use crate::lexer::LexerError::Invalid;
use crate::lexer::input::{LexerInput, LexerInputError};
use crate::lexer::state::LexerState;
use crate::lexer::token::LexerToken;
use crate::token_type::TokenType;

pub mod builder;
pub mod definition;
pub mod state;
pub mod meta;
pub mod token;
pub mod input;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Error reading next input character: {0}")]
    Input(#[from] LexerInputError),
    #[error("The read input does not contain any valid lexemes, read characters: {0:?}")]
    Invalid(Vec<char>)
}

pub struct Lexer<R: Read> {
    definition: LexerDefinition,
    input: LexerInput<R>,
    state: LexerState,
}

impl<R: Read> Lexer<R> {
    pub fn new(
        definition: LexerDefinition,
        input: R
    ) -> Self {
        Self {
            definition,
            input: LexerInput::new(input),
            state: LexerState::start_state(),
        }
    }

    pub fn next(&mut self) -> Result<LexerToken, LexerError> {
        let mut last_final_state: Option<(TokenType, usize)> = None;

        loop {
           let next_char = self.input.next()?;

           let next_char = match next_char {
               None => break,
               Some(char) => char
           };

            let new_state = self.definition.transition_by_char(self.state, next_char);

            let new_state = match new_state {
                None => break,
                Some(state) => state
            };

            self.state = new_state;

            if let Some(new_token_type) = self.definition.state_to_token_type(new_state) {
                last_final_state = Some((new_token_type, self.input.get_current_lexeme_length()));
            }
        }

        let (token_type, lexeme_length) = match last_final_state {
            None => return Err(Invalid(self.input.get_buffer())),
            Some((token_type, lexeme_length)) => (token_type, lexeme_length)
        };

        let token_value = self.input.remove_lexeme(lexeme_length);

        let token = LexerToken {
            token_type,
            token_value
        };

        return Ok(token);
    }
}
