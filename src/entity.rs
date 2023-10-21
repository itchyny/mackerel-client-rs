use fixedstr::str16;
use serde_derive::{Deserialize, Serialize};

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

use std::marker::PhantomData;
#[derive(PartialEq, Eq, Hash)]
pub struct Id<T> {
    id: str16,
    phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: str16) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> From<&str> for Id<T> {
    fn from(id: &str) -> Self {
        Self::new(id.into())
    }
}

impl<T> From<String> for Id<T> {
    fn from(id: String) -> Self {
        Self::new(id.into())
    }
}

impl<T> Into<String> for Id<T> {
    fn into(self: Self) -> String {
        self.id.to_string()
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
        f.write_str("\"")?;
        self.id.fmt(f)?;
        f.write_str("\"")
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
        Ok(Self::new(str16::deserialize(deserializer)?))
    }
}
