use std::{collections::VecDeque, io::BufRead};

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    Data(String),
    Comma,
    Key(String),
}

#[derive(Debug, PartialEq)]
pub struct TokenData {
    pub data: Token,
    pub starting_line_number: usize,
    pub starting_char_index: usize,
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

        // Load Line
        if self.char_buffer_queue.is_empty() {
            let mut line = String::new();
            let result = self.reader.read_line(&mut line);
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
        let line = self.char_buffer_queue.front().unwrap();
        if line.is_empty() || self.char_index >= line.len() {
            self.char_buffer_queue.pop_front();
            return self.next();
        }

        // Handle one-shot cases first.
        // These are cases where we can determine the token type and value without having to read further.
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
            }
            ' ' | '\t' | '\n' | '\r' | '\0' | '\u{000C}' => {
                // If we find a whitespace, there is no data to start with.
                self.char_index += 1;
                return self.next();
            }
            _ => {}
        }

        // If we are here, we have a couple of cases to handle.
        // If it is data, we need to read until we find a comma or whitespace/newline.
        // In the case that we find a whitespace/newline, we must read further to confirm there is no colon.
        // If there is a colon, it is a key, rather than data.

        let token_begin = self.char_index;
        let mut token_end = line.len();
        for i in token_begin..line.len() {
            let char = line[i];
            match char {
                '"' | '{' | '}' | ',' | ';' => {
                    // These characters denote the end of token as data and need to be read as the next token.
                    self.char_index = i;
                    token_end = i;
                    return Some(Ok(TokenData {
                        data: Token::Data(line[token_begin..token_end].iter().collect()),
                        starting_line_number: self.line_number - 1,
                        starting_char_index: token_begin,
                    }));
                }
                '\n' | '\r' | '\0' | '\u{000C}' => {
                    // We have found the end of the token data. We still need to check that there is no colon beyond.
                    token_end = i;
                    break;
                }
                ':' => {
                    // If we find a colon, we have a key.
                    self.char_index = i + 1;
                    token_end = i;
                    return Some(Ok(TokenData {
                        data: Token::Key(line[token_begin..token_end].iter().collect()),
                        starting_line_number: self.line_number - 1,
                        starting_char_index: token_begin,
                    }));
                }
                _ => {}
            }

            if char.is_whitespace() {
                // If we find a whitespace, we have found the end of the token data.
                token_end = i;
                break;
            }
        }

        // whilst we know when the token ends, we do not know if it is a key or data.
        // we need to check if the next non-whitespace character is a colon.
        // keep loading lines until we find a non-whitespace character.
        let starting_line_number = self.line_number - 1;
        let key_or_data: String = line[token_begin..token_end].iter().collect();
        let mut read_start = token_end + 1;
        let mut read_end = line.len();
        if read_start >= read_end {
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
            for index in read_start..read_end {
                let char = line[index];
                match char {
                    c if c.is_whitespace() => {}
                    ':' => {
                        self.char_index = index + 1;
                        return Some(Ok(TokenData {
                            data: Token::Key(key_or_data),
                            starting_line_number,
                            starting_char_index: token_begin,
                        }));
                    }
                    '\n' | '\r' | '\0' | '\u{000C}' => {}
                    _ => {
                        self.char_index = index;
                        return Some(Ok(TokenData {
                            data: Token::Data(key_or_data),
                            starting_line_number,
                            starting_char_index: token_begin,
                        }));
                    }
                }
            }
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
}
