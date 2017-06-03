extern crate clap;
extern crate complesh;
extern crate termion;
extern crate nix;
extern crate glob;

use clap::{Arg, App};
use complesh::dropdown::Dropdown;
use complesh::completer::{Completer, MixedCompleter, ListCompleter};
use complesh::prompt::DropdownPrompt;
use complesh::readkeys::Readkeys;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;
use termion::color::{self, Blue, Fg};

fn run<C>(mut prompt: DropdownPrompt<C>, output_path: Option<&str>)
    where C: Completer
{
    let completion = match prompt.prompt() {
        Some(completion) => completion,
        None => return,
    };

    if let Some(path) = output_path {
        File::create(path).unwrap().write_all(completion.as_bytes()).unwrap();
    } else {
        stdout().write_all(completion.as_bytes()).unwrap();
    }
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
        .arg(Arg::with_name("INPUT")
             .short("-i")
             .long("input")
             .help("Input starting point for completion")
             .takes_value(true))
        .arg(Arg::with_name("CHOICES")
             .short("-c")
             .long("choices")
             .help("Whitespace delimited list of choices")
             .takes_value(true))
        .get_matches();

    let height      = matches.value_of("HEIGHT").unwrap_or("32").parse()
        .expect("Height must but an integer between 0 and 65535.");

    let beginning   = matches.value_of("INPUT").unwrap_or("").to_string();
    let output_path = matches.value_of("OUTPUT");
    let output      = Dropdown::new(height);
    let input       = Readkeys::new(beginning.clone());
    let prompt_str  = format!("{}complesh: {}", Fg(Blue), Fg(color::Reset));

    if let Some(choice_string) = matches.value_of("CHOICES") {
        let choices = choice_string.split_whitespace().map(str::to_string).collect();
        let completer = Box::new(ListCompleter::new(choices));
        run(DropdownPrompt::new(prompt_str, input, output, completer), output_path)
    } else {
        let completer = Box::new(MixedCompleter::new());
        run(DropdownPrompt::new(prompt_str, input, output, completer), output_path)
    };

}
