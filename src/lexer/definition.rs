use crate::classification::class::ClassificationClass;
use crate::classification::table::ClassificationTable;
use crate::lexer::state::LexerState;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct LexerDefinition {
    classification_table: ClassificationTable,
    transition_table: Vec<Vec<(ClassificationClass, LexerState)>>,
    token_type_table: Vec<Option<TokenType>>,
}

impl LexerDefinition {
    pub fn new(state_count: usize) -> Self {
        let mut transition_table = Vec::with_capacity(state_count);

        for _ in 0..state_count {
            transition_table.push(vec![]);
        }

        Self {
            classification_table: ClassificationTable::new(),
            transition_table,
            token_type_table: vec![None; state_count],
        }
    }

    pub fn classify(&self, value: char) -> ClassificationClass {
        self.classification_table.classify(value)
    }

    pub fn transition(&self, state: LexerState, class: ClassificationClass) -> Option<LexerState> {
        let transitions = &self.transition_table[state.get()];

        for (transition_class, new_state) in transitions {
            if *transition_class == class {
                return Some(*new_state);
            }

            if *transition_class > class {
                break;
            }
        }

        return None;
    }

    pub fn transition_by_char(&self, state: LexerState, value: char) -> Option<LexerState> {
        let class = self.classification_table.classify(value);

        self.transition(state, class)
    }

    pub fn is_final_state(&self, state: LexerState) -> bool {
        self.token_type_table[state.get()].is_some()
    }

    pub fn state_to_token_type(&self, state: LexerState) -> Option<TokenType> {
        self.token_type_table[state.get()]
    }

    pub fn add_classification(&mut self, value: char, class: ClassificationClass) {
        self.classification_table.add_classification(value, class);
    }

    pub fn add_transition(&mut self, origin_state: LexerState, class: ClassificationClass, new_state: LexerState) {
        let transitions = &mut self.transition_table[origin_state.get()];

        let mut index = 0;

        for (transition_class, transition_state) in &mut *transitions {
            if *transition_class == class {
                *transition_state = new_state;
                return;
            }

            if *transition_class > class {
                break;
            }

            index += 1;
        }

        transitions.insert(index, (class, new_state));
    }

    pub fn add_token_type(&mut self, state: LexerState, token_type: TokenType) {
        self.token_type_table[state.get()] = Some(token_type);
    }
}