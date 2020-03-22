use serde::Deserialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

mod client;
mod command_line;
mod entities;
mod error;

pub use command_line::options_from_args;
pub use error::{Error, Result};

use client::ListTheDocs;
use command_line::{
    Command, Opt, ProjectCommand, ProjectRole, RoleCommand, UserCommand, VersionCommand,
};
use entities::{get, patch, post};

pub fn execute_command(opt: Opt) -> Result<String> {
    let list_the_docs = make_client(opt.url.clone(), opt.api_key.clone(), &opt.config)?;
    let executor = CommandExecutor {
        list_the_docs,
        json_output: opt.json,
    };

    match opt.cmd {
        Command::Project { project_command } => match project_command {
            ProjectCommand::Add {
                title,
                description,
                logo,
                file_path,
            } => executor.add_project(title, description, logo, file_path),
            ProjectCommand::Get { code } => executor.get_project(code),
            ProjectCommand::GetAll => executor.get_all_projects(),
            ProjectCommand::Update {
                code,
                description,
                logo,
                file_path,
            } => executor.update_project(code, description, logo, file_path),
            ProjectCommand::Remove { code } => executor.remove_project(code),
        },
        Command::Version { version_command } => match version_command {
            VersionCommand::Add {
                code,
                version,
                url,
                file_path,
            } => executor.add_version(code, version, url, file_path),
            VersionCommand::Update { code, version, url } => {
                executor.update_version(code, version, url)
            }
            VersionCommand::Remove { code, version } => executor.remove_version(code, version),
        },
        Command::User { user_command } => match user_command {
            UserCommand::Add {
                name,
                is_admin,
                file_path,
            } => executor.add_user(name, is_admin, file_path),
            UserCommand::Get { name } => executor.get_user(name),
            UserCommand::GetAll => executor.get_all_users(),
            UserCommand::Remove { name } => executor.remove_user(name),
        },
        Command::Role { role_command } => match role_command {
            RoleCommand::Add {
                user_name,
                project_role,
                file_path,
            } => executor.add_roles(user_name, project_role, file_path),
            RoleCommand::Remove {
                user_name,
                project_role,
                file_path,
            } => executor.remove_roles(user_name, project_role, file_path),
            RoleCommand::Get { user_name } => executor.get_roles(user_name),
        },
    }
}

struct CommandExecutor {
    list_the_docs: ListTheDocs,
    json_output: bool,
}

impl CommandExecutor {
    fn add_project(
        &self,
        title: Option<String>,
        description: Option<String>,
        logo: Option<String>,
        file_path: Option<PathBuf>,
    ) -> Result<String> {
        let project = match file_path {
            Some(path) => from_file(path)?,
            None => post::Project {
                title: title.ok_or_else(|| {
                    Error::InputError("Missing compulsory 'title' field".to_owned())
                })?,
                description: description.ok_or_else(|| {
                    Error::InputError("Missing compulsory 'description' field".to_owned())
                })?,
                logo,
            },
        };
        let added_project: get::Project = self
            .list_the_docs
            .post("/api/v2/projects", &project)?
            .expect("404 can never be received when adding a project");
        Ok(to_string(&added_project, self.json_output))
    }

    fn get_project(&self, code: String) -> Result<String> {
        let endpoint_url = &["/api/v2/projects/", &code].concat();
        let project: Option<get::Project> = self.list_the_docs.get(endpoint_url, false)?;
        match project {
            Some(project) => Ok(to_string(&project, self.json_output)),
            None => Ok(format!("Project with code '{}' not found", code)),
        }
    }

    fn get_all_projects(&self) -> Result<String> {
        let projects: Vec<get::Project> = self
            .list_the_docs
            .get("/api/v2/projects", false)?
            .expect("404 can never be received when getting all projects");
        Ok(to_string(&projects, self.json_output))
    }

    fn update_project(
        &self,
        code: String,
        description: Option<String>,
        logo: Option<String>,
        file_path: Option<PathBuf>,
    ) -> Result<String> {
        let project = match file_path {
            Some(path) => from_file(path)?,
            None => patch::Project { description, logo },
        };

        let endpoint_url = &["/api/v2/projects/", &code].concat();
        let project: Option<get::Project> = self.list_the_docs.patch(&endpoint_url, &project)?;
        match project {
            Some(project) => Ok(to_string(&project, self.json_output)),
            None => Err(Error::InputError(format!(
                "Project with code '{}' not found",
                &code
            ))),
        }
    }

    fn remove_project(&self, code: String) -> Result<String> {
        let endpoint_url = &["/api/v2/projects/", &code].concat();
        self.list_the_docs
            .remove(&endpoint_url, false)
            .map(|_| code)
    }

    fn add_version(
        &self,
        code: String,
        version: Option<String>,
        url: Option<String>,
        file_path: Option<PathBuf>,
    ) -> Result<String> {
        let version = match file_path {
            Some(path) => from_file(path)?,
            None => post::Version {
                name: version.ok_or_else(|| {
                    Error::InputError("Missing compulsory 'version' field".to_owned())
                })?,
                url: url.ok_or_else(|| {
                    Error::InputError("Missing compulsory 'url' field".to_owned())
                })?,
            },
        };

        let endpoint_url = &["/api/v2/projects/", &code, "/versions"].concat();
        let project: Option<get::Project> = self.list_the_docs.post(&endpoint_url, &version)?;
        match project {
            Some(project) => Ok(to_string(&project, self.json_output)),
            None => Err(Error::InputError(format!(
                "Project with code '{}' not found",
                &code
            ))),
        }
    }

