use std::num::NonZeroUsize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TokenType(NonZeroUsize);

impl TokenType {
    #[inline]
    #[must_use]
    pub fn get(&self) -> NonZeroUsize {
        self.0
    }

    #[inline]
    #[must_use]
    pub fn new(value: NonZeroUsize) -> TokenType {
        Self(value)
    }
}
