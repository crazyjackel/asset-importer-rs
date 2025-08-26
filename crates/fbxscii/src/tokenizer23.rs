use std::{io::BufRead, str};

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    Data(String),
    Comma,
    Key(String),
    EOF,
}

pub struct Tokenizer<R: BufRead> {
    reader: R,
    done: bool,
}

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    ReadError(String),
}

trait IsLineEnding {
    fn is_line_ending(&self) -> bool;
}

impl IsLineEnding for char {
    fn is_line_ending(&self) -> bool {
        matches!(*self, '\n' | '\r' | '\0' | '\u{000C}')
    }
}

impl<R: BufRead> Tokenizer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            done: false,
        }
    }
}

impl<R: BufRead> Tokenizer<R> {
    fn read_line(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut line_string = String::new();
        let result = self
            .reader
            .read_line(&mut line_string)
            .map_err(|e| TokenizerError::ReadError(e.to_string()))?;
        if result == 0 {
            return Ok(vec![Token::EOF]);
        }

        let mut tokens = Vec::new();
        let char_vec = line_string.chars().collect::<Vec<char>>();
        let mut token_begin = 0;
        let token_end = char_vec.len();
        let mut in_double_quote = false;
        let mut comment = false;
        for i in 0..token_end {
            if i < token_begin {
                continue;
            }
            let c = char_vec[i];

            if c.is_line_ending() {
                comment = false;
            }

            if comment {
                continue;
            }

            if in_double_quote {
                if c == '"' {
                    in_double_quote = false;
                    tokens.push(Token::Data(line_string[token_begin..i].to_string()));
                }
                continue;
            }

            match c {
                '"' => {
                    token_begin = i;
                    in_double_quote = true;
                }
                ';' => {
                    if token_begin < i {
                        tokens.push(Token::Data(line_string[token_begin..i].to_string()));
                    }
                    comment = true;
                }
                '{' => {
                    if token_begin < i {
                        tokens.push(Token::Data(line_string[token_begin..i].to_string()));
                    }
                    tokens.push(Token::OpenBrace);
                }
                '}' => {
                    if token_begin < i {
                        tokens.push(Token::Data(line_string[token_begin..i].to_string()));
                    }
                    tokens.push(Token::CloseBrace);
                }
                ',' => {
                    tokens.push(Token::Data(line_string[token_begin..i].to_string()));
                    tokens.push(Token::Comma);
                    continue;
                }
                ':' => {
                    tokens.push(Token::Key(line_string[token_begin..i].to_string()));
                    continue;
                }
                '\n' | '\r' | '\0' | '\u{000C}' => {
                    if token_begin < i {
                        tokens.push(Token::Data(line_string[token_begin..i].to_string()));
                    }
                    continue;
                }
                _ => {}
            }

            if c.is_whitespace() || c.is_line_ending() {
                token_begin += 1;
                for j in i..token_end {
                    let peek = char_vec[j];
                    if peek.is_whitespace() {
                        continue;
                    }

                    if peek == ':' {
                        tokens.push(Token::Key(line_string[token_begin..j].to_string()));
                    }
                    token_begin = j;
                    break;
                }
            }
        }
        Ok(tokens)
    }
}

// impl<R: BufRead> Iterator for Tokenizer<R> {
//     type Item = Result<Token, TokenizerError>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.done {
//             return None;
//         }
//         // If we're at the end of the line, read the next line
//         while self.column as usize == self.line_string.len()
//             || self.comment
//             || self.line_string.is_empty()
//             || self.line_string == "\n"
//         {
//             self.comment = false;
//             self.column = 0;
//             let result = self
//                 .reader
//                 .read_line(&mut self.line_string)
//                 .map_err(|e| TokenizerError::ReadError(e.to_string()));
//             if result.is_err() {
//                 self.done = true;
//                 return Some(Err(result.unwrap_err()));
//             }

//             self.line += 1;
//         }

//         let bytes = self.line_string.as_bytes();
//         let mut token_begin = self.column as usize;
//         let token_end = self.line_string.len();
//         for i in token_begin..token_end {
//             let c = bytes[i] as char;
//             self.column += 1;
//             match c {
//                 '{' => {
//                     return Some(Ok(Token::OpenBrace));
//                 }
//                 '}' => {
//                     return Some(Ok(Token::CloseBrace));
//                 }
//                 ',' => {
//                     return Some(Ok(Token::Comma));
//                 }
//                 '"' if self.double_quote => {
//                     self.double_quote = false;
//                     return Some(Ok(Token::Data(
//                         self.line_string[token_begin..i - 1].to_string(),
//                     )));
//                 }
//                 '"' => {
//                     token_begin = i;
//                     self.double_quote = true;
//                 }
//                 ';' => {
//                     self.comment = true;
//                     return Some(Ok(Token::Data(
//                         self.line_string[token_begin..i].to_string(),
//                     )));
//                 }
//                 _ => {}
//             }
//         }
//         Some(Ok(Token::Data(
//             self.line_string[token_begin..token_end].to_string(),
//         )))
//     }
// }

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_read_line_empty() {
        let input = "";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(tokenizer.read_line(), Ok(vec![Token::EOF]));
    }

    #[test]
    fn test_read_line_empty_line() {
        let input = "\n";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(tokenizer.read_line(), Ok(vec![]));
    }

    #[test]
    fn test_read_line_comment() {
        let input = "; This is a comment\n";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(tokenizer.read_line(), Ok(vec![]));
    }

    #[test]
    fn test_read_line_key() {
        let input = "Key: Value\n";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.read_line(),
            Ok(vec![
                Token::Key("Key".to_string()),
                Token::Data("Value".to_string())
            ])
        );
    }

    #[test]
    fn test_read_line_key_with_whitespace() {
        let input = "Key : Value\n";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.read_line(),
            Ok(vec![
                Token::Key("Key".to_string()),
                Token::Data("Value".to_string())
            ])
        );
    }

    #[test]
    fn test_read_line_braces() {
        let input = "{Hello World}";
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(
            tokenizer.read_line(),
            Ok(vec![
                Token::OpenBrace,
                Token::Data("Hello".to_string()),
                Token::Data("World".to_string()),
                Token::CloseBrace
            ])
        );
    }

    #[test]
    fn test_read_line() {
        let input = r#"
        FBXHeaderExtension:  {
            FBXHeaderVersion: 1003
        }"#;
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        assert_eq!(tokenizer.read_line(), Ok(vec![]));
        assert_eq!(
            tokenizer.read_line(),
            Ok(vec![
                Token::Key("FBXHeaderExtension".to_string()),
                Token::OpenBrace
            ])
        );
        assert_eq!(
            tokenizer.read_line(),
            Ok(vec![
                Token::Key("FBXHeaderVersion".to_string()),
                Token::Data("1003".to_string()),
            ])
        );
        assert_eq!(tokenizer.read_line(), Ok(vec![Token::CloseBrace]));
        assert_eq!(tokenizer.read_line(), Ok(vec![]));
    }
}
