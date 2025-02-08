mod cl_input;
use cl_input::*;

fn main() {
    let input = ClInput::parse();
    println!("{:?}", input);
}

