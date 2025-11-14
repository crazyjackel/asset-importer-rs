use std::{collections::VecDeque, io::BufRead};

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    Data(String),
    Comma,
    Key(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::Data(data) => write!(f, "\"{}\"", data),
            Token::Comma => write!(f, ","),
            Token::Key(key) => write!(f, "{}:", key),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenData {
    pub data: Token,
    pub starting_line_number: usize,
    pub starting_char_index: usize,
}

impl std::fmt::Display for TokenData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos:({}:{}) {}",
            self.starting_line_number, self.starting_char_index, self.data
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    ReadError(String),
}

pub struct Tokenizer<R: BufRead> {
    reader: R,
    char_buffer_queue: VecDeque<Vec<char>>,
    line_number: usize,
    char_index: usize,
}

impl<R: BufRead> Tokenizer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            char_buffer_queue: VecDeque::new(),
            line_number: 0,
            char_index: 0,
        }
    }
}

impl<R: BufRead> Iterator for Tokenizer<R> {
    type Item = Result<TokenData, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Algorithm Description:
        // We need to read each token from the input stream one by one.
        // Since what we are reading is a stream of characters, it is more efficient to read line by line to save up on utf-8 validation time.
        // Technically, the FBX format is not utf-8, but rather ASCII, but operating on utf-8 gives us less headaches.
        // Each step, our goal is to start reading at char_index, assuming that all the previous characters were correctly read.

        // Load Line, if our buffer queue is empty
        if self.char_buffer_queue.is_empty() {
            let mut line = String::new();
            let result = self.reader.read_line(&mut line);
            // We are unable to read the line, so we return an error.
            if let Err(e) = result {
                return Some(Err(TokenizerError::ReadError(e.to_string())));
            }
            // If the line is empty, we are at the end of the file.
            if result.unwrap() == 0 {
                return None;
            }
            self.char_buffer_queue.push_back(line.chars().collect());
            self.line_number += 1;
            self.char_index = 0;
        }

        // We know that the line is not empty, so we can safely unwrap.
        // If our char_index is out of bounds or we have no data to read, we pop and try again.
        let line = self.char_buffer_queue.front().unwrap();
        if line.is_empty() || self.char_index >= line.len() {
            self.char_buffer_queue.pop_front();
            return self.next();
        }

        // Handle one-shot cases first.
        // These are cases where we can determine the token type and value without having to read further than the current character.
        let char = line[self.char_index];
        match char {
            '{' => {
                // We have found an open brace.
                self.char_index += 1;
                return Some(Ok(TokenData {
                    data: Token::OpenBrace,
                    starting_line_number: self.line_number - 1,
                    starting_char_index: self.char_index - 1,
                }));
            }
            '}' => {
                // We have found a close brace.
                self.char_index += 1;
                return Some(Ok(TokenData {
                    data: Token::CloseBrace,
                    starting_line_number: self.line_number - 1,
                    starting_char_index: self.char_index - 1,
                }));
            }
            ',' => {
                // We have found a comma.
                self.char_index += 1;
                return Some(Ok(TokenData {
                    data: Token::Comma,
                    starting_line_number: self.line_number - 1,
                    starting_char_index: self.char_index - 1,
                }));
            }
            ';' => {
                // The rest of this line is a comment. Pop the line from the queue
                self.char_buffer_queue.pop_front();
                return self.next();
            }
            '"' => {
                // If we are in double quotes, we keep reading until we find the next double quote.
                // We need to handle the case where the double quote is escaped and skip it.
                // The quotes also can go across lines, so we need to keep reading until we find the next double quote.

                // Remember where we discovered the double quote.
                let discover_index = self.char_index;
                let discover_line_number = self.line_number - 1;

                // This is a buffer to build up the data string.
                let mut char_buffer: Vec<char> = Vec::new();
                let mut buffer_ref: &Vec<char> = line;
                let mut line_length = line.len();
                let mut line_char_buffer: Option<Vec<char>> = None;
                loop {
                    self.char_index += 1;
                    if self.char_index >= line_length {
                        let mut new_line = String::new();
                        let result = self.reader.read_line(&mut new_line);
                        // We are unable to read the line, so we return an error.
                        if let Err(e) = result {
                            return Some(Err(TokenizerError::ReadError(e.to_string())));
                        }
                        // If the line is empty, we are at the end of the file.
                        if result.unwrap() == 0 {
                            break;
                        }
                        line_char_buffer = Some(new_line.chars().collect());
                        buffer_ref = line_char_buffer.as_ref().unwrap();
                        line_length = buffer_ref.len();
                        self.line_number += 1;
                        self.char_index = 0;
                    }
                    if buffer_ref[self.char_index] == '"' {
                        break;
                    }
                    char_buffer.push(buffer_ref[self.char_index]);
                }
                if let Some(line_char_buffer) = line_char_buffer {
                    self.char_buffer_queue.pop_front();
                    self.char_buffer_queue.push_back(line_char_buffer);
                }

                // We now have the data string, but we need to check if it is a key or data.
                // We do this by checking if the next non-whitespace, non-endline character is a colon.
                self.char_index += 1;
                return Some(Ok(TokenData {
                    data: Token::Data(char_buffer.iter().collect()),
                    starting_line_number: discover_line_number,
                    starting_char_index: discover_index,
                }));
            }
            ' ' | '\t' | '\n' | '\r' | '\0' | '\u{000C}' => {
                // If we find a whitespace, there is no data to start with.
                // @todo: we can save some checks by peeking ahead for more whitespace and adjusting the char_index accordingly.
                self.char_index += 1;
                return self.next();
            }
            _ => {}
        }

