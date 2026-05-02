#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LexerState(usize);

impl LexerState {
    #[must_use]
    pub fn get(&self) -> usize {
        self.0
    }

    #[must_use]
    pub fn is_start_state(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn new(state: usize) -> Self {
        Self(state)
    }

    #[must_use]
    pub fn start_state() -> LexerState {
        LexerState(0)
    }
}

impl Default for LexerState {
    fn default() -> Self {
        Self::start_state()
    }
}
