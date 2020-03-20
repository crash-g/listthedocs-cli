use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Version {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub title: String,
    pub code: String,
    pub description: String,
    pub logo: Option<String>,
    pub versions: Vec<Version>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiKey {
    pub created_at: String,
    pub is_valid: bool,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Role {
    pub role_name: String,
    pub project_code: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub name: String,
    pub is_admin: bool,
    pub created_at: String,
    pub api_keys: Vec<ApiKey>,
    pub roles: Vec<Role>,
}