        // If we are here, it is either a key or data
        // If it is data, we need to read until we find a comma, whitespace/newline, or special character.
        // In the case that we find a whitespace/newline, we must read further to confirm there is no colon.
        // If there is a colon, it is a key, rather than data.

        let token_begin = self.char_index;
        let mut token_end = line.len();
        for i in token_begin..line.len() {
            let char = line[i];
            match char {
                '"' | '{' | '}' | ',' | ';' => {
                    // These characters denote the end of token as data.
                    // We set char_index to i to ensure the character is read as the next token.
                    self.char_index = i;
                    token_end = i;
                    return Some(Ok(TokenData {
                        data: Token::Data(line[token_begin..token_end].iter().collect()),
                        starting_line_number: self.line_number - 1,
                        starting_char_index: token_begin,
                    }));
                }
                ':' => {
                    // These characters denote the end of token as key.
                    // We set char_index to i + 1 to ensure the colon is not read as part of the next token.
                    self.char_index = i + 1;
                    if token_begin == i {
                        // If there is no characters before the colon, we ignore it.
                        return self.next();
                    }
                    // Return the key token up to the colon.
                    return Some(Ok(TokenData {
                        data: Token::Key(line[token_begin..i].iter().collect()),
                        starting_line_number: self.line_number - 1,
                        starting_char_index: token_begin,
                    }));
                }
                '\n' | '\r' | '\0' | '\u{000C}' => {
                    // These characters denote the end of the token data.
                    // We still need to check that there is no colon beyond.
                    token_end = i;
                    break;
                }
                c if c.is_whitespace() => {
                    // Whitespace denotes the end of the token data.
                    // We still need to check that there is no colon beyond.
                    token_end = i;
                    break;
                }
                _ => {}
            }
        }

        // whilst we know when the token ends, we do not know if it is a key or data.
        // we need to check if the next non-whitespace character is a colon.
        // keep loading lines until we find a non-whitespace character.
        let starting_line_number = self.line_number - 1;
        let key_or_data: String = line[token_begin..token_end].iter().collect();

        // we have already read the token at token_end, so we start at the next character.
        let mut read_start = token_end + 1;
        let mut read_end = line.len();
        if read_start >= read_end {
            // pop the line if we have read everything.
            self.char_buffer_queue.pop_front();
        }

