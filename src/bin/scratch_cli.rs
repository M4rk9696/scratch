use std::str::FromStr;
use std::fs;

use scratch::parser::ast::*;

fn main() {
  let file_name = "/Users/nimalanm/Documents/open-source/personal/scratch/examples/basic";
  let file_content = fs::read_to_string(file_name).expect("unable to read file");

  let ast = Program::from_str(&file_content);

  let serialized = serde_json::to_string(&ast).unwrap();
  println!("{:?}", serialized);
}
