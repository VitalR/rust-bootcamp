// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Drinks {
    Apple,
    Chocolate,
    Lemonade,
    Citrus
}

struct DrinksInfo {
    drink_flavor: Drinks,
    fluid: f64,
}

fn display_drink(drink: DrinksInfo) {
    let drink_flavor = drink.drink_flavor;
    match drink_flavor {
        Drinks::Apple => println!("Apple"),
        Drinks::Chocolate => println!("Chocolate"),
        Drinks::Lemonade => println!("Lemonade"),
        Drinks::Citrus => println!("Citrus"),
    }
    println!("{:?}", drink.fluid);
}

fn main() {
    let lemonade = DrinksInfo {
        drink_flavor: Drinks::Lemonade,
        fluid: 2.97,
    };
    display_drink(lemonade);
}
