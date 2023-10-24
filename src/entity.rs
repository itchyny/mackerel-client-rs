use fixedstr::str16;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::marker::PhantomData;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Entity<T> {
    pub id: Id<T>,
    #[serde(flatten)]
    pub value: T,
}

impl<T> std::ops::Deref for Entity<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct Id<T>(str16, PhantomData<T>);

impl<T> Id<T> {
    pub fn new(id: str16) -> Self {
        Self(id, PhantomData)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParseIdError(String);

impl std::fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse id: {}", self.0)
    }
}

impl<T> std::str::FromStr for Id<T> {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(1..16).contains(&s.len()) {
            return Err(ParseIdError(s.to_string()));
        }
        Ok(Self::new(s.into()))
    }
}

impl<T> From<&str> for Id<T> {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl<T> From<String> for Id<T> {
    fn from(s: String) -> Self {
        s.parse().unwrap()
    }
}

impl<T> Into<String> for Id<T> {
    fn into(self: Self) -> String {
        self.0.to_string()
    }
}

impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
