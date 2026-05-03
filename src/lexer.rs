pub mod builder;
pub mod definition;
pub mod input;
pub mod meta;
pub mod state;
pub mod token;

use crate::lexer::LexerError::Invalid;
use crate::lexer::definition::LexerDefinition;
use crate::lexer::input::{LexerInput, LexerInputError};
use crate::lexer::state::LexerState;
use crate::lexer::token::LexerToken;
use crate::token_type::TokenType;
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Error reading next input character: {0}")]
    Input(#[from] LexerInputError),
    #[error("The read input does not contain any valid lexemes, read characters: {0:?}")]
    Invalid(Vec<char>),
}

pub struct Lexer<R: Read> {
    definition: LexerDefinition,
    input: LexerInput<R>,
    state: LexerState,
}

impl<R: Read> Lexer<R> {
    #[inline]
    pub fn new(
        definition: LexerDefinition,
        input: R,
    ) -> Self {
        Self {
            definition,
            input: LexerInput::new(input),
            state: LexerState::start_state(),
        }
    }

    /// # Errors
    ///
    /// Will return 'Err' if:
    /// - There is an error reading the next input character, see [`LexerInput::next_char`](crate::lexer::input::LexerInput::next_char) for more details.
    /// - There is no more input before the DFA encounters a valid final state, meaning the read input does not contain any valid lexemes.
    /// - The Lexer reads a character that does not have a valid transition from the current state and no valid final state has been encountered yet, meaning the read input does not contain any valid lexemes.
    #[inline]
    pub fn next_lexeme(&mut self) -> Result<Option<LexerToken>, LexerError> {
        let mut last_final_state: Option<(TokenType, usize)> = None;

        loop {
            let next_char = self.input.next_char()?;

            let Some(next_char) = next_char else {
                break;
            };

            let new_state = self
                .definition
                .transition_by_char(self.state, next_char);

            let Some(new_state) = new_state else {
                break;
            };

            self.state = new_state;

            if let Some(new_token_type) = self
                .definition
                .state_to_token_type(new_state)
            {
                last_final_state = Some((
                    new_token_type,
                    self.input
                        .get_current_lexeme_length(),
                ));
            }
        }

        let (token_type, lexeme_length) = match last_final_state {
            None => {
                if self
                    .input
                    .get_current_lexeme_length()
                    == 0
                {
                    self.state = LexerState::start_state();
                    return Ok(None);
                }

                self.state = LexerState::start_state();

                return Err(Invalid(self.input.get_buffer()));
            },
            Some((token_type, lexeme_length)) => (token_type, lexeme_length),
        };

        let token_value = self
            .input
            .remove_lexeme(lexeme_length);

        let token = LexerToken {
            token_type,
            token_value,
        };

        self.state = LexerState::start_state();

        Ok(Some(token))
    }
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Result<LexerToken, LexerError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_lexeme() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
