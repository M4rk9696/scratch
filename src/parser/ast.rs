use std::convert::From;
use std::str::FromStr;
use std::fmt;

use super::*;

use pest::error::Error;
use pest::iterators::Pairs;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
  pub navs: Vec<NavigateBlock>,
}

impl Program {
  pub fn empty() -> Program {
    Program {
      navs: vec![],
    }
  }
}

impl FromStr for Program {
  type Err = Error<Rule>;

  fn from_str(input: &str) -> Result<Self, Self::Err> {

    let mut nav_blocks = vec![];
    let pairs = parse(&input);

    if pairs.is_ok() {
      for pair in pairs.unwrap() {
        match pair.as_rule() {
          Rule::navigate_block => {
            let mut pair = pair.into_inner();
            let url = pair.next().unwrap().as_str().to_string();
            let statements = parse_statements(pair.next().unwrap());
            nav_blocks.push(NavigateBlock {
              url: url,
              statements: statements,
            });
          }
          _ => {
            println!("some other block {:?}", pair);
          }
        }
      }
    }
    else {
      let err = pairs.err().unwrap();
      println!("Error at {:?}", err.line_col);
    }

    Ok(Program{
      navs: nav_blocks
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NavigateBlock {
  pub url: String,
  pub statements: Statements,
}

pub type Statements = Vec<Statement>;

fn parse_statements(pair: pest::iterators::Pair<Rule>) -> Statements {
  let mut statements = vec![];

  match pair.as_rule() {
    Rule::write_statement => {
      let mut pair = pair.into_inner();
      println!("write statement {:?} ", pair);
    },
    _ => {
      println!("other");
    }
  }

  statements
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
  AssignmentStatement {
    ident: String,
    value: Expression,
  },
  InStatement {
    is_parent: bool,
    element: Element,
    selector: Selector,
    statements: Statements,
  },
  WriteStatement {
    exps: Vec<Expression>,
  },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  FromExpression {
    element: Element,
    selector: Selector,
    content: String
  },
  Ident(String),
  Str(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
  Form, Input, Span, H1, H2, H3, H4,
  H5, Div, Table, Thead, Tbody, Tr, Th,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Selector {
  ClassSelector  {
    ident: String
  },
  IdSelector {
    ident: String
  },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Content {
  GetTextContent,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_selector_node() {

  }
}
