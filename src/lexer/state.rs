#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LexerState(usize);


impl LexerState {
    pub fn new(state: usize) -> Self {
        Self(state)
    }

    pub fn start_state() -> LexerState {
        LexerState(0)
    }

    pub fn is_start_state(&self) -> bool {
        self.0 == 0
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl Default for LexerState {
    fn default() -> Self {
        Self::start_state()
    }
}