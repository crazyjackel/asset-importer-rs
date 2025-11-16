use std::{io::BufRead, num::ParseIntError};

use crate::{Token, Tokenizer, TokenizerError};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    TokenizerError(TokenizerError),
    OpenBraceNoKey,
}

/// Element index type alias for clarity.
pub type ElementIndex = usize;

#[derive(Debug, PartialEq)]
pub struct Element {
    pub key: String,
    pub tokens: Vec<String>,
    pub children: Vec<usize>,
    pub parent_index: Option<usize>,
}

impl Element {
    pub fn new(key: String) -> Self {
        Self {
            key,
            tokens: Vec::new(),
            children: Vec::new(),
            parent_index: None,
        }
    }
}

// Data Structure Notes:
// For the Parser, we went with an arena-based approach for dealing with element tree.
// 
// Our implementation was a Vec<Element> where each element had a parent_index and children indices.
// We could store those indices independent of the elements via some hashmaps, but this would only be beneficial if our data was sparse.
// Other considerations were to use indextree:Arena or something similar, wherein each element stores 
// parent_index, next_sibling, previous_sibling, first_child, last_child.
// This however, provides significant indirection and the benefits of quicker removal is not neccessary.
// Last consideration was forgoing either children or parent indices and reconstructing that information as needed.
// Whilst sensible, I determined that it was additionally complex and the performance benefits were unclear as of writing.
/// An ElementAmphitheatre is an Arena that stores children and parent indices within each Node.
/// The Node is an Element with keys and tokens.
#[derive(Debug, PartialEq)]
pub struct ElementAmphitheatre {
    elements: Vec<Element>
}

impl ElementAmphitheatre {
    pub fn new() -> Self {
        Self {
            elements: Vec::new()
        }
    }
    pub fn insert(&mut self, element: Element) -> usize {
        let index = self.elements.len();
        self.elements.push(element);
        index
    }

    pub fn get(&self, index: ElementIndex) -> Option<&Element> {
        self.elements.get(index)    
    }   
    
    pub fn get_mut(&mut self, index: ElementIndex) -> Option<&mut Element> {
        self.elements.get_mut(index)
    }

    pub fn get_by_key(&self, key: &str) -> Option<&Element> {
        self.elements.iter().find(|element| element.key == key)
    }

    /// Creates a new empty `ElementAmphitheatre` with enough capacity to store `n` elements.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            elements: Vec::with_capacity(n),
        }
    }

    /// Returns the number of elements the arena can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.elements.capacity()
    }

    /// Reserves capacity for `additional` more elements to be inserted.
    ///
    /// The arena may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds isize::MAX bytes.
    pub fn reserve(&mut self, additional: usize) {
        self.elements.reserve(additional);
    }

    /// Counts the number of elements in arena and returns it.
    pub fn count(&self) -> usize {
        self.elements.len()
    }

    /// Returns `true` if arena has no elements, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns an iterator of all elements in the arena in storage-order.
    pub fn iter(&self) -> std::slice::Iter<'_, Element> {
        self.elements.iter()
    }

    /// Returns a mutable iterator of all elements in the arena in storage-order.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Element> {
        self.elements.iter_mut()
    }

    /// Clears all the elements in the arena, but retains its allocated capacity.
    ///
    /// Note that this completely removes all elements from the arena storage,
    /// thus invalidating all the indices that were previously created.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Returns a slice of the inner elements collection.
    pub fn as_slice(&self) -> &[Element] {
        self.elements.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Element] {
        self.elements.as_mut_slice()
    }
    
    /// Returns an element handle for the given index.
    ///
    /// Returns `None` if the index is not valid.
    #[inline]
    #[must_use]
    pub fn get_handle(&self, index: ElementIndex) -> Option<ElementHandle<'_>> {
        if self.get(index).is_some() {
            Some(ElementHandle::new(self, index))
        } else {
            None
        }
    }

    /// Returns an element handle for the first element with the given key.
    #[inline]
    #[must_use]
    pub fn get_handle_by_key(&self, key: &str) -> Option<ElementHandle<'_>> {
        self.elements
            .iter()
            .enumerate()
            .find(|(_, element)| element.key == key)
            .map(|(index, _)| ElementHandle::new(self, index))
    }
}


