use std::io::Read;
use thiserror::Error;
use utf8_read::{Char, Reader};
use crate::lexer::definition::LexerDefinition;
use crate::lexer::LexerError::Invalid;
use crate::lexer::meta::LexerMeta;
use crate::lexer::state::LexerState;
use crate::lexer::token::LexerToken;
use crate::token_type::TokenType;

pub mod builder;
pub mod definition;
pub mod state;
pub mod meta;
mod token;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Input is not valid UTF-8: {0}")]
    Utf8(#[from] utf8_read::Error),
    #[error("The read input does not contain any valid lexemes")]
    Invalid(Vec<char>)
}

#[derive(Debug, Clone)]
pub struct Lexer {
    definition: LexerDefinition,
    meta: LexerMeta,
}

impl Lexer {
    pub fn next<R: Read>(&mut self, input: &mut R, previous_remaining: Option<&[char]>) -> Result<(LexerToken, Vec<char>), LexerError> {
        let mut input = Reader::new(input);

        let mut current_state = LexerState::start_state();

        let mut last_token_type: Option<(TokenType, usize)> = None;

        let mut buffer = Vec::<char>::new();

        let mut index = 0;

        if let Some(remaining) = previous_remaining {
            let mut index = 0;
            for remaining_char in remaining {
                let remaining_char = *remaining_char;

                todo!()
            }
        }

        loop {
            let next_char = input.next_char()?;

            let next_char = match next_char {
                Char::Eof => break,
                Char::NoData => panic!("Unexpected NoData"),
                Char::Char(value) => value
            };

            buffer.push(next_char);
            index += 1;

            let new_state = self.definition.transition_by_char(current_state, next_char);

            let new_state = match new_state {
                None => break,
                Some(state) => state
            };

            current_state = new_state;

            if let Some(new_token_type) = self.definition.state_to_token_type(new_state) {
                last_token_type = Some((new_token_type, index));
            }
        }

        let (token_type, final_index) = match last_token_type {
            None => return Err(Invalid(buffer)),
            Some((final_state, final_index)) => (final_state, final_index)
        };

        let token_value = buffer[0..final_index].to_vec();

        let remaining = buffer[final_index..].to_vec();

        let token = LexerToken {
            token_type,
            token_value,
        };

        Ok((token, remaining))
    }
}