    fn update_version(&self, code: String, version: String, url: String) -> Result<String> {
        let url = patch::Version { url };
        let endpoint_url = &["/api/v2/projects/", &code, "/versions/", &version].concat();
        let project: Option<get::Project> = self.list_the_docs.patch(&endpoint_url, &url)?;
        match project {
            Some(project) => Ok(to_string(&project, self.json_output)),
            None => Err(Error::InputError(format!(
                "Project with code '{}' or version '{}' not found",
                &code, &version
            ))),
        }
    }

    fn remove_version(&self, code: String, version: String) -> Result<String> {
        let endpoint_url = &["/api/v2/projects/", &code, "/versions/", &version].concat();
        self.list_the_docs
            .remove(&endpoint_url, true)
            .map(|_| version)
    }

    fn add_user(
        &self,
        name: Option<String>,
        is_admin: Option<bool>,
        file_path: Option<PathBuf>,
    ) -> Result<String> {
        let user = match file_path {
            Some(path) => from_file(path)?,
            None => post::User {
                name: name.ok_or_else(|| {
                    Error::InputError("Missing compulsory 'name' field".to_owned())
                })?,
                is_admin,
            },
        };
        let added_user: get::User = self
            .list_the_docs
            .post("/api/v2/users", &user)?
            .expect("404 can never be received when adding a user");
        Ok(to_string(&added_user, self.json_output))
    }

    fn get_user(&self, name: String) -> Result<String> {
        let endpoint_url = &["/api/v2/users/", &name].concat();
        let user: Option<get::User> = self.list_the_docs.get(&endpoint_url, true)?;
        match user {
            Some(user) => Ok(to_string(&user, self.json_output)),
            None => Ok(format!("User with name '{}' not found", name)),
        }
    }

    fn get_all_users(&self) -> Result<String> {
        let users: Vec<get::User> = self
            .list_the_docs
            .get("/api/v2/users", true)?
            .expect("404 can never be received when getting all users");
        Ok(to_string(&users, self.json_output))
    }

    fn remove_user(&self, name: String) -> Result<String> {
        let endpoint_url = &["/api/v2/users/", &name].concat();
        self.list_the_docs
            .remove(&endpoint_url, false)
            .map(|_| name)
    }

    fn add_roles(
        &self,
        user_name: String,
        project_role: Vec<ProjectRole>,
        file_path: Option<PathBuf>,
    ) -> Result<String> {
        let roles: Vec<_> = match file_path {
            Some(path) => from_file(path)?,
            None => project_role
                .into_iter()
                .map(
                    |ProjectRole {
                         role_name,
                         project_code,
                     }| patch::ProjectRole {
                        role_name,
                        project_code,
                    },
                )
                .collect(),
        };

        self.list_the_docs.add_roles(&user_name, &roles)?;
        Ok("".to_owned())
    }

    fn remove_roles(
        &self,
        user_name: String,
        project_role: Vec<ProjectRole>,
        file_path: Option<PathBuf>,
    ) -> Result<String> {
        let roles: Vec<_> = match file_path {
            Some(path) => from_file(path)?,
            None => project_role
                .into_iter()
                .map(
                    |ProjectRole {
                         role_name,
                         project_code,
                     }| patch::ProjectRole {
                        role_name,
                        project_code,
                    },
                )
                .collect(),
        };

        self.list_the_docs.remove_roles(&user_name, &roles)?;
        Ok("".to_owned())
    }

    fn get_roles(&self, user_name: String) -> Result<String> {
        let endpoint_url = &["/api/v2/users/", &user_name, "/roles"].concat();
        let roles: Option<Vec<get::Role>> = self.list_the_docs.get(&endpoint_url, true)?;
        match roles {
            Some(roles) => Ok(to_string(&roles, self.json_output)),
            None => Ok(format!("User with name '{}' not found", user_name)),
        }
    }
}

//// Utility functions /////

fn from_file<P, R>(path: P) -> Result<R>
where
    P: AsRef<Path>,
    R: serde::de::DeserializeOwned,
{
    let file =
        File::open(path).map_err(|e| Error::InputError(format!("Cannot open file: {}", e)))?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .map_err(|e| Error::InputError(format!("Invalid file content: {}", e)))
}

fn to_string<T>(t: &T, json_output: bool) -> String
where
    T: serde::Serialize + Debug,
{
    if json_output {
        serde_json::to_string(&t).unwrap_or_else(|e| panic!("serde serialization failed: {}", e))
    } else {
        format!("{:#?}", t)
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    url: String,
    api_key: Option<String>,
}

fn make_client<P>(
    url: Option<String>,
    api_key: Option<String>,
    config: &Option<P>,
) -> Result<ListTheDocs>
where
    P: AsRef<Path>,
{
    match config {
        Some(path) => {
            let config: Config = from_file(path)?;
            Ok(ListTheDocs::new(config.url, config.api_key))
        }
        None => Ok(ListTheDocs::new(
            url.ok_or_else(|| Error::InputError("Missing compulsory url parameter".to_owned()))?,
            api_key,
        )),
    }
}
