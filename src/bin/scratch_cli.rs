use std::fs;
use std::str::FromStr;

use scratch::parser::ast::*;

fn main() {
    let file_name = "/Users/nimalanm/Documents/open-source/personal/scratch/examples/basic";
    let file_content = fs::read_to_string(file_name).expect("unable to read file");

    let empty_program = Program::empty();
    let ast = Program::from_str(&file_content);
}
