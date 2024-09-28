/*
    Appellation: account <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::HumanName;

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct AccountCnf {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub name: HumanName,
}
