use crate::client;
use crate::errors::*;
use reqwest::Method;

/// A role
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub memo: String,
}

#[cfg(test)]
mod tests {
    use crate::role::*;
    use serde_json;

    fn role_example() -> Role {
        Role {
            name: "FooRole".to_string(),
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
    /// See https://mackerel.io/api-docs/entry/services#rolelist.
    pub fn list_roles(&self, service_name: String) -> Result<Vec<Role>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/roles", service_name),
            vec![],
            client::empty_body(),
            |res: ListRolesResponse| res.roles,
        )
    }

    /// Creates a new role.
    ///
    /// See https://mackerel.io/api-docs/entry/services#rolecreate.
    pub fn create_role(&self, service_name: String, role: Role) -> Result<Role> {
        self.request(
            Method::POST,
            format!("/api/v0/services/{}/roles", service_name),
            vec![],
            Some(role),
            |role| role,
        )
    }

    /// Deletes a role.
    ///
    /// See https://mackerel.io/api-docs/entry/services#roledelete.
    pub fn delete_role(&self, service_name: String, role_name: String) -> Result<Role> {
        self.request(
            Method::DELETE,
            format!("/api/v0/services/{}/roles/{}", service_name, role_name),
            vec![],
            client::empty_body(),
            |role| role,
        )
    }
}
