use std::error::Error;

mod one;
mod two;

use two::two;
use one::one;

fn main() -> Result<(), Box<dyn Error>> {
    let option = std::env::args().nth(1).expect("Please select a day to run.");
    match option.as_str() {
        "one" => one(),
        "two" => two(),
        s => panic!("Invalid option: {}", s),
    }
}