/// Element handle providing safe access to elements in an arena.
#[derive(Debug, Clone, Copy)]
pub struct ElementHandle<'a> {
    /// The arena the element belongs to.
    arena: &'a ElementAmphitheatre,
    /// Element index.
    index: ElementIndex,
}

impl<'a> ElementHandle<'a> {
    /// Creates a new `ElementHandle`.
    ///
    /// # Panics
    ///
    /// This may panic if the given index is not valid in the given arena.
    ///
    /// Even if `new()` does not panic, subsequent operations through
    /// `ElementHandle` object may panic if the given index is not valid in the
    /// given arena.
    #[inline]
    #[must_use]
    pub fn new(arena: &'a ElementAmphitheatre, index: ElementIndex) -> Self {
        assert!(
            arena.get(index).is_some(),
            "The element index is not valid in the given arena: index={}",
            index
        );

        Self { arena, index }
    }

    /// Returns a reference to the arena.
    #[inline]
    #[must_use]
    pub fn arena(&self) -> &'a ElementAmphitheatre {
        self.arena
    }

    /// Returns the element index.
    #[inline]
    #[must_use]
    pub fn index(&self) -> ElementIndex {
        self.index
    }

    /// Returns the internally managed element data.
    #[inline]
    #[must_use]
    fn element(&self) -> &'a Element {
        self.arena.get(self.index).expect("Element index should be valid")
    }

    /// Returns the element key.
    #[inline]
    #[must_use]
    pub fn key(&self) -> &'a str {
        &self.element().key
    }

    /// Returns the element tokens.
    #[inline]
    #[must_use]
    pub fn tokens(&self) -> &'a [String] {
        &self.element().tokens
    }

    /// Returns an iterator of children.
    #[inline]
    #[must_use]
    pub fn children(&self) -> ElementChildren<'a> {
        ElementChildren {
            arena: self.arena,
            indices: self.element().children.iter(),
        }
    }

    /// Returns an iterator of children with the given key.
    #[inline]
    #[must_use]
    pub fn children_by_key(&self, key: &str) -> ElementChildrenByKey<'a> {
        ElementChildrenByKey {
            key: key.to_string(),
            children_iter: self.children(),
        }
    }

    /// Returns the first child with the given key.
    #[inline]
    #[must_use]
    pub fn first_child_by_key(&self, key: &str) -> Option<Self> {
        self.children_by_key(key).next()
    }

    /// Returns parent element handle if available.
    #[inline]
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        self.element()
            .parent_index
            .map(|parent_index| Self::new(self.arena, parent_index))
    }

    /// Returns first child element handle if available.
    #[inline]
    #[must_use]
    pub fn first_child(&self) -> Option<Self> {
        self.element()
            .children
            .first()
            .map(|&index| Self::new(self.arena, index))
    }

    /// Returns last child element handle if available.
    #[inline]
    #[must_use]
    pub fn last_child(&self) -> Option<Self> {
        self.element()
            .children
            .last()
            .map(|&index| Self::new(self.arena, index))
    }

    /// Returns previous sibling element handle if available.
    ///
    /// This requires finding the current element in its parent's children list.
    #[inline]
    #[must_use]
    pub fn previous_sibling(&self) -> Option<Self> {
        let parent = self.parent()?;
        let parent_element = parent.element();
        let position = parent_element
            .children
            .iter()
            .position(|&idx| idx == self.index)?;
        
        if position > 0 {
            let prev_index = parent_element.children[position - 1];
            Some(Self::new(self.arena, prev_index))
        } else {
            None
        }
    }

    /// Returns next sibling element handle if available.
    ///
    /// This requires finding the current element in its parent's children list.
    #[inline]
    #[must_use]
    pub fn next_sibling(&self) -> Option<Self> {
        let parent = self.parent()?;
        let parent_element = parent.element();
        let position = parent_element
            .children
            .iter()
            .position(|&idx| idx == self.index)?;
        
        if position + 1 < parent_element.children.len() {
            let next_index = parent_element.children[position + 1];
            Some(Self::new(self.arena, next_index))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ElementParseError {
    MissingValueToken,
    ParseError(String),
}

impl TryFrom<ElementHandle<'_>> for u32 {
    type Error = ElementParseError;
    fn try_from(value: ElementHandle<'_>) -> Result<Self, Self::Error> {
        let value = value.tokens().first().ok_or(ElementParseError::MissingValueToken)?;
        let result = value.parse::<u32>().map_err(|e| ElementParseError::ParseError(e.to_string()))?;
        Ok(result)
    }
}

impl TryFrom<ElementHandle<'_>> for String {
    type Error = ElementParseError;
    fn try_from(value: ElementHandle<'_>) -> Result<Self, Self::Error> {
        let value = value.tokens().first().ok_or(ElementParseError::MissingValueToken)?;
        Ok(value.to_string())
    }
}

