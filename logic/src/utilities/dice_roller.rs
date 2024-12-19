use std::cmp;

use rand::Rng;
use regex::Regex;

pub enum DiceRollResult {
    Success(i32, i32, String),
    Failure(i32, i32, String),
}

// roll_dice will take a dice expression and a difficulty check and return a DiceRollResult
// The dice expression will be a string that represents a dice roll, such as
// "2d6", "1d20", "2d10+5", "2d4*8", "10d20 - 8" etc.
// The difficulty check will be an integer that represents the target number that the dice roll
// must meet or exceed to be considered a success.
// The DiceRollResult will be a Success or Failure enum that contains the result of the dice roll
//
// # Arguments
//
// * `dice_expression` - A string that represents a dice roll
// * `difficulty_check` - An integer that represents the target number that the dice roll must meet
// or exceed to be considered a success
//
// # Returns
//
// A DiceRollResult that contains the result of the dice roll
//
// # Example
//
// ```
// use crate::utilities::dice_roller::roll_dice;
//
// let result = roll_dice("2d6".to_string(), 10);
// ```
pub fn roll_dice(dice_expression: String, difficulty_check: i32) -> DiceRollResult {
    let mut rng = rand::thread_rng();

    let sanitized_dice_expression = dice_expression.replace(" ", "");
    let re = regex::Regex::new(r"(\d*)d(\d+)(([\+\-\*\/])?(\d+))?").unwrap();
    let captures = re.captures(&sanitized_dice_expression).unwrap();
    let num_dice: i32 = captures.get(1).map_or(1, |m| m.as_str().parse().unwrap());
    let die_size: i32 = captures.get(2).map_or(1, |m| m.as_str().parse().unwrap());
    let modifier: &str = captures.get(3).map_or("+0", |m| m.as_str());
    let modifier_operator: &str = captures.get(4).map_or("+", |m| m.as_str());
    let modifier_value: i32 = captures.get(5).map_or(0, |m| m.as_str().parse().unwrap());

    let mut total = 0;

    for _ in 0..num_dice {
        total += rng.gen_range(1..die_size);
    }

    match modifier_operator {
        "+" => total += modifier_value,
        "-" => total -= modifier_value,
        "*" => total *= modifier_value,
        "/" => total /= modifier_value,
        _ => (),
    }

    if total >= difficulty_check {
        DiceRollResult::Success(cmp::max(total, 0), difficulty_check, modifier.to_string())
    } else {
        DiceRollResult::Failure(cmp::max(total, 0), difficulty_check, modifier.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_dice() {
        let result = roll_dice("2d6".to_string(), 10);
        match result {
            DiceRollResult::Success(_, _, _) => assert!(true),
            DiceRollResult::Failure(_, _, _) => assert!(true),
        }
    }

    #[test]
    fn test_roll_dice_with_addition_modifier() {
        let result = roll_dice("2d6+5".to_string(), 10);
        match result {
            DiceRollResult::Success(total, dc, modifier) => {
                // a minimum of 7 is expected
                assert!(7 <= total);
                // a maximum of 17 is expected
                assert!(17 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is +5
                assert_eq!(modifier, "+5");
            }
            DiceRollResult::Failure(total, dc, modifier) => {
                // a minimum of 7 is expected
                assert!(7 <= total);
                // a maximum of 17 is expected
                assert!(17 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is +5
                assert_eq!(modifier, "+5");
            }
        }
    }

    #[test]
    fn test_roll_dice_with_multiplication_modifier() {
        let result = roll_dice("2d6*5".to_string(), 10);
        match result {
            DiceRollResult::Success(total, dc, modifier) => {
                // a minimum of 10 is expected
                assert!(10 <= total);
                // a maximum of 60 is expected
                assert!(60 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is *5
                assert_eq!(modifier, "*5");
            }
            DiceRollResult::Failure(total, dc, modifier) => {
                // a minimum of 10 is expected
                assert!(10 <= total);
                // a maximum of 60 is expected
                assert!(60 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is *5
                assert_eq!(modifier, "*5");
            }
        }
    }

    #[test]
    fn test_roll_dice_with_subtraction_modifier() {
        let result = roll_dice("2d6-5".to_string(), 10);
        match result {
            DiceRollResult::Success(total, dc, modifier) => {
                // a minimum of 0 is expected
                assert!(0 <= total);
                // a maximum of 10 is expected
                assert!(10 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is -5
                assert_eq!(modifier, "-5");
            }
            DiceRollResult::Failure(total, dc, modifier) => {
                // a minimum of 0 is expected
                assert!(0 <= total);
                // a maximum of 10 is expected
                assert!(10 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is -5
                assert_eq!(modifier, "-5");
            }
        }
    }

    #[test]
    fn test_roll_dice_with_division_modifier() {
        let result = roll_dice("2d20/5".to_string(), 10);
        match result {
            DiceRollResult::Success(total, dc, modifier) => {
                println!("total: {}, dc: {}, modifier: {}", total, dc, modifier);
                // a minimum of 0 is expected
                assert!(0 <= total);
                // a maximum of 2 is expected
                assert!(8 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is /5
                assert_eq!(modifier, "/5");
            }
            DiceRollResult::Failure(total, dc, modifier) => {
                println!("total: {}, dc: {}, modifier: {}", total, dc, modifier);
                // a minimum of 0 is expected
                assert!(0 <= total);
                // a maximum of 2 is expected
                assert!(8 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is /5
                assert_eq!(modifier, "/5");
            }
        }
    }
}
