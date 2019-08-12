use super::parser::ast;
use select::document::Document;
use std::iter;

pub fn execute(ast: &ast::Program) -> String {
    let mut navs = vec![];
    for nav in &ast.navs {
        navs.push(visit_nav_block(&nav));
    }
    navs.join("\n")
}

fn visit_nav_block(nav: &ast::NavigateBlock) -> String {
    let stmts = visit_statements(&nav.statements);
    format!(
        "let document = Document::from(include_str!(\"{url}\"));\n{statements}",
        url = nav.url,
        statements = stmts
    )
}

fn visit_statements(statements: &ast::Statements) -> String {
    let mut statementsNode = vec![];
    for statement in statements {
        statementsNode.push(visit_statement(&statement));
    }
    statementsNode.join("\n")
}

fn visit_statement(statement: &ast::Statement) -> String {
    match statement {
        ast::Statement::InStatement {
            query,
            sub_query,
            statements,
        } => {
            let nodes = generate_nodes("document".to_string(), query, sub_query);
            let statementNodes = visit_statements(statements);
            let in_statement = format!(
                "for node in {nodes} {{\n\t{statements}\n}}",
                nodes = nodes,
                statements = statementNodes
            );
            in_statement
        }
        ast::Statement::AssignmentStatement { ident, value } => {
            let valueNode = visit_expression(value);
            let assignment_statement =
                format!("let {ident} = {value};", ident = ident, value = valueNode);
            assignment_statement
        }
        ast::Statement::WriteStatement { expressions } => {
            let mut expr_list = vec![];
            for expr in expressions {
                expr_list.push(visit_expression(expr));
            }
            let placeholder: String = iter::repeat("{} ").take(expr_list.len()).collect();
            format!("println!(\"{}\", {});", placeholder, expr_list.join(","))
        }
    }
}

fn visit_expression(expr: &ast::Expression) -> String {
    match expr {
        ast::Expression::FromExpression {
            sub_query,
            query,
            content,
        } => {
            let query = generate_nodes("node".to_string(), query, sub_query);
            let content = match content {
                ast::Content::GetTextContent => match query.as_ref() {
                    "node" => String::from("text()"),
                    _ => String::from("next().unwrap().text()"),
                },
                ast::Content::Attr(ident) => format!("attr(\"{ident}\").unwrap()", ident = ident),
            };
            format!("{query}.{content}", query = query, content = content)
        }
        ast::Expression::Ident(ident) => format!("{}", ident),
        ast::Expression::Str(ident) => format!("\"{}\"", ident),
    }
}

fn generate_nodes(
    node: String,
    dom_query: &ast::DomQuery,
    dom_sub_query: &Option<ast::DomSubQuery>,
) -> String {
    let query = generate_dom_query(&dom_query);

    match dom_sub_query {
        Some(dom_sub_query) => match generate_dom_sub_query(&dom_sub_query) {
            MiscNode::Parent => format!(
                "{node}.find({query}).next().unwrap().parent()",
                node = node,
                query = query
            ),
            MiscNode::Child { node: selector } => format!(
                "{node}.find(Descendant({query}, {selector}))",
                query = query,
                node = node,
                selector = selector
            ),
        },
        None => match query.as_ref() {
            "Any" => format!("{node}", node = node),
            _ => format!("{node}.find({query})", node = node, query = query),
        },
    }
}

enum MiscNode {
    Parent,
    Child { node: String },
}

fn generate_dom_query(query: &ast::DomQuery) -> String {
    let elementNode = generate_predicate_from_elem(&query.element);
    match &query.selector {
        Some(selector) => {
            let selectorNode = generate_predicate_from_selector(&selector);
            match elementNode.as_ref() {
                "Any" => selectorNode,
                _ => format!("And({}, {})", elementNode, selectorNode),
            }
        }
        None => elementNode,
    }
}

fn generate_dom_sub_query(sub_query: &ast::DomSubQuery) -> MiscNode {
    if sub_query.is_parent {
        MiscNode::Parent
    } else {
        match &sub_query.query {
            Some(dom_query) => MiscNode::Child {
                node: generate_dom_query(&dom_query),
            },
            _ => unreachable!(),
        }
    }
}

fn generate_predicate_from_selector(selector: &ast::Selector) -> String {
    match (selector) {
        ast::Selector::ClassSelector { ident } => format!("Class(\"{}\")", ident),
        ast::Selector::IdSelector { ident } => format!("Attr(\"id\", \"{}\")", ident),
    }
}

fn generate_predicate_from_elem(element: &ast::Element) -> String {
    match element {
        ast::Element::DOMElement(elem) => format!("Name(\"{}\")", elem),
        ast::Element::Empty => format!("Any"),
    }
}
