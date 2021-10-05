use serde_derive::{Deserialize, Serialize};
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Entity<T> {
    pub id: String,
    #[serde(flatten)]
    pub value: T,
}
