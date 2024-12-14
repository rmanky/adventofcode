use std::error::Error;

mod eight;
mod eleven;
mod five;
mod four;
mod fourteen;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
mod thirteen;
mod three;
mod twelve;
mod two;

use anyhow::Result;
use eight::eight;
use eleven::eleven;
use five::five;
use four::four;
use fourteen::fourteen;
use nine::nine;
use one::one;
use seven::seven;
use six::six;
use ten::ten;
use thirteen::thirteen;
use three::three;
use twelve::twelve;
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
        "twelve" => twelve(),
        "thirteen" => thirteen(),
        "fourteen" => fourteen(),
        s => panic!("Invalid option: {}", s),
    }
}
