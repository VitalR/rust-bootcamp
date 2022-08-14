// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(result: bool) {
    match result {
        true => println!("its small"),
        false => println!("its big"),
    }
}

fn main() {
    let num = 100;
    let mut result = true;
    
    let result = num <= 100;    // true || false
    // if num <= 100 {
    //     result = true;
    // } else if num > 100 {
    //     result = false;
    // }

    print_message(result);
}
