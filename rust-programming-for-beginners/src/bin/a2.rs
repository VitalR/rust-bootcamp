// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// * Use a function to display the result
fn print_result(result: i32) {
    println!("{:?}", result);
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let result = add(2, 3);
    print_result(result);
}
