use std::{fmt, marker};

use crate::{
    Path, Root,
    gltf::Get,
    validation::{self, Validate},
};

pub struct StringIndex<T>(String, marker::PhantomData<fn() -> T>);

impl<T> StringIndex<T> {
    /// Creates a new `Index` representing an offset into an array containing `T`.
    pub fn new(value: String) -> Self {
        StringIndex(value, std::marker::PhantomData)
    }

    /// Returns the internal offset value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl<T> serde::Serialize for StringIndex<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.value())
    }
}

impl<'de, T> serde::Deserialize<'de> for StringIndex<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<T>(marker::PhantomData<T>);
        impl<'de, T> serde::de::Visitor<'de> for Visitor<T> {
            type Value = StringIndex<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("index into child of root")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StringIndex::new(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StringIndex::new(v))
            }
        }
        deserializer.deserialize_str(Visitor::<T>(marker::PhantomData))
    }
}

impl<T> Ord for StringIndex<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T> PartialOrd for StringIndex<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for StringIndex<T> {}
impl<T> PartialEq for StringIndex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> std::hash::Hash for StringIndex<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> fmt::Debug for StringIndex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> fmt::Display for StringIndex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Clone for StringIndex<T> {
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

impl<T> Validate for StringIndex<T>
where
    Root: Get<T>,
{
    fn validate<P, R>(&self, root: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, validation::Error),
    {
        if root.get(self.clone()).is_none() {
            report(&path, validation::Error::IndexNotFound);
        }
    }
}
