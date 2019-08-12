use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar/Scratch.pest"]
struct ScratchParser;

pub fn parse(source: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    ScratchParser::parse(Rule::program, &source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::*;

    #[test]
    fn string_literal() {
        assert!(ScratchParser::parse(Rule::string, "'Hello World'").is_ok());
        assert!(ScratchParser::parse(Rule::string, "'Hell''o World'").is_ok());
        assert!(ScratchParser::parse(Rule::string, "'Hell\'o World'").is_ok());
    }

    #[test]
    fn parse_ident() {
        assert!(ScratchParser::parse(Rule::ident, "variable").is_ok());
        assert!(ScratchParser::parse(Rule::ident, "vari21able").is_ok());
        assert!(ScratchParser::parse(Rule::ident, "v_ari21_able").is_ok());
        assert!(ScratchParser::parse(Rule::ident, "1v_ari21_able").is_err());
    }

    #[test]
    fn parse_ident_expression() {
        assert!(ScratchParser::parse(Rule::expression, "x").is_ok());
    }

    #[test]
    fn parse_string_expression() {
        assert!(ScratchParser::parse(Rule::expression, "'abd'").is_ok());
    }

    #[test]
    fn parse_from_expression() {
        assert!(ScratchParser::parse(
            Rule::expression,
            "from    (h1 having id('title')) getTextContent"
        )
        .is_ok());

        assert!(ScratchParser::parse(Rule::expression, "from (h1) getTextContent").is_ok());

        assert!(ScratchParser::parse(
            Rule::expression,
            "from
            (h1
            having
            class('title')
            )
            getTextContent"
        )
        .is_ok());
    }

    #[test]
    fn parse_write_statement() {
        assert!(ScratchParser::parse(Rule::write_statement, "write(id)").is_ok());
        assert!(ScratchParser::parse(Rule::write_statement, "write(title, c, a, b)").is_ok());
        assert!(ScratchParser::parse(Rule::write_statement, "write('ab', 'sd', a)").is_ok());
        assert!(ScratchParser::parse(
            Rule::write_statement,
            "write(from (h1 having id('a')) getTextContent)"
        )
        .is_ok())
    }

    #[test]
    fn parse_assignment_statement() {
        assert!(ScratchParser::parse(
            Rule::assignment_statement,
            "x = from (h1 having id('title')) getTextContent",
        )
        .is_ok());
    }

    #[test]
    fn parse_in_statement() {
        assert!(ScratchParser::parse(
            Rule::in_statement,
            "in (div having id('title')) {
                write(x);
            }"
        )
        .is_ok());

        assert!(ScratchParser::parse(
            Rule::in_statement,
            "in (div having class('title')) {
                x = from(h1 having id('abc')) getTextContent;
                write(x);
            }"
        )
        .is_ok());

        assert!(ScratchParser::parse(
            Rule::in_statement,
            "in(table having id('a')) {
            }"
        )
        .is_ok());

        assert!(ScratchParser::parse(
            Rule::in_statement,
            "in (table having id('title')) {
                in (span having class('question')) {
                    write(x, y);
                };
                write(x);
            }"
        )
        .is_ok());

        assert!(ScratchParser::parse(
            Rule::in_statement,
            "in parent of (h1 having id('title')) {
                write(x, y);
            }"
        )
        .is_ok());

        assert!(ScratchParser::parse(
            Rule::in_statement,
            "in child(div having id('parent')) of (tr having class('car')) {
                write(x);
            }"
        )
        .is_ok());

        assert!(ScratchParser::parse(
            Rule::in_statement,
            "in child(div) of (tr) {
                write(x);
            }"
        )
        .is_ok());

        assert!(ScratchParser::parse(
            Rule::in_statement,
            "in(_ having id('title')) {
                write(x, y);
            }"
        )
        .is_ok());
    }

    #[test]
    fn parse_statements() {
        assert!(ScratchParser::parse(Rule::statements, "write(x, y);").is_ok());

        assert!(ScratchParser::parse(
            Rule::statements,
            "question = from(tr having class('question')) getTextContent;
            answer = from(tr having class('question')) getTextContent;
            write(question, answer);
            "
        )
        .is_ok());
    }

    #[test]
    fn parse_navigate_block() {
        // TODO Add more tests here
        assert!(ScratchParser::parse(
            Rule::navigate_block,
            "navigateTo('www.example.com') {
                write(question, answer);
            }"
        )
        .is_ok());

        assert!(ScratchParser::parse(
            Rule::navigate_block,
            "navigateTo('http://www.example.com') {
                in(table having id('a')) {
                };
            }"
        )
        .is_ok());
    }
}
