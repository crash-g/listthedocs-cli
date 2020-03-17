use serde_derive::{Deserialize, Serialize};

pub mod get {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Version {
        name: String,
        url: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Project {
        title: String,
        code: String,
        description: String,
        logo: Option<String>,
        versions: Vec<Version>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApiKey {
        created_at: String,
        is_valid: bool,
        key: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Role {
        role_name: String,
        project_code: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        name: String,
        is_admin: bool,
        created_at: String,
        api_keys: Vec<ApiKey>,
        roles: Vec<Role>,
    }
}

pub mod post {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Project {
        pub title: String,
        pub description: String,
        pub logo: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub name: String,
        pub is_admin: Option<bool>,
    }
}

pub mod patch {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Project {
        #[serde(skip_serializing)]
        pub code: String,
        pub description: Option<String>,
        pub logo: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectRole {
        pub role_name: String,
        pub project_code: String,
    }
}
