use crate::abilities::ability::Ability;
use crate::abilities::detailed_ability_score::DetailedAbilityScore;

pub trait AbilityScores {
    fn get_ability_score(&self, ability: Ability) -> i32;
    fn get_raw_ability_score(&self, ability: Ability) -> i32;
    fn get_detail_ability_score(&self, ability: Ability) -> DetailedAbilityScore;
    fn get_modifier(&self, ability: Ability) -> i32;
}