/// An iterator of children of an element.
#[derive(Clone)]
pub struct ElementChildren<'a> {
    /// Arena.
    arena: &'a ElementAmphitheatre,
    /// Iterator over child indices.
    indices: std::slice::Iter<'a, usize>,
}

impl<'a> Iterator for ElementChildren<'a> {
    type Item = ElementHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = *self.indices.next()?;
        Some(ElementHandle::new(self.arena, index))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl ExactSizeIterator for ElementChildren<'_> {}

impl std::iter::FusedIterator for ElementChildren<'_> {}

impl<'a> std::fmt::Debug for ElementChildren<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ElementChildren").finish()
    }
}

/// An iterator of children of an element, with a specific key.
#[derive(Clone)]
pub struct ElementChildrenByKey<'a> {
    /// Key to match.
    key: String,
    /// Children element iterator.
    children_iter: ElementChildren<'a>,
}

impl<'a> Iterator for ElementChildrenByKey<'a> {
    type Item = ElementHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.children_iter
            .find(|child| child.key() == self.key)
    }
}

impl std::iter::FusedIterator for ElementChildrenByKey<'_> {}

impl<'a> std::fmt::Debug for ElementChildrenByKey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ElementChildrenByKey")
            .field("key", &self.key)
            .finish()
    }
}


pub struct Parser<R: BufRead> {
    tokenizer: Tokenizer<R>,
    element_arena: ElementAmphitheatre,
}

impl<R: BufRead> Parser<R> {
    pub fn new(tokenizer: Tokenizer<R>) -> Self {
        Self {
            tokenizer,
            element_arena: ElementAmphitheatre::new(),
        }
    }
    
    pub fn load(mut self) -> Result<ElementAmphitheatre, ParserError> {
        let mut iter = self.iter();
        while let Some(_result) = iter.next() {
        }
        Ok(self.element_arena)
    }

    pub fn get_arena_ref(&self) -> &ElementAmphitheatre {
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
    parser_arena: &'a mut ElementAmphitheatre,
    current_scope: Option<usize>,
    current_element: Option<Element>,
}

impl<'a, R: BufRead> ParserIter<'a, R> {
    pub fn create_element(&mut self, key: String) {   
        let mut new_element = Element::new(key);
        new_element.parent_index = self.current_scope;
        self.current_element = Some(new_element);
    }
    
    pub fn insert_element(&mut self, element: Element) -> usize {
        let parent_index = element.parent_index;
        let index = self.parser_arena.insert(element);
        if let Some(parent_index) = parent_index{
            if let Some(parent) = self.parser_arena.get_mut(parent_index) {
                parent.children.push(index);
            }   
        }
        index
    }
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

                    // move current element out of the option and insert it.
                    let element: Element = self.current_element.take().unwrap();
                    let index = self.insert_element(element);

                    // Update the current scope to the new element.
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
                        let index = self.insert_element(element);
                        self.create_element(key);
                        return Some(Ok(index));
                    }
                    self.create_element(key);
                }
                _ => {
                    continue;
                }
            }
        }
        // If we are here, we have reached the end of the file.
        // We should return the current element if it exists.
        if let Some(element) = self.current_element.take() {
            let index = self.insert_element(element);
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