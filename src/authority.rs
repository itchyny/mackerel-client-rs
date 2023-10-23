use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// User authority
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Authority {
    Owner,
    Manager,
    Collaborator,
    Viewer,
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Authority::Owner => write!(f, "owner"),
            Authority::Manager => write!(f, "manager"),
            Authority::Collaborator => write!(f, "collaborator"),
            Authority::Viewer => write!(f, "viewer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::authority::*;

    #[test]
    fn authorities() {
        let test_cases = [
            (Authority::Owner, "owner"),
            (Authority::Manager, "manager"),
            (Authority::Collaborator, "collaborator"),
            (Authority::Viewer, "viewer"),
        ];
        for &(authority, authority_str) in &test_cases {
            let str_value = serde_json::Value::String(authority_str.to_string());
            assert_eq!(
                authority,
                serde_json::from_value(str_value.clone()).unwrap()
            );
            assert_eq!(str_value, serde_json::to_value(authority).unwrap());
            assert_eq!(str_value, format!("{}", authority).as_str());
        }
    }
}
