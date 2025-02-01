
mod greetings;
// use greetings::default_greeting;

use greetings::{french, spanish, default_greeting}

fn main() {
    println!("Hello, world!");
    println!("{}", default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
}
