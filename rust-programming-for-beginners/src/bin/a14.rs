// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print(data: &str) {
    println!("{:?} ", data);
}

fn main() {
    let persons = vec![
        Person {
            age: 27,
            name: "Alice".to_owned(),
            color: String::from("Ping"),
        },
        Person {
            age: 30,
            name: "Bob".to_owned(),
            color: String::from("Blue"),
        },
        Person {
            age: 7,
            name: "Steve".to_owned(),
            color: String::from("Orange"),
        },
    ];

    for person in persons {
        if person.age <= 10 {
            // println!("{:?} {:?} {:?}", person.age, person.name, person.color);
            // println!("{:?} ", person.age);
            print(&person.name);
            print(&person.color);
        }
    }
}
