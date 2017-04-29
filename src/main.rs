extern crate complesh;
extern crate termion;

use termion::{color, style};
use complesh::dropdown::{Dropdown, DropdownPrompt};
use complesh::readline::{Readline, ReadEvent, Printable};


fn main() {
    let mut popup = DropdownPrompt::new(Dropdown::new(5));
    let prompt = format!("{}{}enter text: {}{}",
                         style::Bold, color::Fg(color::Blue),
                         color::Fg(color::Reset), style::Reset);

    let mut readline = Readline::new();

    popup.prompt(&prompt, &readline.value, &["first", &*readline.value, "third", "fourth"]);

    loop {
        if let ReadEvent::Exit = *readline.recv() { break }
        popup.prompt(&prompt, &readline.value, &["first", &*readline.value, "third", "fourth"]);
        popup.dropdown.set_cursor((prompt.width() + readline.cursor) as u16);
    }
    popup.dropdown.reset();
}
