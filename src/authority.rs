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
