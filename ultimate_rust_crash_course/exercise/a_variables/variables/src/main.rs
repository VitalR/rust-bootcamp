const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    //let not_user_var: i32;

    // missiles = STARTING_MISSILES;
    // ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);

    // READY_AMOUNT = 1;
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}