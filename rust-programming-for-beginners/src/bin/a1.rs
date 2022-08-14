// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn display_first_name() {
    let first_name = "Vital";

    println!("{}", first_name);
}

fn display_last_name() {
    let last_name = "R";

    println!("{}", last_name);
}

fn main() {
    display_first_name();
    display_last_name();
}
