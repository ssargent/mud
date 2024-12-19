use crate::abilities::ability::Ability;
use crate::races::race::Race;

pub fn load_default_races() -> std::collections::HashMap<String, Race> {
    [
        (
            "Dwarf".to_string(),
            Race::new(
                "Dwarf",
                [(Ability::Constitution, 2), (Ability::Charisma, -2)]
                    .iter()
                    .cloned()
                    .collect(),
                vec![
                    "Darkvision".to_string(),
                    "Dwarven Resilience".to_string(),
                    "Dwarven Combat Training".to_string(),
                    "Stonecunning".to_string(),
                ],
                25,
            ),
        ),
        (
            "Elf".to_string(),
            Race::new(
                "Elf",
                [(Ability::Dexterity, 2), (Ability::Constitution, -2)]
                    .iter()
                    .cloned()
                    .collect(),
                vec![
                    "Darkvision".to_string(),
                    "Keen Senses".to_string(),
                    "Fae Ancestry".to_string(),
                    "Trance".to_string(),
                ],
                30,
            ),
        ),
        (
            "Halfling".to_string(),
            Race::new(
                "Halfling",
                [(Ability::Constitution, 2)].iter().cloned().collect(),
                vec![
                    "Lucky".to_string(),
                    "Brave".to_string(),
                    "Halfling Nimbleness".to_string(),
                ],
                25,
            ),
        ),
        (
            "Human".to_string(),
            Race::new(
                "Human",
                [
                    (Ability::Strength, 1),
                    (Ability::Dexterity, 1),
                    (Ability::Constitution, 1),
                    (Ability::Intelligence, 1),
                    (Ability::Wisdom, 1),
                    (Ability::Charisma, 1),
                ]
                .iter()
                .cloned()
                .collect(),
                vec!["odd elections".to_string()],
                30,
            ),
        ),
    ]
    .iter()
    .cloned()
    .collect()
}
