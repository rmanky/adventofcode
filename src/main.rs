use std::error::Error;

mod one;
mod two;
mod three;
mod four;

use two::two;
use one::one;
use three::three;
use four::four;

fn main() -> Result<(), Box<dyn Error>> {
    let option = std::env::args().nth(1).expect("Please select a day to run.");
    match option.as_str() {
        "one" => one(),
        "two" => two(),
        "three" => three(),
        "four" => four(),
        s => panic!("Invalid option: {}", s),
    }
}
