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
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
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

    fn get_store() -> Inventory {
        Inventory { shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue], }
    }

    #[test]
    fn test_specified_preference() {

        assert_eq!(
            get_store().return_preference(Some(ShirtColor::Red)),
            ShirtColor::Red
        )
    }
    
    #[test]
    fn test_no_preference() {

        assert_eq!(
            get_store().return_preference(None),
            ShirtColor::Blue
        )
    }
}
