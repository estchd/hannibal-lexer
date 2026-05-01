use std::collections::HashMap;
use std::num::NonZeroUsize;
use crate::classification::class::ClassificationClass;
use crate::lexer::definition::LexerDefinition;
use crate::lexer::meta::LexerMeta;
use crate::lexer::state::LexerState;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct LexerBuilder {
    meta: LexerMeta,
    classifications: HashMap<char, ClassificationClass>,
    transitions: HashMap<LexerState, HashMap<ClassificationClass, LexerState>>,
    token_types: HashMap<LexerState, TokenType>,
    next_class: ClassificationClass,
    next_state: LexerState,
    next_type: TokenType,
}

impl LexerBuilder {
    pub fn new() -> Self {
        let mut meta = LexerMeta::new();

        meta.add_state("start".to_string(), LexerState::start_state());
        meta.add_class("default".to_string(), ClassificationClass::default_class());

        Self {
            meta: LexerMeta::new(),
            classifications: HashMap::new(),
            transitions: HashMap::new(),
            token_types: HashMap::new(),
            next_class: ClassificationClass::new(1),
            next_state: LexerState::new(1),
            next_type: TokenType::new(NonZeroUsize::new(1).unwrap()),
        }
    }

    pub fn add_classification(&mut self, value: char, class_name: &str) {
        let class = self.get_or_create_class(class_name);

        self.classifications.insert(value, class);
    }

    pub fn add_transition(&mut self, origin_state_name: &str, class_name: &str, new_state_name: &str) {
        let origin_state = self.get_or_create_state(origin_state_name);
        let class = self.get_or_create_class(class_name);
        let new_state = self.get_or_create_state(new_state_name);

        if !self.transitions.contains_key(&origin_state) {
            self.transitions.insert(origin_state, HashMap::new());
        }

        let transitions = self.transitions.get_mut(&origin_state).unwrap();

        transitions.insert(class, new_state);
    }

    pub fn add_token_type(&mut self, state_name: &str, token_type_name: &str) {
        let state = self.get_or_create_state(state_name);
        let token_type = self.get_or_create_type(token_type_name);

        self.token_types.insert(state, token_type);
    }

    pub fn build(self) -> (LexerDefinition, LexerMeta) {
        let state_count = self.next_state.get();

        let mut definition = LexerDefinition::new(state_count);

        for (value, class) in self.classifications {
            definition.add_classification(value, class);
        }

        for (origin_state, transitions) in self.transitions {
            for (class, new_state) in transitions {
                definition.add_transition(origin_state, class, new_state);
            }
        }

        for (state, token_type) in self.token_types {
            definition.add_token_type(state, token_type);
        }

        (definition, self.meta)
    }

    fn get_or_create_class(&mut self, class_name: &str) -> ClassificationClass {
        let existing_class = self.meta.get_class_from_name(class_name);

        if let Some(existing_class) = existing_class {
            existing_class
        }
        else {
            let class = self.next_class;

            self.increment_class();

            self.meta.add_class(class_name.to_string(), class);
            class
        }
    }

    fn get_or_create_state(&mut self, state_name: &str) -> LexerState {
        let existing_state = self.meta.get_state_from_name(state_name);

        if let Some(existing_state) = existing_state {
            existing_state
        }
        else {
            let state = self.next_state;

            self.increment_state();

            self.meta.add_state(state_name.to_string(), state);
            state
        }
    }

    fn get_or_create_type(&mut self, type_name: &str) -> TokenType {
        let existing_type = self.meta.get_type_from_name(type_name);

        if let Some(existing_type) = existing_type {
            existing_type
        }
        else {
            let token_type = self.next_type;

            self.increment_type();

            self.meta.add_type(type_name.to_string(), token_type);
            token_type
        }
    }

    fn increment_class(&mut self) {
        self.next_class = ClassificationClass::new(self.next_class.get() + 1);
    }

    fn increment_state(&mut self) {
        self.next_state = LexerState::new(self.next_state.get() + 1);
    }

    fn increment_type(&mut self) {
        let non_zero = self.next_type.get().checked_add(1).expect("Too many types");

        self.next_type = TokenType::new(non_zero);
    }
}