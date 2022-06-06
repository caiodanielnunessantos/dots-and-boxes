#![feature(string_remove_matches)]

mod logic;
mod app;
mod colors;
mod options;


pub fn main() {
    options::get_options();
}
