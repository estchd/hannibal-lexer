use std::{cmp::min, io::Read};

use thiserror::Error;
use utf8_read::{Char, Reader};

#[derive(Debug, Error)]
pub enum LexerInputError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Input is not valid UTF-8: {0}")]
    Utf8(#[from] utf8_read::Error),
}

pub struct LexerInput<R: Read> {
    buffer: Vec<char>,
    input: Reader<R>,
    current_index: Option<usize>,
}

impl<R: Read> LexerInput<R> {
    pub fn new(input: R) -> Self {
        Self {
            buffer: Vec::with_capacity(256),
            input: Reader::new(input),
            current_index: None,
        }
    }

    pub fn next(&mut self) -> Result<Option<char>, LexerInputError> {
        if let Some(current_index) = self.current_index {
            let next_char = self.buffer[current_index];

            let next_index = current_index + 1;

            self.current_index = if next_index == self.buffer.len() {
                None
            } else {
                Some(next_index)
            };

            return Ok(Some(next_char));
        }

        let next_char = self.input.next_char()?;

        let next_char = match next_char {
            Char::Eof => return Ok(None),
            Char::NoData => panic!("Unexpected NoData"),
            Char::Char(value) => value
        };

        self.buffer.push(next_char);

        return Ok(Some(next_char));
    }

    pub fn remove_lexeme(&mut self, lexeme_length: usize) -> Vec<char> {
        let actual_length = min(lexeme_length, self.buffer.len());

        let lexeme = self.buffer.drain(0..actual_length).collect::<Vec<char>>();
        self.current_index = Some(0);

        return lexeme
    }

    pub fn get_buffer(&self) -> Vec<char> {
        self.buffer.clone()
    }

    pub fn get_remaining(&self) -> Vec<char> {
        return if let Some(current_index) = self.current_index {
            self.buffer[current_index..].to_vec()
        }
        else {
            self.buffer.clone()
        }
    }

    pub fn get_current_lexeme_length(&self) -> usize {
        if let Some(current_index) = self.current_index {
            current_index
        }
        else {
            self.buffer.len()
        }
    }
}
