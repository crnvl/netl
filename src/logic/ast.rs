#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Equal,
    NotEqual,
    Plus,
    Minus,
    Identifier(String),
    Number(i32),
    StringLiteral(String),
    Print,
    LeftParenthesis,
    RightParenthesis,
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
            _ => self.parse_assignment(),
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
            || self.current_token() == Token::NotEqual
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

pub fn parse(tokens: Vec<Token>) -> Result<ASTNode, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
