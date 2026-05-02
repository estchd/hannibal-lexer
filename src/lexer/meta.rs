use crate::classification::class::ClassificationClass;
use crate::lexer::state::LexerState;
use crate::token_type::TokenType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LexerMeta {
    class_to_name: HashMap<ClassificationClass, String>,
    name_to_class: HashMap<String, ClassificationClass>,
    name_to_state: HashMap<String, LexerState>,
    name_to_type: HashMap<String, TokenType>,
    state_to_name: HashMap<LexerState, String>,
    type_to_name: HashMap<TokenType, String>,
}

impl LexerMeta {
    pub fn add_class(
        &mut self,
        name: String,
        class: ClassificationClass,
    ) {
        self.class_to_name
            .insert(class, name.clone());
        self.name_to_class
            .insert(name, class);
    }

    pub fn add_state(
        &mut self,
        name: String,
        state: LexerState,
    ) {
        self.state_to_name
            .insert(state, name.clone());
        self.name_to_state
            .insert(name, state);
    }

    pub fn add_type(
        &mut self,
        name: String,
        token_type: TokenType,
    ) {
        self.type_to_name
            .insert(token_type, name.clone());
        self.name_to_type
            .insert(name, token_type);
    }

    #[must_use]
    pub fn get_class_from_name(
        &self,
        name: &str,
    ) -> Option<ClassificationClass> {
        self.name_to_class
            .get(name)
            .copied()
    }

    #[must_use]
    pub fn get_name_from_class(
        &self,
        class: ClassificationClass,
    ) -> Option<String> {
        self.class_to_name
            .get(&class)
            .cloned()
    }

    #[must_use]
    pub fn get_name_from_state(
        &self,
        state: LexerState,
    ) -> Option<String> {
        self.state_to_name
            .get(&state)
            .cloned()
    }

    #[must_use]
    pub fn get_name_from_type(
        &self,
        token_type: TokenType,
    ) -> Option<String> {
        self.type_to_name
            .get(&token_type)
            .cloned()
    }

    #[must_use]
    pub fn get_state_from_name(
        &self,
        name: &str,
    ) -> Option<LexerState> {
        self.name_to_state
            .get(name)
            .copied()
    }

    #[must_use]
    pub fn get_type_from_name(
        &self,
        name: &str,
    ) -> Option<TokenType> {
        self.name_to_type
            .get(name)
            .copied()
    }

    #[must_use]
    pub fn new() -> Self {
        Self {
            name_to_state: HashMap::default(),
            state_to_name: HashMap::default(),
            name_to_class: HashMap::default(),
            class_to_name: HashMap::default(),
            name_to_type: HashMap::default(),
            type_to_name: HashMap::default(),
        }
    }
}

impl Default for LexerMeta {
    fn default() -> Self {
        Self::new()
    }
}
