#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClassificationClass(usize);

impl ClassificationClass {
    #[must_use]
    pub fn default_class() -> Self {
        Self(0)
    }

    #[must_use]
    pub fn get(&self) -> usize {
        self.0
    }

    #[must_use]
    pub fn is_default_class(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

impl Default for ClassificationClass {
    fn default() -> Self {
        Self::default_class()
    }
}
