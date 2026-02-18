#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(String),
    Complex(String),
    Pipe,
}

pub struct Lexer {
    buffer: String,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            buffer: String::new(),
        }
    }

    pub fn push(&mut self, input: &str) {
        self.buffer.push_str(input);
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        let mut in_single = false;
        let mut in_double = false;

        for ch in self.buffer.chars() {
            match ch {
                ' ' | '\t' if !in_single && !in_double => {
                    if !current.is_empty() {
                        tokens.push(Token::Literal(current.clone()));
                        current.clear();
                    }
                }
                '\'' if !in_double => in_single = !in_single,
                '\"' if !in_single => in_double = !in_double,
                _ => {
                    current.push(ch);
                }
            }
        }
        if !current.is_empty() {
            tokens.push(Token::Literal(current));
        }

        self.buffer.clear();

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_space() {
        let mut lex = Lexer::new();
        lex.push("hello world");

        let expect = vec![
            Token::Literal("hello".to_string()),
            Token::Literal("world".to_string()),
        ];

        assert_eq!(expect, lex.tokenize());
    }

    #[test]
    fn test_multiple_spaces() {
        let mut lex = Lexer::new();
        lex.push("  hello     world    ");

        let expect = vec![
            Token::Literal("hello".to_string()),
            Token::Literal("world".to_string()),
        ];

        assert_eq!(expect, lex.tokenize());
    }

    #[test]
    fn test_single_quote() {
        let mut lex = Lexer::new();
        lex.push("'hello' world");

        let expect = vec![
            Token::Literal("hello".to_string()),
            Token::Literal("world".to_string()),
        ];

        assert_eq!(expect, lex.tokenize());
    }

    #[test]
    fn test_consecutive_single_quote() {
        let mut lex = Lexer::new();
        lex.push("'hello''world'");

        let expect = vec![Token::Literal("helloworld".to_string())];

        assert_eq!(expect, lex.tokenize());
    }
}
