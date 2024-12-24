use std::{cmp, fmt};

use rand::Rng;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct DiceRollError {
    message: String,
}

impl fmt::Display for DiceRollError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid dice expression: {}", self.message)
    }
}

pub enum DiceRoll {
    Success(i32, i32, String),
    Failure(i32, i32, String),
}

pub type DiceRollResult = Result<DiceRoll, DiceRollError>;

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
// A DiceRollResult that contains the result of the dice roll.  If the dice roll meets or exceeds
// the dc then the result will be a Success enum that contains the total of the dice roll, the dc,
// and the modifier.  If the dice roll does not meet the dc then the result will be a Failure enum
// that contains the total of the dice roll, the dc, and the modifier.
// The total can never be less than 0. `1d4-5` will return 0.
//
// # Example
//
// ```
// use crate::utilities::dice_roller::roll_dice_check;
//
// let result = roll_dice_check("2d6".to_string(), 10);
// match result {
//     Err(_) => println!("Invalid dice expression"),
//     Ok(roll) => match roll {
//         DiceRoll::Success(total, dc, modifier) => {
//             println!("Success! Total: {}, DC: {}, Modifier: {}", total, dc, modifier);
//         },
//         DiceRoll::Failure(total, dc, modifier) => {
//             println!("Failure! Total: {}, DC: {}, Modifier: {}", total, dc, modifier);
//         },
//     },
// }
// ```
pub fn roll_dice_check(dice_expression: String, difficulty_check: i32) -> DiceRollResult {
    let mut rng = rand::thread_rng();

    let sanitized_dice_expression = dice_expression.replace(" ", "");
    let re = Regex::new(r"^(\d*)d(\d+)([\+\-\*\/]\d+)?$").unwrap();

    let captures = match re.captures(&sanitized_dice_expression) {
        Some(caps) => caps,
        None => {
            return Err(DiceRollError {
                message: "invalid dice expression".to_string(),
            });
        }
    };

    let num_dice: i32 = captures
        .get(1)
        .map_or(1, |m| m.as_str().parse().unwrap_or(1));
    let die_size: i32 = captures
        .get(2)
        .map_or(1, |m| m.as_str().parse().unwrap_or(1));
    let modifier_str = captures.get(3).map_or("+0", |m| m.as_str());

    let (modifier_operator, modifier_value) = if modifier_str.len() > 1 {
        let operator = &modifier_str[0..1];
        let value = modifier_str[1..].parse().unwrap_or(0);
        (operator, value)
    } else {
        ("+", 0)
    };

    if num_dice == 0 || die_size == 0 {
        return Ok(DiceRoll::Failure(0, difficulty_check, "+0".to_string()));
    }

    let mut total = 0;

    for _ in 0..num_dice {
        total += rng.gen_range(1..=die_size);
    }

    match modifier_operator {
        "+" => total += modifier_value,
        "-" => total -= modifier_value,
        "*" => total *= modifier_value,
        "/" => total /= modifier_value,
        _ => (),
    }

    if total >= difficulty_check {
        Ok(DiceRoll::Success(
            cmp::max(total, 0),
            difficulty_check,
            modifier_str.to_string(),
        ))
    } else {
        Ok(DiceRoll::Failure(
            cmp::max(total, 0),
            difficulty_check,
            modifier_str.to_string(),
        ))
    }
}

