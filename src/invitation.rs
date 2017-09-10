use std::fmt;

/// An invitation
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub email: String,
    pub authority: Authority,
}

/// Invitation authority
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Authority {
    Manager,
    Collaborator,
    Viewer,
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Authority::Manager => write!(f, "manager"),
            Authority::Collaborator => write!(f, "collaborator"),
            Authority::Viewer => write!(f, "viewer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use invitation::*;

    fn invitation_example1() -> Invitation {
        Invitation {
            email: "example1@example.com".to_string(),
            authority: Authority::Manager,
        }
    }

    fn json_example1() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "email": "example1@example.com",
                "authority": "manager"
            }
        "##)
            .unwrap()
    }

    fn invitation_example2() -> Invitation {
        Invitation {
            email: "example2@example.com".to_string(),
            authority: Authority::Collaborator,
        }
    }

    fn json_example2() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "email": "example2@example.com",
                "authority": "collaborator"
            }
        "##)
            .unwrap()
    }

    fn invitation_example3() -> Invitation {
        Invitation {
            email: "example3@example.com".to_string(),
            authority: Authority::Viewer,
        }
    }

    fn json_example3() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "email": "example3@example.com",
                "authority": "viewer"
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_invitation() {
        assert_eq!(json_example1(), serde_json::to_value(&invitation_example1()).unwrap());
        assert_eq!(json_example2(), serde_json::to_value(&invitation_example2()).unwrap());
        assert_eq!(json_example3(), serde_json::to_value(&invitation_example3()).unwrap());
    }

    #[test]
    fn deserialize_invitation() {
        assert_eq!(invitation_example1(), serde_json::from_value(json_example1()).unwrap());
        assert_eq!(invitation_example2(), serde_json::from_value(json_example2()).unwrap());
        assert_eq!(invitation_example3(), serde_json::from_value(json_example3()).unwrap());
    }
}
