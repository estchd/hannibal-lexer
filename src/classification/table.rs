use std::collections::HashMap;
use crate::classification::class::ClassificationClass;

#[derive(Debug, Clone)]
pub struct ClassificationTable {
    ascii_table: [ClassificationClass; 128],
    extended_table: HashMap<char, ClassificationClass>,
}

impl ClassificationTable {
    pub fn new() -> Self {
        Self {
            ascii_table: [ClassificationClass::default_class(); 128],
            extended_table: Default::default(),
        }
    }

    pub fn classify(&self, value: char) -> ClassificationClass {
        if value.is_ascii() {
            return self.ascii_table[value as usize]
        }

        self.extended_table.get(&value)
            .cloned()
            .unwrap_or(ClassificationClass::default_class())
    }

    pub fn add_classification(&mut self, value: char, class: ClassificationClass) {
        if value.is_ascii() {
            self.ascii_table[value as usize] = class;
        }

        self.extended_table.insert(value, class);
    }
}