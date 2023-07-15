use super::ast::Token;

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = code.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => continue,
            '=' => tokens.push(Token::Equal),
            '!' => {
                if let Some(&'=') = iter.peek() {
                    iter.next();
                    tokens.push(Token::NotEqual);
                } else {
                    tokens.push(Token::Unknown);
                }
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::Slash),
            '%' => tokens.push(Token::Modulo),
            '(' => tokens.push(Token::LeftParenthesis),
            ')' => tokens.push(Token::RightParenthesis),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            '<' => tokens.push(Token::LessThan),
            '>' => tokens.push(Token::GreaterThan),
            ';' => tokens.push(Token::SemiColon),
            ',' => tokens.push(Token::Comma),
            '"' => {
                let mut string_literal = String::new();
                while let Some(ch) = iter.next() {
                    if ch == '"' {
                        break;
                    } else {
                        string_literal.push(ch);
                    }
                }
                tokens.push(Token::StringLiteral(string_literal));
            }
            c if c.is_alphabetic() => {
                let mut identifier = String::new();
                identifier.push(c);

                while let Some(&next_ch) = iter.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        identifier.push(next_ch);
                        iter.next();
                    } else {
                        break;
                    }
                }

                match identifier.as_str() {
                    "let" => tokens.push(Token::Let),
                    "print" => tokens.push(Token::Print),
                    "fn" => tokens.push(Token::Fn),
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "elif" => tokens.push(Token::IfElse),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }
            c if c.is_digit(10) => {
                let mut number = String::new();
                number.push(c);

                while let Some(&next_ch) = iter.peek() {
                    if next_ch.is_digit(10) {
                        number.push(next_ch);
                        iter.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(number.parse().unwrap()));
            }
            _ => tokens.push(Token::Unknown),
        }
    }

    tokens.push(Token::EndOfFile);
    tokens
}