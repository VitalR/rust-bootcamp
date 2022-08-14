// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

fn make_purchase(age: i32) -> Result<String, String> {
    if age >= 21 {
        Ok("the purchase is availible".to_owned())
    } else {
        Err("unable to make a purchase".to_owned())
    }
}

fn main() {
    let customer = Customer { age: 20 };
    let is_able = make_purchase(customer.age);

    match is_able {
        Ok(y) => println!("{:?}", y),
        Err(e) => println!("{:?}", e),
    }
}
