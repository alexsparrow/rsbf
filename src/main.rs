extern crate rsbf;

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file() -> io::Result<String> {
    let mut f = try!(File::open("helloworld.bf"));
    let mut buffer = String::new();

    try!(f.read_to_string(&mut buffer));

    Ok(buffer)
}

fn main() {
    let mut state = rsbf::initial_state();
    let mut stdout = std::io::stdout();
    match read_file() {
        Ok(s) => rsbf::interpret(s.as_str(), &mut state, &mut stdout),
        Err(e) => panic!(e)
    }
}
