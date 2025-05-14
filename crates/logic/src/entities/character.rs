use std::cmp::Ordering;
use std::collections::HashMap;

use crate::abilities::{Ability, AbilityScores, DetailedAbilityScore};
use crate::races::Race;
// Add this line to import the Race type

pub struct Character {
    name: String,
    level: i32,
    class: String,
    race: Race,
    strength: DetailedAbilityScore,
    dexterity: DetailedAbilityScore,
    constitution: DetailedAbilityScore,
    intelligence: DetailedAbilityScore,
    wisdom: DetailedAbilityScore,
    charisma: DetailedAbilityScore,
}

impl AbilityScores for Character {
    fn get_ability_score(&self, ability: Ability) -> i32 {
        match ability {
            Ability::Strength => self.strength.effective_score(),
            Ability::Dexterity => self.dexterity.effective_score(),
            Ability::Constitution => self.constitution.effective_score(),
            Ability::Intelligence => self.intelligence.effective_score(),
            Ability::Wisdom => self.wisdom.effective_score(),
            Ability::Charisma => self.charisma.effective_score(),
        }
    }

    fn get_modifier(&self, ability: Ability) -> i32 {
        let score = self.get_ability_score(ability);
        (score - 10) / 2
    }

    fn get_raw_ability_score(&self, ability: Ability) -> i32 {
        match ability {
            Ability::Strength => self.strength.score,
            Ability::Dexterity => self.dexterity.score,
            Ability::Constitution => self.constitution.score,
            Ability::Intelligence => self.intelligence.score,
            Ability::Wisdom => self.wisdom.score,
            Ability::Charisma => self.charisma.score,
        }
    }

    fn get_detail_ability_score(&self, ability: Ability) -> DetailedAbilityScore {
        match ability {
            Ability::Strength => self.strength.clone(),
            Ability::Dexterity => self.dexterity.clone(),
            Ability::Constitution => self.constitution.clone(),
            Ability::Intelligence => self.intelligence.clone(),
            Ability::Wisdom => self.wisdom.clone(),
            Ability::Charisma => self.charisma.clone(),
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
            race: race.clone(),
            strength: race.calculate_ability_score(Ability::Strength, strength),
            dexterity: race.calculate_ability_score(Ability::Dexterity, dexterity),
            constitution: race.calculate_ability_score(Ability::Constitution, constitution),
            intelligence: race.calculate_ability_score(Ability::Intelligence, intelligence),
            wisdom: race.calculate_ability_score(Ability::Wisdom, wisdom),
            charisma: race.calculate_ability_score(Ability::Charisma, charisma),
        }
    }

    pub fn create(
        name: String,
        level: i32,
        class: String,
        race: Race,
        abilities: HashMap<Ability, i32>,
    ) -> Result<Self, String> {
        let mut character = Character::new(name, level, class, race, 10, 10, 10, 10, 10, 10);

        match abilities.values().sum::<i32>().cmp(&10) {
            Ordering::Less => {
                let msg = format!(
                    "Ability Purchases score total cannot be less than 10. Current: {}",
                    abilities.values().sum::<i32>()
                );
                return Err(msg);
            }
            Ordering::Greater => {
                let msg = format!(
                    "Ability Purchases score total cannot be greater than 10. Current: {}",
                    abilities.values().sum::<i32>()
                );
                return Err(msg);
            }
            _ => {}
        }

        for (ability, score_additive) in abilities {
            let base_score = character.get_ability_score(ability);
            let computed_score = base_score + score_additive;
            if computed_score > 18 {
                let msg = format!(
                    "Ability score {:#?} cannot exceed 18. Current: {}, Additive: {}",
                    ability, base_score, score_additive
                );
                return Err(msg);
            }

            match ability {
                Ability::Strength => character
                    .strength
                    .add_modifier("purchased".to_string(), score_additive),
                Ability::Dexterity => character
                    .dexterity
                    .add_modifier("purchased".to_string(), score_additive),
                Ability::Constitution => character
                    .constitution
                    .add_modifier("purchased".to_string(), score_additive),
                Ability::Intelligence => character
                    .intelligence
                    .add_modifier("purchased".to_string(), score_additive),
                Ability::Wisdom => character
                    .wisdom
                    .add_modifier("purchased".to_string(), score_additive),
                Ability::Charisma => character
                    .charisma
                    .add_modifier("purchased".to_string(), score_additive),
            }
        }

        Ok(character)
    }
}

#[cfg(test)]
mod tests {

    use std::clone;

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

    #[test]
    fn test_create_human_character_with_too_high_strength_score() {
        let races = load_default_races();
        let human = &races["Human"].clone();
        let abilities = HashMap::from([(Ability::Strength, 10)]);

        let character_result = Character::create(
            "Test".to_string(),
            1,
            "Fighter".to_string(),
            human.clone(),
            abilities,
        );
        assert!(character_result.is_err());

        let error_message = character_result.err().unwrap();
        assert_eq!(
            error_message,
            "Ability score Strength cannot exceed 18. Current: 11, Additive: 10"
        );
    }

    #[test]
    fn test_create_dwarf_character_with_very_high_charisma_score() {
        let races = load_default_races();
        let dwarf = &races["Dwarf"].clone();
        // dwarves get a -2 to charisma, so we can put 10p in and get 18 charisma.
        let abilities = HashMap::from([(Ability::Charisma, 10)]);

        let character_result = Character::create(
            "Test".to_string(),
            1,
            "Fighter".to_string(),
            dwarf.clone(),
            abilities,
        );
        assert!(character_result.is_ok());
        let character = character_result.unwrap();
        assert_eq!(character.get_ability_score(Ability::Charisma), 18);
        assert_eq!(character.get_modifier(Ability::Charisma), 4);
        assert_eq!(
            character
                .get_detail_ability_score(Ability::Charisma)
                .effective_score(),
            18
        );
        assert_eq!(
            character
                .get_detail_ability_score(Ability::Charisma)
                .modifiers,
            HashMap::from([("racial".to_string(), -2), ("purchased".to_string(), 10)])
        );
    }
}
