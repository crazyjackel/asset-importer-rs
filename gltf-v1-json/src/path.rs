use std::fmt;

/// An immutable JSON source path.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Path(pub String);

impl Path {
    pub fn new() -> Self {
        Path(String::new())
    }

    pub fn field(&self, name: &str) -> Self {
        if self.0.is_empty() {
            Path(name.to_string())
        } else {
            Path(format!("{}.{}", self.0, name))
        }
    }

    pub fn index(&self, index: usize) -> Self {
        Path(format!("{}[{}]", self.0, index))
    }

    pub fn key(&self, key: &str) -> Self {
        Path(format!("{}[\"{}\"]", self.0, key))
    }

    pub fn value_str(&self, value: &str) -> Self {
        Path(format!("{} = \"{}\"", self.0, value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
