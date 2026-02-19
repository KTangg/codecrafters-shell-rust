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

        enum Quote {
            None,
            Single,
            Double,
        }

        let mut quote = Quote::None;

        let mut chars = self.buffer.chars().peekable();

        while let Some(ch) = chars.next() {
            match (ch, &quote) {
                (' ' | '\t', Quote::None) => {
                    if !current.is_empty() {
                        tokens.push(Token::Literal(std::mem::take(&mut current)));
                    }
                }

                ('\'', Quote::None) => quote = Quote::Single,
                ('\'', Quote::Single) => quote = Quote::None,

                ('\"', Quote::None) => quote = Quote::Double,
                ('\"', Quote::Double) => quote = Quote::None,

                ('|', Quote::None) => tokens.push(Token::Pipe),

                ('\\', Quote::None) => {
                    let Some(c) = chars.peek() else { todo!() };

                    current.push(*c);
                    chars.next();
                }
                ('\\', Quote::Double) => {
                    let Some(c) = chars.peek() else { todo!() };

                    match c {
                        '"' | '\\' | '$' | '`' | '\n' => {
                            current.push(*c);
                            chars.next();
                        }
                        _ => {
                            current.push(ch);
                        }
                    }
                }

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

        lex.push("'hello''world'");
        let expect = vec![Token::Literal("helloworld".to_string())];
        assert_eq!(expect, lex.tokenize());
    }

    #[test]
    fn test_double_quote() {
        let mut lex = Lexer::new();

        lex.push("\"hello     world\"");
        let expect = vec![Token::Literal("hello     world".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("\"shell's test\"");
        let expect = vec![Token::Literal("shell's test".to_string())];
        assert_eq!(expect, lex.tokenize());
    }

    #[test]
    fn test_escape() {
        let mut lex = Lexer::new();

        lex.push("multiple\\ \\ \\ \\ spaces");
        let expect = vec![Token::Literal("multiple    spaces".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("\\'\\\"literal quotes\\\"\\'");
        let expect = vec![
            Token::Literal("\'\"literal".to_string()),
            Token::Literal("quotes\"\'".to_string()),
        ];
        assert_eq!(expect, lex.tokenize());

        lex.push("ignore\\_backslash");
        let expect = vec![Token::Literal("ignore_backslash".to_string())];
        assert_eq!(expect, lex.tokenize());
    }

    #[test]
    fn test_escape_in_quote() {
        let mut lex = Lexer::new();

        lex.push("'shell\\\\\\nscript'");
        let expect = vec![Token::Literal("shell\\\\\\nscript".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("'example\\\"test'");
        let expect = vec![Token::Literal("example\\\"test".to_string())];
        assert_eq!(expect, lex.tokenize());
    }

    #[test]
    fn test_escape_in_double_quote() {
        //Within double quotes, a backslash only escapes certain special characters: ", \, $, `, and newline.
        //For all other characters, the backslash is treated literally.
        let mut lex = Lexer::new();

        lex.push("\"\\hello \\world\"");
        let expect = vec![Token::Literal("\\hello \\world".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("\"A \\ escapes itself\"");
        let expect = vec![Token::Literal("A \\ escapes itself".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("\"A \\\" inside double quotes\"");
        let expect = vec![Token::Literal("A \" inside double quotes".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("\"\\$ is a Dollar sign\" ");
        let expect = vec![Token::Literal("$ is a Dollar sign".to_string())];
        assert_eq!(expect, lex.tokenize());
    }
}
