use super::ast::{ASTNode, Token};

struct Interpreter {
    variables: std::collections::HashMap<String, ASTNode>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            variables: std::collections::HashMap::new(),
        }
    }

    fn interpret(&mut self, ast: ASTNode) {
        match ast {
            ASTNode::Program(statements) => {
                for statement in statements.iter() {
                    self.interpret(statement.clone());
                }
            }
            ASTNode::Variable(identifier, value) => {
                let evaluated_value = self.evaluate_expression(*value);
                self.variables.insert(identifier, evaluated_value);
            }
            ASTNode::Assignment(identifier, value) => {
                let evaluated_value = self.evaluate_expression(*value);
                self.variables.insert(identifier, evaluated_value);
            }
            ASTNode::Print(expression) => {
                let evaluated_expression = self.evaluate_expression(*expression);
                println!("{}", self.stringify_value(evaluated_expression));
            }
            _ => panic!("Unexpected AST node: {:?}", ast),
        }
    }

    fn evaluate_expression(&mut self, ast: ASTNode) -> ASTNode {
        match ast {
            ASTNode::BinaryOperation(left, operator, right) => {
                let left_value = self.evaluate_expression(*left);
                let right_value = self.evaluate_expression(*right);
                self.evaluate_binary_operation(left_value, operator, right_value)
            }
            ASTNode::Identifier(identifier) => {
                self.variables
                    .get(&identifier)
                    .expect(&format!("Undefined variable: {}", identifier))
                    .clone()
            }
            ASTNode::Number(value) => ASTNode::Number(value),
            ASTNode::StringLiteral(value) => ASTNode::StringLiteral(value),
            _ => panic!("Unexpected AST node: {:?}", ast),
        }
    }

    fn evaluate_binary_operation(&mut self, left: ASTNode, operator: Token, right: ASTNode) -> ASTNode {
        match operator {
            Token::Plus => self.evaluate_addition(left, right),
            Token::Minus => self.evaluate_subtraction(left, right),
            Token::Asterisk => self.evaluate_multiplication(left, right),
            Token::Slash => self.evaluate_division(left, right),
            Token::Modulo => self.evaluate_modulo(left, right),
            Token::Equal => self.evaluate_equal(left, right),
            Token::NotEqual => self.evaluate_not_equal(left, right),
            _ => panic!("Unexpected operator: {:?}", operator),
        }
    }

    fn evaluate_addition(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left_value), ASTNode::Number(right_value)) => {
                ASTNode::Number(left_value + right_value)
            }
            _ => panic!("Cannot add {:?} and {:?}", left, right),
        }
    }

    fn evaluate_subtraction(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left_value), ASTNode::Number(right_value)) => {
                ASTNode::Number(left_value - right_value)
            }
            _ => panic!("Cannot subtract {:?} and {:?}", left, right),
        }
    }

    fn evaluate_multiplication(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left_value), ASTNode::Number(right_value)) => {
                ASTNode::Number(left_value * right_value)
            }
            _ => panic!("Cannot multiply {:?} and {:?}", left, right),
        }
    }

    fn evaluate_division(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left_value), ASTNode::Number(right_value)) => {
                ASTNode::Number(left_value / right_value)
            }
            _ => panic!("Cannot divide {:?} and {:?}", left, right),
        }
    }

    fn evaluate_modulo(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left_value), ASTNode::Number(right_value)) => {
                ASTNode::Number(left_value % right_value)
            }
            _ => panic!("Cannot modulo {:?} and {:?}", left, right),
        }
    }

    fn evaluate_equal(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left_value), ASTNode::Number(right_value)) => {
                if left_value == right_value {
                    ASTNode::Number(1)
                } else {
                    ASTNode::Number(0)
                }
            }
            _ => panic!("Cannot compare {:?} and {:?}", left, right),
        }
    }

    fn evaluate_not_equal(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left_value), ASTNode::Number(right_value)) => {
                if left_value != right_value {
                    ASTNode::Number(1)
                } else {
                    ASTNode::Number(0)
                }
            }
            _ => panic!("Cannot compare {:?} and {:?}", left, right),
        }
    }

    fn stringify_value(&mut self, ast: ASTNode) -> String {
        match ast {
            ASTNode::Number(value) => value.to_string(),
            ASTNode::StringLiteral(value) => value,
            _ => panic!("Unexpected AST node: {:?}", ast),
        }
    }
}

pub fn interpret(ast: ASTNode) {
    let mut interpreter = Interpreter::new();
    interpreter.interpret(ast);
}