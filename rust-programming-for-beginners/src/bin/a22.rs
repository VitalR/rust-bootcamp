// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    /// Ensures n is >= lower and <= upper.
    #[test]
    fn check_clamp_1() {
        // (n: i32, lower: i32, upper: i32) -> i32
        let num = 45;
        let lower = 0;
        let upper = 100;
        let result = clamp(num, lower, upper);
        assert_eq!(num, result, "should be num");
    }

    #[test]
    fn check_clamp_2() {
        let num = 10;
        let lower = 50;
        let upper = 100;
        let result = clamp(num, lower, upper);
        assert_eq!(lower, result, "should be lower");
    }  

    #[test]
    fn check_clamp_3() {
        let num = 450;
        let lower = 50;
        let upper = 100;
        let result = clamp(num, lower, upper);
        assert_eq!(upper, result, "should be upper");
    } 

    #[test]
    fn check_div() {
        let a = 10;
        let b = 5;
        let result = div(a, b);
        let expected = Some(2);
        assert_eq!(expected, result, "should be 2");
    }

    #[test]
    fn check_div_2() {
        let a = 10;
        let b = 0;
        let result = div(a, b);
        let expected = None;
        assert_eq!(expected, result, "cannot divide on zero");
    }

    #[test]
    fn check_concat() {
        let s1 = "hi";
        let s2 = "there";
        let result = concat(s1, s2);
        // let expected = "hi there";
        let expected = String::from("hithere");
        assert_eq!(expected, result, "invalid");
    }
}