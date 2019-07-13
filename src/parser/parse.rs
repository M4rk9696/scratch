use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar/scratch.pest"]
struct CrawlerParser;

use pest::iterators::Pairs;

pub fn parse(source: &str) -> Pairs<Rule> {
  CrawlerParser::parse(Rule::program, source).expect("unable to parse")
}

#[cfg(test)]
mod tests {
  use pest::*;
  use super::*;

  #[test]
  fn string_literal() {
    assert!(CrawlerParser::parse(Rule::string, "'Hello World'").is_ok());
    assert!(CrawlerParser::parse(Rule::string, "'Hell''o World'").is_ok());
    assert!(CrawlerParser::parse(Rule::string, "'Hell\'o World'").is_ok());
  }

  #[test]
  fn parse_ident() {
    assert!(CrawlerParser::parse(Rule::ident, "variable").is_ok());
    assert!(CrawlerParser::parse(Rule::ident, "vari21able").is_ok());
    assert!(CrawlerParser::parse(Rule::ident, "v_ari21_able").is_ok());
    assert!(CrawlerParser::parse(Rule::ident, "1v_ari21_able").is_err());
  }

  #[test]
  fn parse_from_expression() {
    assert!(CrawlerParser::parse(
      Rule::from_expression,
      "from    (h1 having id('title')) getTextContent"
    ).is_ok());

    assert!(CrawlerParser::parse(
      Rule::from_expression,
      "from
      (h1 
      having 
      class('title') 
      )
      getTextContent"
    ).is_ok());
  }

  #[test]
  fn parse_write_statement() {
    assert!(CrawlerParser::parse(Rule::write_statement, "write(id)").is_ok());
    assert!(CrawlerParser::parse(Rule::write_statement, "write(title, c, a, b)").is_ok());
    assert!(CrawlerParser::parse(Rule::write_statement, "write('ab', 'sd', a)").is_ok());
  }

  #[test]
  fn parse_assignment_statement() {
    assert!(CrawlerParser::parse(
      Rule::assignment_statement,
      "x = from (h1 having id('title')) getTextContent",
      ).is_ok());
  }

  #[test]
  fn parse_in_statement() {
    assert!(CrawlerParser::parse(
      Rule::in_statement,
      "in (div having id('title')) {
        write(x);
      }"
    ).is_ok());

    assert!(CrawlerParser::parse(
      Rule::in_statement,
      "in (div having class('title')) {
        x = from(h1 having id('abc')) getTextContent;
        write(x);
      }"
    ).is_ok());

    assert!(CrawlerParser::parse(
      Rule::in_statement,
      "in (table having id('title')) {
        in (span having class('question')) {
          write(x, y);
        };
        write(x);
      }"
    ).is_ok());
  }

  #[test]
  fn parse_statements() {
    assert!(CrawlerParser::parse(
      Rule::statements,
      "write(x, y);
      "
    ).is_ok());

    assert!(CrawlerParser::parse(
      Rule::statements,
      "question = from(tr having class('question')) getTextContent;
      answer = from(tr having class('question')) getTextContent;
      write(question, answer);
      "
    ).is_ok());
  }

  #[test]
  fn parse_navigate_block() { // TODO Add more tests here
    assert!(CrawlerParser::parse(
      Rule::navigate_block,
      "navigateTo('www.example.com') {
        write(question, answer);
      }"
    ).is_ok());
  }
}