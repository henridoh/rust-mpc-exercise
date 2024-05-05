use std::iter::Peekable;
use crate::circuit::error::ParserError;
use crate::circuit::tokenizer::LexicalUnit::NewLine;

#[derive(Debug)]
pub enum LexicalUnit {
    Number(usize),
    Identifier(String),
    NewLine,
    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct Token {
    pub location: Location,
    pub value: LexicalUnit,
}

pub struct TokenStream<'a> {
    char_stream: Peekable<&'a mut dyn Iterator<Item=char>>,
    location: Location,
}

impl<'a> TokenStream<'a> {
    pub fn new(char_stream: &'a mut dyn Iterator<Item=char>) -> Self {
        TokenStream {
            char_stream: char_stream.peekable(),
            location: Location { column: 0, line: 0 },
        }
    }

    pub fn current_location(&self) -> Location {
        self.location.clone()
    }

    pub fn accept_newline(&mut self) -> Result<(), ParserError> {
        let token = self.next();

        match token {
            Some(Token { value: NewLine, .. }) =>
                Ok(()),

            Some(token @ Token { .. }) =>
                Err(ParserError::Token {
                    expected: "NewLine",
                    actual: token
                }),

            None =>
                Err(ParserError::Token {
                    expected: "Newline",
                    actual: Token { location: self.location.clone(), value: LexicalUnit::EndOfFile }
                })
        }
    }

    pub fn accept_number(&mut self) -> Result<usize, ParserError> {
        let token = self.next();

        match token {
            Some(Token { value: LexicalUnit::Number(n), .. }) =>
                Ok(n),

            Some(t @ Token { .. }) =>
                Err(ParserError::Token {
                    expected: "Number",
                    actual: t
                }),

            None =>
                Err(ParserError::Token {
                    expected: "Number",
                    actual: Token { location: self.location.clone(), value: LexicalUnit::EndOfFile }
                })
        }
    }

    pub fn accept_n_numbers(&mut self, n: usize) -> Result<Vec<usize>, ParserError> {
        let mut numbers = Vec::with_capacity(n);
        for _ in 0..n {
            numbers.push(self.accept_number()?)
        }
        Ok(numbers)
    }

    pub fn accept_identifier(&mut self) -> Result<String, ParserError> {
        let token = self.next();

        match token {
            Some(Token { value: LexicalUnit::Identifier(s), .. }) =>
                Ok(s),

            Some(t @ Token { .. }) =>
                Err(ParserError::Token {
                    expected: "Identifier",
                    actual: t
                }),

            None =>
                Err(ParserError::Token {
                    expected: "Identifier",
                    actual: Token { location: self.location.clone(), value: LexicalUnit::EndOfFile }
                })
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ') = self.char_stream.peek() {
            self.char_stream.next();
            self.location.column += 1
        }
    }

    fn parse_number(&mut self) -> Token {
        let mut number = String::new();
        let location = self.location.clone();

        while let Some(digit) = self.char_stream.next_if(|x| x.is_numeric()) {
            self.location.column += 1;
            number.push(digit);
        }

        let number: usize = number.parse().expect("Number should be parseable here");

        Token {
            location,
            value: LexicalUnit::Number(number)
        }
    }

    fn parse_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let location = self.location.clone();

        while let Some(c) = self.char_stream.next_if(|&x| x != ' ' && x != '\n') {
            self.location.column += 1;
            identifier.push(c)
        }

        Token{
            location,
            value: LexicalUnit::Identifier(identifier)
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let c = self.char_stream.peek()?;
        
        if *c == '\n' {
            self.char_stream.next();
            let token = Some(Token{
                location: self.location.clone(),
                value: NewLine
            });
            self.location = Location {
                line: self.location.line + 1,
                column: 0
            };

            token
        } else if c.is_numeric() {
            Some(self.parse_number())
        } else {
            Some(self.parse_identifier())
        }
    }
}