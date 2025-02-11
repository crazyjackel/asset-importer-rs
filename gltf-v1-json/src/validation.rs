use serde::ser;
use serde::{Serialize, Serializer};

/// Specifies a type that has been pre-validated during deserialization or otherwise.
#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Checked<T> {
    /// The item is valid.
    Valid(T),

    /// The item is invalid.
    Invalid,
}

impl<T> Checked<T> {
    /// Converts from `Checked<T>` to `Checked<&T>`.
    pub fn as_ref(&self) -> Checked<&T> {
        match *self {
            Checked::Valid(ref item) => Checked::Valid(item),
            Checked::Invalid => Checked::Invalid,
        }
    }

    /// Takes ownership of the contained item if it is `Valid`.
    ///
    /// # Panics
    ///
    /// Panics if called on an `Invalid` item.
    pub fn unwrap(self) -> T {
        match self {
            Checked::Valid(item) => item,
            Checked::Invalid => panic!("attempted to unwrap an invalid item"),
        }
    }
}

impl<T: Serialize> Serialize for Checked<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Checked::Valid(ref item) => item.serialize(serializer),
            Checked::Invalid => Err(ser::Error::custom("invalid item")),
        }
    }
}

impl<T: Clone> Clone for Checked<T> {
    fn clone(&self) -> Self {
        match *self {
            Checked::Valid(ref item) => Checked::Valid(item.clone()),
            Checked::Invalid => Checked::Invalid,
        }
    }
}

impl<T: Copy> Copy for Checked<T> {}

impl<T: Default> Default for Checked<T> {
    fn default() -> Self {
        Checked::Valid(T::default())
    }
}
