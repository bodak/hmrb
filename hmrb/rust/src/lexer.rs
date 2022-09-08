use pest::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "bab.pest"]
pub struct Grammar;

#[derive(Debug)]
pub enum Node {
    Children(Vec<Node>),
    Child(HashMap<String, Regex>),
}

#[derive(Debug)]
pub struct Core {
    root: Vec<Node>,
    variable: bool,
    name: String,
}

impl Core {
    fn new(file: &str) -> Core {
        fn parse_element(
            token: pest::iterators::Pair<Rule>,
        ) -> Node {
            match token.as_rule() {
                Rule::law | Rule::var | Rule::attributes => Node::Children(
                    token.into_inner().map(parse_element).collect(),
                ),
                Rule::attribute => {
                    let mut tokens = token.into_inner();
                    let name = tokens.next().unwrap().as_str();
                    let value = tokens.next().unwrap().as_str();
                    Node::Child(HashMap::from([(
                        String::from(name),
                        Regex::new(&format!("{}", value)).unwrap(),
                    )]))
                }
                Rule::grammar => unreachable!(),
                Rule::name => unreachable!(),
                Rule::value => unreachable!(),
                Rule::WHITESPACE => unreachable!(),
            }
        }
        let grammar_tree = Grammar::parse(Rule::grammar, &file)
            .unwrap()
            .next()
            .unwrap();
        let root: Vec<Node> =
            grammar_tree.into_inner().map(parse_element).collect();
        Core {
            root: root,
            variable: false,
            name: String::from("default"),
        }
    }
}

#[test]
fn test_lexer_1() {
    let test_file =
        fs::read_to_string("tests/lexer.1").expect("cannot read file");
    let core = Core::new(&test_file);
    assert_eq!(
        format!("{:?}", core),
        "Core { root: [Children([Children([Child({\"lemma\": Hello}), Child({\"lemma\": World})])])], variable: false, name: \"default\" }"
    );
}
