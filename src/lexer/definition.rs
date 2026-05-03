use crate::classification::class::ClassificationClass;
use crate::classification::table::ClassificationTable;
use crate::lexer::state::LexerState;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct LexerDefinition {
    classification: ClassificationTable,
    token_type: Vec<Option<TokenType>>,
    transition: Vec<Vec<(ClassificationClass, LexerState)>>,
}

impl LexerDefinition {
    #[inline]
    pub fn add_classification(
        &mut self,
        value: char,
        class: ClassificationClass,
    ) {
        self.classification
            .add_classification(value, class);
    }

    #[inline]
    pub fn add_token_type(
        &mut self,
        state: LexerState,
        token_type: TokenType,
    ) {
        self.token_type[state.get()] = Some(token_type);
    }

    #[inline]
    pub fn add_transition(
        &mut self,
        origin_state: LexerState,
        class: ClassificationClass,
        new_state: LexerState,
    ) {
        let transitions = &mut self.transition[origin_state.get()];

        for (transition_class, transition_state) in transitions.iter_mut() {
            if *transition_class == class {
                *transition_state = new_state;
                return;
            }
        }

        transitions.push((class, new_state));

        transitions.sort_by_key(|(class, _)| *class);
    }

    #[inline]
    #[must_use]
    pub fn classify(
        &self,
        value: char,
    ) -> ClassificationClass {
        self.classification
            .classify(value)
    }

    #[inline]
    #[must_use]
    pub fn is_final_state(
        &self,
        state: LexerState,
    ) -> bool {
        self.token_type[state.get()].is_some()
    }

    #[inline]
    #[must_use]
    pub fn new(state_count: usize) -> Self {
        let mut transition = Vec::with_capacity(state_count);

        for _ in 0..state_count {
            transition.push(vec![]);
        }

        Self {
            classification: ClassificationTable::new(),
            transition,
            token_type: vec![None; state_count],
        }
    }

    #[inline]
    #[must_use]
    pub fn state_to_token_type(
        &self,
        state: LexerState,
    ) -> Option<TokenType> {
        self.token_type[state.get()]
    }

    #[inline]
    #[must_use]
    pub fn transition(
        &self,
        state: LexerState,
        class: ClassificationClass,
    ) -> Option<LexerState> {
        let transitions = &self.transition[state.get()];

        for (transition_class, new_state) in transitions {
            if *transition_class == class {
                return Some(*new_state);
            }

            if *transition_class > class {
                break;
            }
        }

        None
    }

    #[inline]
    #[must_use]
    pub fn transition_by_char(
        &self,
        state: LexerState,
        value: char,
    ) -> Option<LexerState> {
        let class = self
            .classification
            .classify(value);

        self.transition(state, class)
    }
}
