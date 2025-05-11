use crate::abilities::ability::Ability;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DetailedAbilityScore {
    pub ability: Ability,
    pub score: i32,
    pub modifiers: HashMap<String, i32>,
}
