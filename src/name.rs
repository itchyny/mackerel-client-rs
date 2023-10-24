use fixedstr::str64;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::marker::PhantomData;

#[derive(PartialEq, Eq, SerializeDisplay, DeserializeFromStr)]
pub struct Name<T>(str64, PhantomData<T>);

impl<T> Name<T> {
    fn new(name: str64) -> Self {
        Self(name, PhantomData)
    }
}

impl<T> Copy for Name<T> {}

impl<T> Clone for Name<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParseNameError(String);

impl std::fmt::Display for ParseNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse name: {}", self.0)
    }
}

impl<T> std::str::FromStr for Name<T> {
    type Err = ParseNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(2..=63).contains(&s.len())
            || !s
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(ParseNameError(s.to_string()));
        }
        Ok(Self::new(s.into()))
    }
}

impl<T> From<&str> for Name<T> {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl<T> From<String> for Name<T> {
    fn from(s: String) -> Self {
        s.parse().unwrap()
    }
}

impl<T> Into<String> for Name<T> {
    fn into(self: Self) -> String {
        self.0.to_string()
    }
}

impl<T> std::ops::Deref for Name<T> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::fmt::Display for Name<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> std::fmt::Debug for Name<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl<T> std::hash::Hash for Name<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
