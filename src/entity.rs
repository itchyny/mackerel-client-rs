use fixedstr::str16;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::marker::PhantomData;
use typed_builder::TypedBuilder;

/// An entity represents a value identified by the id.
/// You can access the value fields without `.value`.
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct Entity<T> {
    pub id: Id<T>,
    #[serde(flatten)]
    pub value: T,
}

impl<T> Entity<T> {
    /// Creates a new [`Entity`].
    pub fn new(id: impl Into<Id<T>>, value: T) -> Entity<T> {
        Entity {
            id: id.into(),
            value,
        }
    }
}

impl<T> std::ops::Deref for Entity<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// An id represents a unique id of the type `T`.
#[derive(DeserializeFromStr, SerializeDisplay)]
pub struct Id<T>(str16, PhantomData<T>);

impl<T> Id<T> {
    /// Creates a new [`Id`].
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

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

impl<T> Eq for Id<T> {}

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

#[doc(hidden)]
impl<T> std::ops::Deref for Id<T> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn entity() {
        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        struct Value {
            x: i64,
        }
        let entity = Entity::<Value>::new("id0", Value { x: 1 });
        assert_eq!(
            entity,
            Entity::<Value>::builder()
                .id("id0")
                .value(Value { x: 1 })
                .build(),
        );
        assert_eq!(entity.x, 1);
        let json = json!({"id": "id0", "x": 1});
        assert_eq!(serde_json::to_value(&entity).unwrap(), json);
        assert_eq!(entity, serde_json::from_value(json).unwrap());
    }

    #[test]
    fn id() {
        struct Value {
            #[allow(dead_code)]
            x: i64,
        }
        let id = Id::<Value>::from("id0");
        assert_eq!(id, "id0".into());
        assert_eq!(HashMap::from([(id, 1)]).get(&id), Some(&1));
        assert_eq!(HashSet::from([id]).iter().next(), Some(&id));
        assert_eq!(id.to_string(), "id0");
        assert_eq!(format!("{}", id), "id0");
        assert_eq!(format!("{:?}", id), r#""id0""#);
        assert_eq!(id, "id0".parse().unwrap());
        assert_eq!(id, serde_json::from_value(json!("id0")).unwrap());
        assert_eq!(serde_json::to_value(id).unwrap(), json!("id0"));
    }
}
