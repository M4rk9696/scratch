use pest::Parser;

use scratch::parser::grammar::ScratchParser;
use scratch::parser::grammar::Rule;

use std::fs;

fn main() {
  let file_name = "/Users/nimalanm/Documents/open-source/personal/crawler_dsl/examples/basic";
  let file_content = fs::read_to_string(file_name).expect("unable to read file");
  let parsed = ScratchParser::parse(Rule::program, &file_content)
    .expect("unable to parse")
    .next()
    .unwrap();
  println!("Parsed {}", parsed);
}
