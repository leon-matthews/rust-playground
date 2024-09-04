#![allow(dead_code)]
#![allow(unused_variables)]


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Blue,
    Red,
}


struct Inventory {
    shirts: Vec<ShirtColor>,
}


impl Inventory {
    /// Pick which colour to give user. Either the user preference, or the
    /// shirt we have the most stock of.
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    /// The shirt colour tha we have the most of in stock.
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for colour in &self.shirts {
            match colour {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red { ShirtColor::Blue } else { ShirtColor::Red }
    }
}


fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
        ],
    };

    // This user wants a red shirt
    let user_pref = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_pref);
    println!("The user with preference {:?} gets {:?}", user_pref, giveaway);

    // This user doesn't care
    let user_pref = None;
    let giveaway = store.giveaway(user_pref);
    println!("The user with preference {:?} gets {:?}", user_pref, giveaway);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_store() -> Inventory {
         Inventory {
            shirts: vec![
                ShirtColor::Blue,
                ShirtColor::Red,
                ShirtColor::Blue,
            ],
        }
    }

    #[test]
    /// Find the shirt colour we have the most of
    fn most_stocked() {
        let store = make_store();
        assert_eq!(store.most_stocked(), ShirtColor::Blue);
    }

    #[test]
    /// In the absence of a user preference, return the most-stocked colour
    fn giveway_default() {
        let store = make_store();
        assert_eq!(store.giveaway(None), ShirtColor::Blue);
    }

    #[test]
    /// Respect the user's preference, return their desired color
    fn giveway_preference() {
        let store = make_store();
        assert_eq!(store.giveaway(Some(ShirtColor::Red)), ShirtColor::Red);
    }
}
