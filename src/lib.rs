#[macro_use] extern crate quick_error;
#[macro_use] extern crate lazy_static;
extern crate nix;
extern crate nlp_tokenize;
extern crate regex;
extern crate termion;
extern crate walkdir;
extern crate ignore;
extern crate crossbeam;
extern crate rayon;

pub mod completer;
pub mod filter;
pub mod dropdown;
pub mod errors;
pub mod readkeys;
pub mod util;
pub mod prompt;
pub mod ring_buffer;
