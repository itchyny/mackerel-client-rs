use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

/// User authority
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Authority {
    Owner,
    Manager,
    Collaborator,
    Viewer,
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
            assert_eq!(authority.to_string(), authority_str);
            assert_eq!(authority, authority_str.parse().unwrap());
            assert_eq!(
                authority,
                serde_json::from_value(authority_str.into()).unwrap()
            );
            assert_eq!(serde_json::to_value(authority).unwrap(), authority_str);
        }
    }
}
