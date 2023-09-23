use crate::{
    cc::CanonicalCollection, grammar::Grammar, production::Production, sets_gen::SetsGenerator,
};

struct LRItem {
    production: Production,
    dot_position: usize,
    grammar: Grammar,
    can_col: CanonicalCollection,
    sets_gen: SetsGenerator,
}
