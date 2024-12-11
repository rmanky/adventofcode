use std::error::Error;

mod eight;
mod eleven;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
mod three;
mod two;

use eight::eight;
use eleven::eleven;
use five::five;
use four::four;
use nine::nine;
use one::one;
use seven::seven;
use six::six;
use ten::ten;
use three::three;
use two::two;

fn main() -> Result<(), Box<dyn Error>> {
    let option = std::env::args()
        .nth(1)
        .expect("Please select a day to run.");
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
        "eleven" => eleven(),
        s => panic!("Invalid option: {}", s),
    }
}
