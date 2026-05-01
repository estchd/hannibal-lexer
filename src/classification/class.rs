#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClassificationClass(usize);

impl ClassificationClass {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn default_class() -> Self {
        Self(0)
    }

    pub fn is_default_class(&self) -> bool {
        self.0 == 0
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl Default for ClassificationClass {
    fn default() -> Self {
        Self::default_class()
    }
}