use std::str::FromStr;
use super::*;

use pest::error::Error;

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
          },
          Rule::EOI => {},
          _ => panic!("unknown rule {:}", pair),
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
  match pair.as_rule() {
    Rule::statements => {
      let stmt_rules = pair.into_inner();
      let mut statements = vec![];
      for stmt in stmt_rules {
        statements.push(parse_statement(stmt));
      }
      statements
    },
    _ => panic!("unknown rule {:?}", pair),
  }
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
    expressions: Vec<Expression>,
  },
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Statement {
  match pair.as_rule() {
    Rule::in_statement => {
      let mut pair = pair.into_inner();
      let element = Element::from(pair.next().unwrap().as_str());
      let selector = parse_selector(pair.next().unwrap());
      let stmts = pair.next().unwrap();
      let statements = parse_statements(stmts);
      Statement::InStatement {
        is_parent: false,
        element: element,
        selector: selector,
        statements: statements,
      }
    }
    Rule::assignment_statement => {
      let mut pair = pair.into_inner();
      let ident = pair.next().unwrap().as_str().to_string();
      let expression = parse_expression(pair.next().unwrap());
      Statement::AssignmentStatement {
        ident: ident,
        value: expression,
      }
    }
    Rule::write_statement => {
      let mut expressions = vec![];
      for exp in pair.into_inner() {
        expressions.push(parse_expression(exp));
      }
      Statement::WriteStatement {
        expressions: expressions,
      }
    },
    _ => panic!("other {:?}", pair),
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  FromExpression {
    element: Element,
    selector: Selector,
    content: Content,
  },
  Ident(String),
  Str(String),
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Expression {
  match pair.as_rule() {
    Rule::from_expression => {
      let mut pair = pair.into_inner();
      let element = Element::from(pair.next().unwrap().as_str());
      let selector = parse_selector(pair.next().unwrap());
      let content = Content::from(pair.next().unwrap().as_str());
      Expression::FromExpression {
        element: element,
        selector: selector,
        content: content,
      }
    },
    Rule::ident => Expression::Ident(pair.as_str().to_string()),
    Rule::string => Expression::Str(pair.as_str().to_string()),
    _ => panic!("Unknown expression {:?}", pair),
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
  Form, Input, Span, H1, H2, H3, H4,
  H5, Div, Table, Thead, Tbody, Tr, Th,
}

impl Element {
  fn from(elem: &str) -> Self {
    match elem {
      "form" => Element::Form,
      "input" => Element::Input,
      "span" => Element::Span,
      "h1" => Element::H1,
      "h2" => Element::H2,
      "h3" => Element::H3,
      "h4" => Element::H4,
      "h5" => Element::H5,
      "div" => Element::Div,
      "table" => Element::Table,
      "thead" => Element::Thead,
      "tbody" => Element::Tbody,
      "tr" => Element::Tr,
      "th" => Element::Th,
      _ => panic!("Unrecognised element"),
    }
  }
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

impl Selector {
  fn from(selector: &str, ident: &str) -> Self {
    match selector {
      "class" => Selector::ClassSelector {
        ident: ident.to_string(),
      },
      "id" => Selector::IdSelector {
        ident: ident.to_string(),
      },
      _ => panic!("Unrecognised selector {}", selector),
    }
  }
}

fn parse_selector(pair: pest::iterators::Pair<Rule>) -> Selector {
  let mut pair = pair.into_inner();
  let selector = pair.next().unwrap().as_str();
  let ident = pair.next().unwrap().as_str();
  Selector::from(selector, ident)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Content {
  GetTextContent,
}

impl Content {
  fn from(id: &str) -> Self {
    match id {
      "getTextContent" => Content::GetTextContent,
      _ => panic!("Unknown content type {}", id),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_selector_node() {

  }
}
