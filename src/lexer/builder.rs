use crate::classification::class::ClassificationClass;
use crate::lexer::definition::LexerDefinition;
use crate::lexer::meta::LexerMeta;
use crate::lexer::state::LexerState;
use crate::token_type::TokenType;
use std::collections::HashMap;
use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
pub struct LexerBuilder {
    classifications: HashMap<char, ClassificationClass>,
    meta: LexerMeta,
    next_class: ClassificationClass,
    next_state: LexerState,
    next_type: TokenType,
    token_types: HashMap<LexerState, TokenType>,
    transitions: HashMap<LexerState, HashMap<ClassificationClass, LexerState>>,
}

impl LexerBuilder {
    #[inline]
    pub fn add_classification(
        &mut self,
        value: char,
        class_name: &str,
    ) {
        let class = self.get_or_create_class(class_name);

        self.classifications
            .insert(value, class);
    }

    #[inline]
    pub fn add_token_type(
        &mut self,
        state_name: &str,
        token_type_name: &str,
    ) {
        let state = self.get_or_create_state(state_name);
        let token_type = self.get_or_create_type(token_type_name);

        self.token_types
            .insert(state, token_type);
    }

    #[inline]
    pub fn add_transition(
        &mut self,
        origin_state_name: &str,
        class_name: &str,
        new_state_name: &str,
    ) {
        let origin_state = self.get_or_create_state(origin_state_name);
        let class = self.get_or_create_class(class_name);
        let new_state = self.get_or_create_state(new_state_name);

        let transitions = self
            .transitions
            .entry(origin_state)
            .or_default();

        transitions.insert(class, new_state);
    }

    #[inline]
    #[must_use]
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

    fn get_or_create_class(
        &mut self,
        class_name: &str,
    ) -> ClassificationClass {
        let existing_class = self
            .meta
            .get_class_from_name(class_name);

        if let Some(existing_class) = existing_class {
            existing_class
        } else {
            let class = self.next_class;

            self.increment_class();

            self.meta
                .add_class(class_name.to_owned(), class);
            class
        }
    }

    fn get_or_create_state(
        &mut self,
        state_name: &str,
    ) -> LexerState {
        let existing_state = self
            .meta
            .get_state_from_name(state_name);

        if let Some(existing_state) = existing_state {
            existing_state
        } else {
            let state = self.next_state;

            self.increment_state();

            self.meta
                .add_state(state_name.to_owned(), state);
            state
        }
    }

    fn get_or_create_type(
        &mut self,
        type_name: &str,
    ) -> TokenType {
        let existing_type = self
            .meta
            .get_type_from_name(type_name);

        if let Some(existing_type) = existing_type {
            existing_type
        } else {
            let token_type = self.next_type;

            self.increment_type();

            self.meta
                .add_type(type_name.to_owned(), token_type);
            token_type
        }
    }

    /// # Panics
    ///
    /// - Will panic if the number of classes exceeds `usize::MAX`
    fn increment_class(&mut self) {
        let class_index = self.next_class.get();

        #[expect(
            clippy::expect_used,
            reason = "This expect only panics if the number of types exceeds usize::MAX, if that ever happens, a panic is fine for now"
        )]
        let next_class_index = class_index
            .checked_add(1)
            .expect("Too many classes");

        self.next_class = ClassificationClass::new(next_class_index);
    }

    /// # Panics
    ///
    /// - Will panic if the number of classes exceeds `usize::MAX`
    fn increment_state(&mut self) {
        let state_index = self.next_state.get();

        #[expect(
            clippy::expect_used,
            reason = "This expect only panics if the number of types exceeds usize::MAX, if that ever happens, a panic is fine for now"
        )]
        let next_state_index = state_index
            .checked_add(1)
            .expect("Too many states");

        self.next_state = LexerState::new(next_state_index);
    }

    fn increment_type(&mut self) {
        #[expect(
            clippy::expect_used,
            reason = "This expect only panics if the number of types exceeds usize::MAX, if that ever happens, a panic is fine for now"
        )]
        let non_zero = self
            .next_type
            .get()
            .checked_add(1)
            .expect("Too many types");

        self.next_type = TokenType::new(non_zero);
    }

    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let mut meta = LexerMeta::new();

        meta.add_state("start".to_owned(), LexerState::start_state());
        meta.add_class("default".to_owned(), ClassificationClass::default_class());

        #[expect(
            clippy::missing_panics_doc,
            clippy::expect_used,
            reason = "NonZeroUsize::new(1) should never panic since the value is a constant and non-zero"
        )]
        let next_type_index = NonZeroUsize::new(1).expect("NonZeroUsize::new(1) should never panic since the value is a constant and non-zero");

        Self {
            meta: LexerMeta::new(),
            classifications: HashMap::new(),
            transitions: HashMap::new(),
            token_types: HashMap::new(),
            next_class: ClassificationClass::new(1),
            next_state: LexerState::new(1),
            next_type: TokenType::new(next_type_index),
        }
    }
}

impl Default for LexerBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
