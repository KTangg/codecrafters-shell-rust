#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(String),
    Complex(String),
    Pipe,
    Redirect(usize),
    Append(usize),
}

pub enum Quote {
    None,
    Single,
    Double,
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
        let mut tk = String::new();
        let mut tokens = Vec::new();
        let mut quote = Quote::None;
        let mut chars = self.buffer.chars().peekable();

        while let Some(ch) = chars.next() {
            match (ch, &quote) {
                ('\'', Quote::None) => quote = Quote::Single,
                ('\'', Quote::Single) => quote = Quote::None,

                ('\"', Quote::None) => quote = Quote::Double,
                ('\"', Quote::Double) => quote = Quote::None,

                ('|', Quote::None) => tokens.push(Token::Pipe),

                ('\\', Quote::None) => {
                    let Some(c) = chars.peek() else { todo!() };

                    tk.push(*c);
                    chars.next();
                }
                ('\\', Quote::Double) => {
                    let Some(next_char) = chars.peek() else {
                        todo!()
                    };

                    match next_char {
                        '"' | '\\' | '$' | '`' | '\n' => {
                            tk.push(*next_char);
                            chars.next();
                        }
                        _ => {
                            tk.push(ch);
                        }
                    }
                }

                // TODO: Handle & for all fd redirect/append
                ('>', Quote::None) => {
                    let Some(next_char) = chars.peek() else {
                        todo!()
                    };
                    let fd = Self::take_io_number(&mut tk).unwrap_or(1);
                    Self::flush_token(&mut tk, &mut tokens, Token::Literal);

                    match next_char {
                        '>' => {
                            chars.next();
                            tokens.push(Token::Append(fd))
                        }
                        _ => tokens.push(Token::Redirect(fd)),
                    }
                }

                (ch, Quote::None) if ch.is_ascii_whitespace() => {
                    Self::flush_token(&mut tk, &mut tokens, Token::Literal);
                }
                _ => {
                    tk.push(ch);
                }
            }
        }

        Self::flush_token(&mut tk, &mut tokens, Token::Literal);
        self.buffer.clear();

        tokens
    }

    fn flush_token(tk: &mut String, tokens: &mut Vec<Token>, kind: fn(String) -> Token) {
        if !tk.is_empty() {
            tokens.push(kind(std::mem::take(tk)));
        }
    }

    fn take_io_number(tk: &mut String) -> Option<usize> {
        if !tk.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }

        let fd = tk.parse::<usize>().ok()?;
        tk.clear();
        Some(fd)
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

    #[test]
    fn test_redirect_and_append() {
        let mut lex = Lexer::new();

        lex.push("> test.txt");
        let expect = vec![Token::Redirect(1), Token::Literal("test.txt".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("2> test.txt");
        let expect = vec![Token::Redirect(2), Token::Literal("test.txt".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push(">> test.txt");
        let expect = vec![Token::Append(1), Token::Literal("test.txt".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("2>> test.txt");
        let expect = vec![Token::Append(2), Token::Literal("test.txt".to_string())];
        assert_eq!(expect, lex.tokenize());

        lex.push("just2>test.txt");
        let expect = vec![
            Token::Literal("just2".to_string()),
            Token::Redirect(1),
            Token::Literal("test.txt".to_string()),
        ];
        assert_eq!(expect, lex.tokenize());
    }
}
