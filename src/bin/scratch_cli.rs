use std::fs;
use std::str::FromStr;

use scratch::core::*;
use scratch::parser::ast::*;

fn main() {
    let file_name = "/path/to/file";
    let file_content = fs::read_to_string(file_name).expect("unable to read file");

    let empty_program = Program::empty();
    let ast = Program::from_str(&file_content);

    if ast.is_err() {
        println!("{:?}", ast);
    } else {
        println!("{}", execute(&ast.unwrap()));
    }
}
