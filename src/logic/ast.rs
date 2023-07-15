use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Fn,
    If,
    Else,
    IfElse,
    Equal,
    NotEqual,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    Identifier(String),
    Number(i32),
    StringLiteral(String),
    Print,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LessThan,
    GreaterThan,
    Comma,
    SemiColon,
    EndOfFile,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Box<Vec<ASTNode>>),
    Assignment(String, Box<ASTNode>),
    Variable(String, Box<ASTNode>),
    Print(Box<ASTNode>),
    Identifier(String),
    Number(i32),
    StringLiteral(String),
    BinaryOperation(Box<ASTNode>, Token, Box<ASTNode>),
    If(Box<ASTNode>, Vec<Box<ASTNode>>),
    IfElse(Box<ASTNode>, Vec<Box<ASTNode>>, Vec<Box<ASTNode>>),
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();

        while self.current_token() != Token::EndOfFile {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(ASTNode::Program(Box::new(statements)))
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        match self.current_token() {
            Token::Let => self.parse_variable_declaration(),
            Token::Print => self.parse_print_statement(),
            Token::Identifier(_) => self.parse_assignment(),
            Token::If => self.parse_if_statement(),
            _ => Err(format!(
                "Unexpected token {:?} at {}",
                self.current_token(),
                self.current
            )),
            _ => self.parse_assignment(),
        }
    }

    fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
        self.expect_token(Token::If)?;
        let condition = self.parse_expression()?;
        self.expect_token(Token::LeftBrace)?;
        let mut statements = Vec::new();
        while self.current_token() != Token::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(Box::new(statement));
        }
        self.expect_token(Token::RightBrace)?;

        if self.current_token() == Token::Else {
            self.next_token()?;
            self.expect_token(Token::LeftBrace)?;
            let mut else_statements = Vec::new();
            while self.current_token() != Token::RightBrace {
                let statement = self.parse_statement()?;
                else_statements.push(Box::new(statement));
            }
            self.expect_token(Token::RightBrace)?;

            Ok(ASTNode::IfElse(
                Box::new(condition),
                statements,
                else_statements,
            ))
        } else {
            Ok(ASTNode::If(Box::new(condition), statements))
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        self.expect_token(Token::Let)?;
        let identifier = self.expect_identifier()?;
        self.expect_token(Token::Equal)?;
        let value = self.parse_expression()?;
        self.expect_token(Token::SemiColon)?;

        Ok(ASTNode::Variable(identifier, Box::new(value)))
    }

    fn parse_print_statement(&mut self) -> Result<ASTNode, String> {
        self.expect_token(Token::Print)?;
        let expression = self.parse_expression()?;
        self.expect_token(Token::SemiColon)?;

        Ok(ASTNode::Print(Box::new(expression)))
    }

    fn parse_assignment(&mut self) -> Result<ASTNode, String> {
        let identifier = self.expect_identifier()?;
        self.expect_token(Token::Equal)?;
        let expression = self.parse_expression()?;
        self.expect_token(Token::SemiColon)?;

        Ok(ASTNode::Assignment(identifier, Box::new(expression)))
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut left_node = self.parse_term()?;

        while self.current_token() == Token::Plus
            || self.current_token() == Token::Minus
            || self.current_token() == Token::Equal
            || self.current_token() == Token::NotEqual
            || self.current_token() == Token::Asterisk
            || self.current_token() == Token::Slash
            || self.current_token() == Token::Modulo
            || self.current_token() == Token::LessThan
            || self.current_token() == Token::GreaterThan
        {
            let operator = self.current_token();
            self.next_token()?;

            let right_node = self.parse_term()?;
            left_node = ASTNode::BinaryOperation(
                Box::new(left_node),
                operator,
                Box::new(right_node),
            );
        }

        Ok(left_node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut left_node = self.parse_factor()?;

        while self.current_token() == Token::Equal
            || self.current_token() == Token::NotEqual
        {
            let operator = self.current_token();
            self.next_token()?;

            let right_node = self.parse_factor()?;
            left_node = ASTNode::BinaryOperation(
                Box::new(left_node),
                operator,
                Box::new(right_node),
            );
        }

        Ok(left_node)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        match self.current_token() {
            Token::Number(value) => {
                self.next_token()?;
                Ok(ASTNode::Number(value))
            }
            Token::StringLiteral(value) => {
                self.next_token()?;
                Ok(ASTNode::StringLiteral(value))
            }
            Token::Identifier(value) => {
                self.next_token()?;
                Ok(ASTNode::Identifier(value))
            }
            Token::LeftParenthesis => {
                self.next_token()?;
                let expression = self.parse_expression()?;
                self.expect_token(Token::RightParenthesis)?;
                Ok(expression)
            }
            _ => Err(format!(
                "Unexpected token {:?} at {}",
                self.current_token(),
                self.current
            )),
        }
    }

    fn expect_token(&mut self, token: Token) -> Result<(), String> {
        if self.current_token() == token {
            self.next_token()?;
            Ok(())
        } else {
            Err(format!(
                "Expected token {:?} but found {:?} at {}",
                token,
                self.current_token(),
                self.current
            ))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        match self.current_token() {
            Token::Identifier(identifier) => {
                self.next_token()?;
                Ok(identifier)
            }
            _ => Err(format!(
                "Expected identifier but found {:?} at {}",
                self.current_token(),
                self.current
            )),
        }
    }

    fn current_token(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn next_token(&mut self) -> Result<(), String> {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
            Ok(())
        } else {
            Err("Unexpected end of file".to_string())
        }
    }
}

impl Eq for ASTNode {}

impl PartialOrd for ASTNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ASTNode::Number(a), ASTNode::Number(b)) => a.partial_cmp(b),
            (ASTNode::StringLiteral(a), ASTNode::StringLiteral(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Ord for ASTNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ASTNode::Number(a), ASTNode::Number(b)) => a == b,
            (ASTNode::StringLiteral(a), ASTNode::StringLiteral(b)) => a == b,
            (ASTNode::Identifier(a), ASTNode::Identifier(b)) => a == b,
            _ => false,
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<ASTNode, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
