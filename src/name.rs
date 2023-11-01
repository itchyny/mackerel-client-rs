use fixedstr::str64;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::marker::PhantomData;

#[derive(SerializeDisplay, DeserializeFromStr)]
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

impl<T> PartialEq for Name<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

impl<T> Eq for Name<T> {}

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
                .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'))
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

impl<T> From<Name<T>> for String {
    fn from(val: Name<T>) -> Self {
        val.0.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::{HashMap, HashSet};

    struct Value {
        #[allow(dead_code)]
        x: String,
    }

    #[test]
    fn name() {
        let name = Name::<Value>::from("ExampleName");
        assert_eq!(name, "ExampleName".into());
        assert_eq!(HashMap::from([(name, 1)]).get(&name), Some(&1));
        assert_eq!(HashSet::from([name]).iter().next(), Some(&name));
        assert_eq!(name.to_string(), "ExampleName");
        assert_eq!(format!("{}", name), "ExampleName");
        assert_eq!(format!("{:?}", name), r#""ExampleName""#);
        assert_eq!(name, "ExampleName".parse().unwrap());
        assert_eq!(name, serde_json::from_value(json!("ExampleName")).unwrap());
        assert_eq!(serde_json::to_value(name).unwrap(), json!("ExampleName"));
    }
}