        loop {
            if self.char_buffer_queue.is_empty() {
                let mut line = String::new();
                let result = self.reader.read_line(&mut line);
                if let Err(e) = result {
                    return Some(Err(TokenizerError::ReadError(e.to_string())));
                }
                // If the line is empty, we are at the end of the file.
                // In this case, we return what we have so far as a data token.
                if result.unwrap() == 0 {
                    return Some(Ok(TokenData {
                        data: Token::Data(key_or_data),
                        starting_line_number,
                        starting_char_index: token_begin,
                    }));
                }
                self.char_buffer_queue.push_back(line.chars().collect());
                self.line_number += 1;
                self.char_index = 0;
                read_start = 0;
                read_end = line.len();
            }
            let line = self.char_buffer_queue.front().unwrap();
            for (index, char) in line.iter().take(read_end).skip(read_start).enumerate() {
                match char {
                    c if c.is_whitespace() => {}
                    '\n' | '\r' | '\0' | '\u{000C}' => {}
                    ':' => {
                        // If we find a colon, we have a key.
                        // We set char_index to index + 1 to ensure the colon is not read as part of the next token.
                        self.char_index = index + 1;
                        return Some(Ok(TokenData {
                            data: Token::Key(key_or_data),
                            starting_line_number,
                            starting_char_index: token_begin,
                        }));
                    }
                    _ => {
                        // If we find a non-whitespace, non-endline character, we have a data token.
                        // We set char_index to index to ensure the character is read as the next token.
                        self.char_index = index;
                        return Some(Ok(TokenData {
                            data: Token::Data(key_or_data),
                            starting_line_number,
                            starting_char_index: token_begin,
                        }));
                    }
                }
            }
            // pop the line if we have read everything.
            self.char_buffer_queue.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_read_line_empty() {
        let input = "";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(tokenizer.next(), None);
    }
    #[test]
    fn test_read_line_empty_line() {
        let input = "\n";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_read_line_comment() {
        let input = "; This is a comment\n";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_read_line_key() {
        let input = "Key: Value\n";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Key("Key".to_string()),
                starting_line_number: 0,
                starting_char_index: 0,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Value".to_string()),
                starting_line_number: 0,
                starting_char_index: 5,
            }))
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_read_line_key_with_whitespace() {
        let input = "Key : Value\n";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Key("Key".to_string()),
                starting_line_number: 0,
                starting_char_index: 0,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Value".to_string()),
                starting_line_number: 0,
                starting_char_index: 6,
            }))
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_read_line_braces() {
        let input = "{Hello World}";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::OpenBrace,
                starting_line_number: 0,
                starting_char_index: 0,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Hello".to_string()),
                starting_line_number: 0,
                starting_char_index: 1,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("World".to_string()),
                starting_line_number: 0,
                starting_char_index: 7,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::CloseBrace,
                starting_line_number: 0,
                starting_char_index: 12,
            }))
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_read_line() {
        let input = r#"
FBXHeaderExtension:  {
    FBXHeaderVersion: 1003
}"#;
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Key("FBXHeaderExtension".to_string()),
                starting_line_number: 1,
                starting_char_index: 0,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::OpenBrace,
                starting_line_number: 1,
                starting_char_index: 21,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Key("FBXHeaderVersion".to_string()),
                starting_line_number: 2,
                starting_char_index: 4,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("1003".to_string()),
                starting_line_number: 2,
                starting_char_index: 22,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::CloseBrace,
                starting_line_number: 3,
                starting_char_index: 0,
            }))
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_read_line_data_with_quotes() {
        let input = r#""Hello World""#;
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Hello World".to_string()),
                starting_line_number: 0,
                starting_char_index: 0,
            }))
        );
    }

    #[test]
    fn test_read_line_data_with_quotes_and_whitespace() {
        let input = "\t\tVertices: *6324 {";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Key("Vertices".to_string()),
                starting_line_number: 0,
                starting_char_index: 2,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("*6324".to_string()),
                starting_line_number: 0,
                starting_char_index: 12,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::OpenBrace,
                starting_line_number: 0,
                starting_char_index: 17,
            }))
        );
    }

    #[test]
    fn test_read_line_data_with_multiple_quotes() {
        let input = r#""Hello World" "Goodbye World""#;
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Hello World".to_string()),
                starting_line_number: 0,
                starting_char_index: 0,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Goodbye World".to_string()),
                starting_line_number: 0,
                starting_char_index: 14,
            }))
        );
    }

    #[test]
    fn test_read_line_data_key_value() {
        let input = r#"Key: "Value""#;
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Key("Key".to_string()),
                starting_line_number: 0,
                starting_char_index: 0,
            }))
        );
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Value".to_string()),
                starting_line_number: 0,
                starting_char_index: 5,
            }))
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_read_line_unescape_quote() {
        let input = r#" "Hello World"#;
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Hello World".to_string()),
                starting_line_number: 0,
                starting_char_index: 1,
            }))
        );
    }

    #[test]
    fn test_read_multiline_data_with_quotes() {
        let input = r#""Hello World
Hello World
Hello World""#;
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.next(),
            Some(Ok(TokenData {
                data: Token::Data("Hello World\nHello World\nHello World".to_string()),
                starting_line_number: 0,
                starting_char_index: 0,
            }))
        );
    }
}
