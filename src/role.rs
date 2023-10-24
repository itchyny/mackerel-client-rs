use crate::client;
use crate::error::*;
use crate::service::ServiceName;
use fixedstr::str64;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

/// A role
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub name: RoleName,
    pub memo: String,
}

use std::marker::PhantomData;
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct RoleName {
    role_name: str64,
    phantom: PhantomData<Role>,
}

impl RoleName {
    pub fn new(role_name: str64) -> Self {
        Self {
            role_name,
            phantom: PhantomData,
        }
    }
}

impl From<&str> for RoleName {
    fn from(role_name: &str) -> Self {
        Self::new(role_name.into())
    }
}

impl From<String> for RoleName {
    fn from(role_name: String) -> Self {
        Self::new(role_name.into())
    }
}

impl Into<String> for RoleName {
    fn into(self: Self) -> String {
        self.role_name.to_string()
    }
}

impl std::ops::Deref for RoleName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.role_name
    }
}

impl std::fmt::Display for RoleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.role_name.fmt(f)
    }
}

impl std::fmt::Debug for RoleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\"")?;
        self.role_name.fmt(f)?;
        f.write_str("\"")
    }
}

use serde::ser::{Serialize, Serializer};
impl Serialize for RoleName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.role_name.serialize(serializer)
    }
}

use serde::de::{Deserialize, Deserializer};
impl<'de> Deserialize<'de> for RoleName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(str64::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use crate::role::*;
    use serde_json::json;

    fn role_example() -> Role {
        Role {
            name: "FooRole".into(),
            memo: "role memo".to_string(),
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "name": "FooRole",
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
