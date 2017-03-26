/// Monitor types
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum MonitorType {
    #[serde(rename = "connectivity")]
    Connectivity,
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "expression")]
    Expression,
}
