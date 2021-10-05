use serde_derive::{Deserialize, Serialize};
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Entity<T> {
    pub id: Id<T>,
    #[serde(flatten)]
    pub value: T,
}

use std::marker::PhantomData;
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Id<T> {
    id: String,
    phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: String) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}

impl<T> From<&str> for Id<T> {
    fn from(id: &str) -> Self {
        Self {
            id: id.into(),
            phantom: PhantomData,
        }
    }
}

impl<T> From<String> for Id<T> {
    fn from(id: String) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}

impl<T> Into<String> for Id<T> {
    fn into(self: Self) -> String {
        self.id
    }
}

use std::fmt;
impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)
    }
}

use serde::ser::{Serialize, Serializer};
impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.serialize(serializer)
    }
}

use serde::de::{Deserialize, Deserializer};
impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            id: String::deserialize(deserializer)?,
            phantom: PhantomData,
        })
    }
}
