use crate::abilities::ability::Ability;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DetailedAbilityScore {
    pub ability: Ability,
    pub score: i32,
    pub modifiers: HashMap<String, i32>,
}

impl DetailedAbilityScore {
    pub fn new(ability: Ability, score: i32) -> Self {
        Self {
            ability,
            score,
            modifiers: HashMap::new(),
        }
    }

    pub fn add_modifier(&mut self, source: String, modifier: i32) {
        self.modifiers.insert(source, modifier);
    }

    pub fn get_modifier(&self, source: &str) -> Option<&i32> {
        self.modifiers.get(source)
    }
    pub fn effective_score(&self) -> i32 {
        let mut total = self.score;
        for modifier in self.modifiers.values() {
            total += modifier;
        }
        total
    }
}
