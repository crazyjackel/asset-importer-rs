use fbxscii::ElementArena;
use fbxscii::Parser;
use std::io::BufRead;

pub struct Document(ElementArena);

impl Document {
    pub fn from_parser<R>(parser: Parser<R>) -> Self
    where
        R: BufRead,
    {
        let elements = parser.load().unwrap();
        Self(elements)
    }
}