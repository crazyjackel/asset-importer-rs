use std::io::BufRead;

use crate::{Token, Tokenizer, TokenizerError};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    TokenizerError(TokenizerError),
    OpenBraceNoKey,
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub key: String,
    pub tokens: Vec<String>,
    pub parent_index: Option<usize>,
}

impl Element {
    pub fn new(key: String) -> Self {
        Self {
            key,
            tokens: Vec::new(),
            parent_index: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ElementArena {
    pub elements: Vec<Element>,
    pub root_index: usize,
}

impl ElementArena {
    pub fn insert(&mut self, element: Element) -> usize {
        let index = self.elements.len();
        self.elements.push(element);
        index
    }

    pub fn get(&self, index: usize) -> Option<&Element> {
        self.elements.get(index)
    }
}

impl ElementArena {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            root_index: 0,
        }
    }
}
pub struct Parser<R: BufRead> {
    tokenizer: Tokenizer<R>,
    element_arena: ElementArena,
}

impl<R: BufRead> Parser<R> {
    pub fn new(tokenizer: Tokenizer<R>) -> Self {
        Self {
            tokenizer,
            element_arena: ElementArena::new(),
        }
    }
    
    pub fn load(mut self) -> Result<ElementArena, ParserError> {
        let mut iter = self.iter();
        while let Some(_result) = iter.next() {
        }
        Ok(self.element_arena)
    }

    pub fn get_arena_ref(&self) -> &ElementArena {
        &self.element_arena
    }

    pub fn iter(&'_ mut self) -> ParserIter<'_, R> {
        ParserIter {
            tokenizer: &mut self.tokenizer,
            parser_arena: &mut self.element_arena,
            current_scope: None,
            current_element: None,
        }
    }
}

pub struct ParserIter<'a, R: BufRead> {
    tokenizer: &'a mut Tokenizer<R>,
    parser_arena: &'a mut ElementArena,
    current_scope: Option<usize>,
    current_element: Option<Element>,
}

impl<'a, R: BufRead> Iterator for ParserIter<'a, R> {
    type Item = Result<usize, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(token) = self.tokenizer.next() {
            let valid_token = match token {
                Ok(t) => t,
                Err(e) => return Some(Err(ParserError::TokenizerError(e))),
            };
            // We are trying to parse an element, which is made up of a key and a set of tokens.
            match valid_token.data {
                Token::OpenBrace => {
                    // Return the current element and set it's index as the current scope.
                    if self.current_element.is_none() {
                        return Some(Err(ParserError::OpenBraceNoKey));
                    }
                    // move current element out of the option, current_element is guarenteed to have parent_index
                    let element: Element = self.current_element.take().unwrap();
                    let index = self.parser_arena.insert(element);
                    self.current_scope = Some(index);
                    return Some(Ok(index));
                }
                Token::CloseBrace => {
                    // If Close Brace we should move out of the current scope, moving up into the parent scope.
                    // We then should return the next element in the parent scope.
                    if let Some(index) = self.current_scope {
                        if let Some(element) = self.parser_arena.get(index) {
                            // element is guarenteed to have parent_index
                            self.current_scope = element.parent_index;
                            continue;
                        }
                    }
                    return None;
                }
                Token::Data(data) => {
                    // If Data we should add its string value as a token to the current element.
                    if let Some(ref mut element) = self.current_element {
                        element.tokens.push(data);
                    }
                }
                Token::Key(key) => {
                    // If we find a key, we should return the current element and create a new one with the key as the key.
                    if let Some(element) = self.current_element.take() {
                        let index = self.parser_arena.insert(element);
                        let mut new_element = Element::new(key);
                        new_element.parent_index = self.current_scope;
                        self.current_element = Some(new_element);
                        return Some(Ok(index));
                    }
                    let mut element = Element::new(key);
                    element.parent_index = self.current_scope;
                    self.current_element = Some(element);
                }
                _ => {
                    continue;
                }
            }
        }
        // If we are here, we have reached the end of the file.
        // We should return the current element if it exists.
        if let Some(element) = self.current_element.take() {
            let index = self.parser_arena.insert(element);
            return Some(Ok(index));
        }
        // If we are here, we have no more elements to return.
        None
    }
}


#[cfg(test)]
mod tests {

    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_parser_load_empty() {
        let input = "";
        let tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        let parser = Parser::new(tokenizer);
        let elements = parser.load().unwrap();
        assert_eq!(elements.elements.len(), 0);
    }

    #[test]
    fn test_parser_read_line_key() {
        let input = "Key: Value\n";
        let tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        let parser = Parser::new(tokenizer);
        let elements = parser.load().unwrap();
        assert_eq!(elements.elements.len(), 1);
        assert_eq!(elements.elements[0].key, "Key");
        assert_eq!(elements.elements[0].tokens, vec!["Value"]);
    }

    #[test]
    fn test_parser_read_line(){
        let input = r#"
FBXHeaderExtension:  {
    FBXHeaderVersion: 1003
}"#;
        let tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        let parser = Parser::new(tokenizer);
        let elements = parser.load().unwrap();
        assert_eq!(elements.elements.len(), 2);
        assert_eq!(elements.elements[0].key, "FBXHeaderExtension");
        assert_eq!(elements.elements[0].tokens.len(), 0);
        assert_eq!(elements.elements[0].parent_index, None);
        assert_eq!(elements.elements[1].key, "FBXHeaderVersion");
        assert_eq!(elements.elements[1].tokens, vec!["1003"]);
        assert_eq!(elements.elements[1].parent_index, Some(0));
    }
}