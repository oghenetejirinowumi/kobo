use crate::token::{keyword, Token, TokenType};

/// Cut `source` into tokens. Returns everything it managed to scan alongside every
/// error it found; the caller decides whether to go on.
pub fn scan(source: &str) -> (Vec<Token>, Vec<String>) {
    let mut s = Scanner {
        src: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    s.run();
    (s.tokens, s.errors)
}

struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl Scanner {
    fn run(&mut self) {
        // TODO(you): drive the scan: read one token at a time until the source runs out, then
        //            add the EOF token. Spec 6.1 says which line EOF carries.

        while !self.at_end() {
            self.start = self.current; 
            self.scan_token();
        
        }

        self.line = self.tokens.last().map(|t| t.line).unwrap_or(1);

        self.start = self.current;
        self.add(TokenType::Eof);
    }

    fn scan_token(&mut self) {
        // TODO(you): recognise one token. Spec 1.2 lists every token type, 1.1 covers
        //            whitespace and comments, and an unrecognised character is 'Character is
        //            not part of any token.' (5.1).

        let c = self.advance();
        match c {
            // PUNCTUATION
            '(' => self.add(TokenType::LParen),
            ')' => self.add(TokenType::RParen),
            '{' => self.add(TokenType::LBrace),
            '}' => self.add(TokenType::RBrace),
            ',' => self.add(TokenType::Comma),
            ';' => self.add(TokenType::Semicolon),
            // '.' => self.add(TokenType::Dot),

            // ARITHMETIC
            '+' => self.add(TokenType::Plus),
            '-' => self.add(TokenType::Minus),
            '*' => self.add(TokenType::Star),

            '/' => {
                if self.peek() == '/' {
                    self.advance();
                    // To consume the second slash
                    while !self.at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add(TokenType::Slash);
                }
            }

            // EQUALITY & COMPARISON, NEGATION ASSIGNMENT
            '!' => {
                if !self.at_end() && self.peek() == '=' {
                    self.advance();
                    self.add(TokenType::BangEqual);
                } else {
                    self.add(TokenType::Bang)
                }
            }

            '=' => {
                if !self.at_end() && self.peek() == '=' {
                    self.advance();
                    self.add(TokenType::EqualEqual);
                } else {
                    self.add(TokenType::Equal)
                }
            }

            '<' => {
                if !self.at_end() && self.peek() == '=' {
                    self.advance();
                    self.add(TokenType::LessEqual);
                } else {
                    self.add(TokenType::Less)
                }
            }

            '>' => {
                if !self.at_end() && self.peek() == '=' {
                    self.advance();
                    self.add(TokenType::GreaterEqual);
                } else {
                    self.add(TokenType::Greater)
                }
            }

            ' ' | '\t' | '\r' => {}

            '\n' => {
                self.line +=1
            }

            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => self.error(self.line, "Character is not part of any token."),
        }

    }

    fn string(&mut self) {
        // TODO(you): scan a string literal. A string may span lines (1.5); an unterminated one
        //            is reported at the line it opened on (5.1).
            let opening_line = self.line;

            while !self.at_end() && self.peek() != '"' {
                if self.peek() == '\n' {
                    self.line += 1;
                } 
                self.advance();
            }

            if self.at_end() {
                self.error(opening_line, "String is never closed.");
                return;
            }

            self.advance();
            self.add(TokenType::Str);
    }

    fn number(&mut self) {
        // TODO(you): scan a number literal: digits, then a fractional part only when a digit
        //            follows the dot (1.4).
        while self.peek().is_ascii_digit(){
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit(){
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add(TokenType::Number);
    }

    fn identifier(&mut self) {
        // TODO(you): scan an identifier, then decide whether it is a keyword; keyword() in
        //            token.rs does the lookup (1.2, 1.3).
        // while (self.peek() >= 'a' && self.peek() <= 'z') ||
        //       (self.peek() >= 'A' && self.peek() <= 'Z') ||
        //       (self.peek() >= '0' && self.peek() <= '9') ||
        //       self.peek() == '_' {
        //         self.advance();
        //       }

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        // let mut identifier = String::new();
        // for i in self.start..self.current }
        // identifier.push(self.src[i]);

        let word: String = self.src[self.start..self.current].iter().collect();

        // let kind = match keyword(%word) {
        //     Some() => t;
        //     None => TokenType::Identifier,
        // };

        let kind = keyword(&word).unwrap_or(TokenType::Identifier);

        self.add(kind);
    }

    // --- primitives ---------------------------------------------------------------

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = self.src[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() || self.src[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src[self.current + 1]
        }
    }

    fn add(&mut self, kind: TokenType) {
        self.tokens.push(Token {
            kind,
            lexeme: self.src[self.start..self.current].iter().collect(),
            line: self.line,
        });
    }

    fn error(&mut self, line: usize, message: &str) {
        self.errors.push(format!("[line {}] Error: {}", line, message));
    }
}
