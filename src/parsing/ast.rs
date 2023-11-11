use crate::lexing::token::{Operator, Token};
use crate::parsing::ast::Ast::{Nil, Node};
use crate::parsing::ast::Parameters::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Parameters {
    Int(i64),
    Float(f64),
    Identifier(String),
    PlusOperation,
    MinusOperation,
    MultiplicationOperation,
    DivideOperation,
    Assign,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Nil,
    Node {
        value: Parameters,
        left: Box<Ast>,
        right: Box<Ast>,
    },
}

impl Ast {
    pub fn new(p: Parameters) -> Self {
        Node {
            value: p,
            left: Box::from(Nil),
            right: Box::from(Nil),
        }
    }
    pub fn insert_left(self, node: Ast) -> Self {
        match &self {
            Nil => node,
            Node {
                value,
                left: _left,
                right,
            } => Node {
                value: value.clone(),
                left: Box::from(node),
                right: right.clone(),
            },
        }
    }
    pub fn insert_right(self, node: Ast) -> Self {
        match &self {
            Nil => node,
            Node {
                value,
                left,
                right: _right,
            } => Node {
                value: value.clone(),
                left: left.clone(),
                right: Box::from(node),
            },
        }
    }
    pub fn value(self) -> Parameters {
        match &self {
            Nil => Null,
            Node {
                value,
                left: _left,
                right: _right,
            } => {
                return value.clone();
            }
        }
    }
    pub fn left(self) -> Ast {
        match &self {
            Nil => Nil,
            Node {
                value: _value,
                left: l,
                right: _right,
            } => {
                return *(*(l)).clone();
            }
        }
    }
}

pub fn token_to_parameter(token: Token) -> Parameters {
    match token {
        Token::INT(i) => Int(i),
        Token::FLOAT(f) => Float(f),
        Token::IDENTIFIER(s) => Identifier(s),
        Token::OPE(Operator::PLUS) => PlusOperation,
        Token::OPE(Operator::MINUS) => MinusOperation,
        Token::OPE(Operator::MULTIPLICATION) => MultiplicationOperation,
        Token::OPE(Operator::DIVIDE) => DivideOperation,
        Token::EQUAL => Assign,
        _ => Null,
    }
}

#[cfg(test)]
mod test {
    use crate::parsing::ast::{Ast, Parameters};

    #[test]
    pub fn test_new() {
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::Nil),
        };
        let result = Ast::new(Parameters::Int(2));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_insert_left() {
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::new(Parameters::Int(2))),
            right: Box::from(Ast::Nil),
        };
        let result = Ast::new(Parameters::Int(2)).insert_left(Ast::new(Parameters::Int(2)));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_insert_right() {
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::Nil),
            right: Box::from(Ast::new(Parameters::Int(2))),
        };
        let result = Ast::new(Parameters::Int(2)).insert_right(Ast::new(Parameters::Int(2)));
        assert_eq!(result, expected)
    }

    #[test]
    pub fn test_insert_both() {
        let expected = Ast::Node {
            value: Parameters::Int(2),
            left: Box::from(Ast::new(Parameters::Int(2))),
            right: Box::from(Ast::new(Parameters::Int(2))),
        };
        let result = Ast::new(Parameters::Int(2))
            .insert_right(Ast::new(Parameters::Int(2)))
            .insert_left(Ast::new(Parameters::Int(2)));
        assert_eq!(result, expected);
    }
}
