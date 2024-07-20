
/// Enum variants are public or private all together
pub enum Appetizer {
    Soup,
    Salad,
}


/// Structure fields are separately public or private
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}


impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("Peaches"),
        }
    }
}


fn cook_order() {}


fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}
