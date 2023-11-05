use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::error::Result;
use crate::name::Name;
use crate::service::ServiceName;

/// A role value
#[derive(PartialEq, Eq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct Role {
    pub name: RoleName,
    #[builder(default)]
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
            .ok_or(ParseRoleFullnameError(s.to_owned()))?;
        Ok(RoleFullname::new(
            service_name_str
                .parse()
                .map_err(|_| ParseRoleFullnameError(s.to_owned()))?,
            role_name_str
                .trim_start()
                .parse()
                .map_err(|_| ParseRoleFullnameError(s.to_owned()))?,
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

impl From<RoleFullname> for String {
    fn from(val: RoleFullname) -> Self {
        val.to_string()
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
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn role_example1() -> Role {
        Role::builder()
            .name("ExampleRole1")
            .memo("role memo")
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "name": "ExampleRole1",
            "memo": "role memo"
        })
    }

    fn role_example2() -> Role {
        Role::builder().name("ExampleRole2").build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "name": "ExampleRole2",
            "memo": ""
        })
    }

    #[rstest]
    #[case(role_example1(), json_example1())]
    #[case(role_example2(), json_example2())]
    fn test_role_json(#[case] role: Role, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&role).unwrap(), json);
        assert_eq!(role, serde_json::from_value(json).unwrap());
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

impl Client {
    /// Fetches the roles in the specified service.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#rolelist>.
    pub async fn list_roles(&self, service_name: impl Into<ServiceName>) -> Result<Vec<Role>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/roles", service_name.into()),
            query_params![],
            request_body![],
            response_body! { roles: Vec<Role> },
        )
        .await
    }

    /// Creates a new role.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#rolecreate>.
    pub async fn create_role(
        &self,
        service_name: impl Into<ServiceName>,
        role: &Role,
    ) -> Result<Role> {
        self.request(
            Method::POST,
            format!("/api/v0/services/{}/roles", service_name.into()),
            query_params![],
            request_body!(role),
            response_body!(..),
        )
        .await
    }

    /// Deletes a role.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#roledelete>.
    pub async fn delete_role(
        &self,
        service_name: impl Into<ServiceName>,
        role_name: impl Into<RoleName>,
    ) -> Result<Role> {
        self.request(
            Method::DELETE,
            format!(
                "/api/v0/services/{}/roles/{}",
                service_name.into(),
                role_name.into()
            ),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::role::*;
    use crate::tests::*;

    fn value_example() -> Role {
        Role::builder()
            .name("role0")
            .memo("This is a role memo.")
            .build()
    }

    fn json_example() -> serde_json::Value {
        json!({
            "name": "role0",
            "memo": "This is a role memo.",
        })
    }

    #[async_std::test]
    async fn list_roles() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/services/service0/roles",
            response = json!({
                "roles": [json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_roles("service0").await,
            Ok(vec![value_example()]),
        );
    }

    #[async_std::test]
    async fn create_role() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/services/service0/roles",
            request = json_example(),
            response = json_example(),
        };
        assert_eq!(
            test_client!(server)
                .create_role("service0", &value_example())
                .await,
            Ok(value_example()),
        );
        assert_eq!(
            test_client!(server)
                .create_role(ServiceName::from("service0"), &value_example())
                .await,
            Ok(value_example()),
        );
    }

    #[async_std::test]
    async fn delete_role() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/services/service0/roles/role0",
            response = json_example(),
        };
        assert_eq!(
            test_client!(server).delete_role("service0", "role0").await,
            Ok(value_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_role(ServiceName::from("service0"), RoleName::from("role0"))
                .await,
            Ok(value_example()),
        );
    }
}
