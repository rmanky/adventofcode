use std::error::Error;

mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;

use two::two;
use one::one;
use three::three;
use four::four;
use five::five;
use six::six;
use seven::seven;
use eight::eight;
use nine::nine;
use ten::ten;

fn main() -> Result<(), Box<dyn Error>> {
    let option = std::env::args().nth(1).expect("Please select a day to run.");
    match option.as_str() {
        "one" => one(),
        "two" => two(),
        "three" => three(),
        "four" => four(),
        "five" => five(),
        "six" => six(),
        "seven" => seven(),
        "eight" => eight(),
        "nine" => nine(),
        "ten" => ten(),
        s => panic!("Invalid option: {}", s),
    }
}
