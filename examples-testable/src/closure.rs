#![allow(dead_code)]

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn return_preference(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.get_most_stocked_shirt_color())
    }

    fn get_most_stocked_shirt_color(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up_blue_majority_store() -> Inventory {
        Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        }
    }

    #[test]
    fn test_specified_red_shirt_preference() {
        let test_store: Inventory = set_up_blue_majority_store();
        assert_eq!(
            test_store.return_preference(Some(ShirtColor::Red)),
            ShirtColor::Red
        );
    }

    #[test]
    fn test_no_preference() {
        let test_store: Inventory = set_up_blue_majority_store();
        assert_eq!(test_store.return_preference(None), ShirtColor::Blue);
    }
}
