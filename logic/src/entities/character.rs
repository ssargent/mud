use std::collections::HashMap;

use crate::abilities::{Ability, AbilityScores, DetailedAbilityScore};
use crate::races::Race; // Add this line to import the Race type

pub struct Character {
    name: String,
    level: i32,
    class: String,
    race: Race,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

impl AbilityScores for Character {
    fn get_ability_score(&self, ability: Ability) -> i32 {
        let score = self.get_raw_ability_score(ability);
        let ability_score_modifier = self.race.ability_modifiers.get(&ability);
        match ability_score_modifier {
            Some(modifier) => score + modifier,
            None => score,
        }
    }

    fn get_modifier(&self, ability: Ability) -> i32 {
        let score = self.get_ability_score(ability);
        (score - 10) / 2
    }

    fn get_raw_ability_score(&self, ability: Ability) -> i32 {
        match ability {
            Ability::Strength => self.strength,
            Ability::Dexterity => self.dexterity,
            Ability::Constitution => self.constitution,
            Ability::Intelligence => self.intelligence,
            Ability::Wisdom => self.wisdom,
            Ability::Charisma => self.charisma,
        }
    }

    fn get_detail_ability_score(&self, ability: Ability) -> DetailedAbilityScore {
        let score = self.get_raw_ability_score(ability);
        let ability_score_modifier = self.race.ability_modifiers.get(&ability);
        match ability_score_modifier {
            Some(modifier) => {
                let mut detailed = DetailedAbilityScore {
                    ability,
                    score: score + modifier,
                    modifiers: HashMap::new(),
                };
                detailed.modifiers.insert("racial".to_string(), *modifier);
                detailed
            }
            None => DetailedAbilityScore {
                ability,
                score: score,
                modifiers: HashMap::new(),
            },
        }
    }
}

impl Character {
    pub fn new(
        name: String,
        level: i32,
        class: String,
        race: Race,
        strength: i32,
        dexterity: i32,
        constitution: i32,
        intelligence: i32,
        wisdom: i32,
        charisma: i32,
    ) -> Self {
        Character {
            name,
            level,
            class,
            race,
            strength,
            dexterity,
            constitution,
            intelligence,
            wisdom,
            charisma,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::char;

    use super::*;
    use crate::{abilities::ability::Ability, load_default_races};

    #[test]
    fn test_get_ability_score() {
        let races = load_default_races();
        let human = &races["Human"].clone();

        let character = Character::new(
            "Test".to_string(),
            1,
            "Fighter".to_string(),
            human.clone(),
            10,
            10,
            10,
            10,
            10,
            10,
        );

        assert_eq!(character.get_ability_score(Ability::Strength), 11);
        assert_eq!(character.get_ability_score(Ability::Dexterity), 11);
        assert_eq!(character.get_ability_score(Ability::Constitution), 11);
        assert_eq!(character.get_ability_score(Ability::Intelligence), 11);
        assert_eq!(character.get_ability_score(Ability::Wisdom), 11);
        assert_eq!(character.get_ability_score(Ability::Charisma), 11);
    }
}
