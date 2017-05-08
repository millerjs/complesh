extern crate clap;
extern crate complesh;
extern crate termion;

use termion::{color, style};
use complesh::dropdown::{Dropdown, DropdownPrompt};
use complesh::readkeys::{Readkeys, ReadEvent, Printable};
use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
use clap::{Arg, App};


fn get_prompt() -> String {
    format!("{}{}enter text: {}{}",
            style::Bold, color::Fg(color::Blue),
            color::Fg(color::Reset), style::Reset)
}


fn main() {
    let matches = App::new("complesh")
        .version("0.1.0")
        .author("Joshua Miller <jsmiller@uchicago.edu>")
        .about("Ido-like bash completion")
        .arg(Arg::with_name("HEIGHT")
             .short("H")
             .long("height")
             .help("Height of prompt")
             .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
             .short("-o")
             .long("output")
             .help("Output file path")
             .takes_value(true))
        .get_matches();

    let height: u16 = matches.value_of("HEIGHT").unwrap_or("5").parse()
        .expect("Height must but an integer between 0 and 65535.");

    let mut popup = DropdownPrompt::new(Dropdown::new(height));
    let mut readkeys = Readkeys::new();

    let prompt = get_prompt();
    popup.prompt(&prompt, &readkeys.value, &["first", &*readkeys.value, "third", "fourth"]);

    loop {
        match *readkeys.recv() {
            ReadEvent::Exit | ReadEvent::Submit => break,
            _ => ()
        };
        popup.prompt(&prompt, &readkeys.value, &["first", &*readkeys.value, "third", "fourth"]);
        popup.dropdown.set_cursor((prompt.width() + readkeys.cursor) as u16);
    }

    popup.dropdown.teardown();

    if let Some(path) = matches.value_of("OUTPUT") {
        File::create(path).unwrap().write_all(readkeys.value.as_bytes()).unwrap();
    } else {
        stdout().write_all(readkeys.value.as_bytes()).unwrap();
    }
}
