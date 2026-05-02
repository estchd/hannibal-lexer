use std::{cmp::min, io::Read};

use thiserror::Error;
use utf8_read::{Char, Reader};

#[derive(Debug, Error)]
pub enum LexerInputError {
    #[error("Input is not valid UTF-8: {0}")]
    Utf8(#[from] utf8_read::Error),
}

pub struct LexerInput<R: Read> {
    buffer: Vec<char>,
    current_index: Option<usize>,
    input: Reader<R>,
}

impl<R: Read> LexerInput<R> {
    pub fn get_buffer(&self) -> Vec<char> {
        self.buffer
            .clone()
    }

    pub fn get_current_lexeme_length(&self) -> usize {
        if let Some(current_index) = self.current_index {
            current_index
        } else {
            self.buffer
                .len()
        }
    }

    pub fn get_remaining(&self) -> Vec<char> {
        if let Some(current_index) = self.current_index {
            self.buffer[current_index..].to_vec()
        } else {
            self.buffer
                .clone()
        }
    }

    pub fn new(input: R) -> Self {
        Self {
            buffer: Vec::with_capacity(256),
            input: Reader::new(input),
            current_index: Option::default(),
        }
    }

    /// # Errors
    ///
    /// Will return 'Err' if:
    /// - There is an error reading the next UTF-8 character from the input, see [`utf8_read::Reader::next_char`](utf8_read::Reader::next_char) for more details.
    pub fn next_char(&mut self) -> Result<Option<char>, LexerInputError> {
        if let Some(current_index) = self.current_index {
            let next_char = self.buffer[current_index];

            let next_index = current_index + 1;

            self.current_index = if next_index
                == self
                    .buffer
                    .len()
            {
                None
            } else {
                Some(next_index)
            };

            return Ok(Some(next_char));
        }

        let next_char = self
            .input
            .next_char()?;

        let next_char = match next_char {
            Char::Eof => return Ok(None),
            Char::NoData => unreachable!("NoData should only be returned if the Reader is configured to emit it, which we don't do"),
            Char::Char(value) => value,
        };

        self.buffer
            .push(next_char);

        Ok(Some(next_char))
    }

    pub fn remove_lexeme(
        &mut self,
        lexeme_length: usize,
    ) -> Vec<char> {
        let actual_length = min(
            lexeme_length,
            self.buffer
                .len(),
        );

        let lexeme = self
            .buffer
            .drain(0..actual_length)
            .collect::<Vec<char>>();
        self.current_index = Some(0);

        lexeme
    }
}

impl<R: Read> Iterator for LexerInput<R> {
    type Item = Result<char, LexerInputError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_char() {
            Ok(Some(next_char)) => Some(Ok(next_char)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
