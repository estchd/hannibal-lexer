use crate::token_type::TokenType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LexerToken {
    pub token_type: TokenType,
    pub token_value: Vec<char>
}