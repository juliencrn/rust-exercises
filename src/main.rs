// mod hello;
// mod fibonacci;
mod degree_converter;

// use crate::hello::mod_hello;
// use crate::fibonacci::mod_fibonacci;
use degree_converter::mod_degree_converter;
fn main() {
    // mod_hello::demo();
    // mod_fibonacci::print_fibonacci();
    mod_degree_converter::demo()
}