use crate::client;
use crate::error::*;
use crate::name::Name;
use crate::service::ServiceName;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// A role
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub name: RoleName,
    pub memo: String,
}

/// A role name
/// ```rust
/// use mackerel_client::role::RoleName;
///
/// let role_name = RoleName::from("ExampleRole");
/// ```
pub type RoleName = Name<Role>;

/// A role full name
/// ```rust
/// use mackerel_client::role::RoleFullname;
///
/// let role_fullname = RoleFullname::from("ExampleService:ExampleRole");
/// ```
#[derive(PartialEq, Eq, Copy, Clone, Hash, SerializeDisplay, DeserializeFromStr)]
pub struct RoleFullname {
    pub service_name: ServiceName,
    pub role_name: RoleName,
}

impl RoleFullname {
    pub fn new(service_name: ServiceName, role_name: RoleName) -> Self {
        Self {
            service_name,
            role_name,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParseRoleFullnameError(String);

impl std::fmt::Display for ParseRoleFullnameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse role fullname: {}", self.0)
    }
}

impl std::str::FromStr for RoleFullname {
    type Err = ParseRoleFullnameError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (service_name_str, role_name_str) = s
            .split_once(':')
            .ok_or(ParseRoleFullnameError(s.to_string()))?;
        Ok(RoleFullname::new(
            service_name_str
                .parse()
                .map_err(|_| ParseRoleFullnameError(s.to_string()))?,
            role_name_str
                .trim_start()
                .parse()
                .map_err(|_| ParseRoleFullnameError(s.to_string()))?,
        ))
    }
}

impl From<&str> for RoleFullname {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl From<String> for RoleFullname {
    fn from(s: String) -> Self {
        s.parse().unwrap()
    }
}

impl Into<String> for RoleFullname {
    fn into(self: Self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for RoleFullname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.service_name, self.role_name)
    }
}

impl std::fmt::Debug for RoleFullname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::role::*;
    use rstest::rstest;
    use serde_json::json;

    fn role_example() -> Role {
        Role {
            name: "ExampleRole".into(),
            memo: "role memo".to_string(),
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "name": "ExampleRole",
            "memo": "role memo"
        })
    }

    #[test]
    fn serialize_role() {
        assert_eq!(
            json_example(),
            serde_json::to_value(&role_example()).unwrap()
        );
    }

    #[test]
    fn deserialize_role() {
        assert_eq!(
            role_example(),
            serde_json::from_value(json_example()).unwrap()
        );
    }

    #[rstest]
    #[case("ExampleService:ExampleRole".into(), "ExampleService:ExampleRole")]
    #[case("s0:r0".into(), "s0:r0")]
    #[case("service-0: role_1".into(), "service-0:role_1")]
    fn test_role_fullname(#[case] role_fullname: RoleFullname, #[case] role_fullname_str: &str) {
        assert_eq!(role_fullname.to_string(), role_fullname_str);
        assert_eq!(role_fullname, role_fullname_str.parse().unwrap());
        assert_eq!(
            role_fullname,
            serde_json::from_value(role_fullname_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(role_fullname).unwrap(),
            role_fullname_str
        );
    }

    #[rstest]
    #[case("")]
    #[case(":")]
    #[case("ExampleService:")]
    #[case(":ExampleRole")]
    #[case("ExampleService:ExampleRole:")]
    #[case(":ExampleService:ExampleRole")]
    #[case("s:role")]
    #[case("service:r")]
    #[case(" service:role")]
    #[case("service:role ")]
    #[case("service#0:role#0")]
    fn test_role_fullname_error(#[case] role_fullname_str: &str) {
        assert_eq!(
            Err(ParseRoleFullnameError(role_fullname_str.to_string())),
            role_fullname_str.parse::<RoleFullname>()
        );
    }
}

#[derive(Deserialize)]
struct ListRolesResponse {
    roles: Vec<Role>,
}

impl client::Client {
    /// Fetches the roles in the specified service.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#rolelist>.
    pub async fn list_roles(&self, service_name: ServiceName) -> Result<Vec<Role>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/roles", service_name),
            vec![],
            client::empty_body(),
            |res: ListRolesResponse| res.roles,
        )
        .await
    }

    /// Creates a new role.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#rolecreate>.
    pub async fn create_role(&self, service_name: ServiceName, role: Role) -> Result<Role> {
        self.request(
            Method::POST,
            format!("/api/v0/services/{}/roles", service_name),
            vec![],
            Some(role),
            |role| role,
        )
        .await
    }

    /// Deletes a role.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#roledelete>.
    pub async fn delete_role(
        &self,
        service_name: ServiceName,
        role_name: RoleName,
    ) -> Result<Role> {
        self.request(
            Method::DELETE,
            format!("/api/v0/services/{}/roles/{}", service_name, role_name),
            vec![],
            client::empty_body(),
            |role| role,
        )
        .await
    }
}
