// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// #[derive(Debug)]
enum Ticket {
    Standard(f64),
    Backstage(String, f64),
    Vip(String, f64),
}

fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Standard(25.99),
        Ticket::Backstage("Alice".to_owned(), 35.99),
        Ticket::Vip("Bob".to_owned(), 50.99),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("standard ticket price {:?}", price),
            Ticket::Backstage(name, price) => println!("backstage ticket name {:?} and price {:?}", name, price),
            Ticket::Vip(name, price) => println!("vip ticket name {:?} and price {:?}", name, price),
        }
    }
}