pub fn roll_dice(dice_expression: String) -> i32 {
    let result = roll_dice_check(dice_expression, 0);
    match result {
        Ok(DiceRoll::Success(total, _, _)) | Ok(DiceRoll::Failure(total, _, _)) => total,
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_dice() {
        let result = roll_dice_check("2d6".to_string(), 10);
        match result {
            Err(_) => assert!(false),
            Ok(roll) => match roll {
                DiceRoll::Success(_, _, _) => assert!(true),
                DiceRoll::Failure(_, _, _) => assert!(true),
            },
        }
    }

    #[test]
    fn test_roll_dice_with_addition_modifier() {
        let result = roll_dice_check("2d6+5".to_string(), 10);
        match result {
            Ok(DiceRoll::Success(total, dc, modifier))
            | Ok(DiceRoll::Failure(total, dc, modifier)) => {
                // a minimum of 7 is expected
                assert!(7 <= total);
                // a maximum of 17 is expected
                assert!(17 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is +5
                assert_eq!(modifier, "+5");
            }
            Err(_) => assert!(false, "Expected a valid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_multiplication_modifier() {
        let result = roll_dice_check("2d6*5".to_string(), 10);
        match result {
            Ok(DiceRoll::Success(total, dc, modifier))
            | Ok(DiceRoll::Failure(total, dc, modifier)) => {
                // a minimum of 10 is expected
                assert!(10 <= total);
                // a maximum of 60 is expected
                assert!(60 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is *5
                assert_eq!(modifier, "*5");
            }
            Err(_) => assert!(false, "Expected a valid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_subtraction_modifier() {
        let result = roll_dice_check("2d6-5".to_string(), 10);
        match result {
            Ok(DiceRoll::Success(total, dc, modifier))
            | Ok(DiceRoll::Failure(total, dc, modifier)) => {
                // a minimum of 0 is expected
                assert!(0 <= total);
                // a maximum of 7 is expected
                assert!(12 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is -5
                assert_eq!(modifier, "-5");
            }
            Err(_) => assert!(false, "Expected a valid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_division_modifier() {
        let result = roll_dice_check("2d20/5".to_string(), 10);
        match result {
            Ok(DiceRoll::Success(total, dc, modifier))
            | Ok(DiceRoll::Failure(total, dc, modifier)) => {
                // a minimum of 0 is expected
                assert!(0 <= total);
                // a maximum of 8 is expected
                assert!(8 >= total);
                // the difficulty check is 10
                assert_eq!(dc, 10);
                // the modifier is /5
                assert_eq!(modifier, "/5");
            }
            Err(_) => assert!(false, "Expected a valid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_invalid_dice_expression() {
        let result = roll_dice_check("2d6+5+5".to_string(), 10);
        match result {
            Err(_) => assert!(true),
            Ok(_) => assert!(false, "Expected an invalid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_invalid_dice_expression_2() {
        let result = roll_dice_check("2d6+5-".to_string(), 10);
        match result {
            Err(_) => assert!(true),
            Ok(_) => assert!(false, "Expected an invalid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_single_die() {
        let result = roll_dice_check("1d6".to_string(), 4);
        match result {
            Ok(DiceRoll::Success(total, dc, modifier))
            | Ok(DiceRoll::Failure(total, dc, modifier)) => {
                // a minimum of 1 is expected
                assert!(1 <= total);
                // a maximum of 6 is expected
                assert!(6 >= total);
                // the difficulty check is 4
                assert_eq!(dc, 4);
                // the modifier is +0
                assert_eq!(modifier, "+0");
            }
            Err(_) => assert!(false, "Expected a valid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_large_number_of_dice() {
        let result = roll_dice_check("100d6".to_string(), 300);
        match result {
            Ok(DiceRoll::Success(total, dc, modifier))
            | Ok(DiceRoll::Failure(total, dc, modifier)) => {
                // a minimum of 100 is expected
                assert!(100 <= total);
                // a maximum of 600 is expected
                assert!(600 >= total);
                // the difficulty check is 300
                assert_eq!(dc, 300);
                // the modifier is +0
                assert_eq!(modifier, "+0");
            }
            Err(_) => assert!(false, "Expected a valid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_zero_dice() {
        let result = roll_dice_check("0d6".to_string(), 1);
        match result {
            Ok(DiceRoll::Failure(total, dc, modifier)) => {
                // the total should be 0
                assert_eq!(total, 0);
                // the difficulty check is 1
                assert_eq!(dc, 1);
                // the modifier is +0
                assert_eq!(modifier, "+0");
            }
            _ => assert!(false, "Expected a valid dice roll result"),
        }
    }

    #[test]
    fn test_roll_dice_with_zero_sided_die() {
        let result = roll_dice_check("1d0".to_string(), 1);
        match result {
            Ok(DiceRoll::Failure(total, dc, modifier)) => {
                // the total should be 0
                assert_eq!(total, 0);
                // the difficulty check is 1
                assert_eq!(dc, 1);
                // the modifier is +0
                assert_eq!(modifier, "+0");
            }
            _ => assert!(false, "Expected a valid dice roll result"),
        }
    }
}
