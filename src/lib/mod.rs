use serde_derive::Deserialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

mod client;
mod command_line;
mod entities;
mod error;

use client::ListTheDocs;
use command_line::{
    opt_from_args, Command, ProjectCommand, ProjectRole, RoleCommand, UserCommand, VersionCommand,
};
use entities::{patch, post};
pub use error::{Error, Result};

pub fn execute_command() -> Result<String> {
    let opt = opt_from_args();
    let list_the_docs = make_client(opt.url.clone(), opt.api_key.clone(), &opt.config)?;
    let executor = CommandExecutor {
        list_the_docs: list_the_docs,
        pretty_print: opt.pretty_print,
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
        Command::Version { version_command } => unimplemented!(),
        Command::User { user_command } => match user_command {
            UserCommand::Add {
                name,
                is_admin,
                file_path,
            } => executor.add_user(name, is_admin, file_path),
            UserCommand::Get { name } => executor.get_user(name),
            UserCommand::GetAll => executor.get_all_users(),
        },
        Command::Role { role_command } => match role_command {
            RoleCommand::Add {
                user_name,
                project_role,
                file_path,
            } => executor.add_roles(user_name, project_role, file_path),
        },
    }
}

pub struct CommandExecutor {
    list_the_docs: ListTheDocs,
    pretty_print: bool,
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
                title: title.ok_or(Error::InputError(
                    "Missing compulsory 'title' field".to_owned(),
                ))?,
                description: description.ok_or(Error::InputError(
                    "Missing compulsory 'description' field".to_owned(),
                ))?,
                logo,
            },
        };
        let added_project = self.list_the_docs.add_project(&project)?;
        Ok(to_string(&added_project, self.pretty_print))
    }

    fn get_project(&self, code: String) -> Result<String> {
        match self.list_the_docs.get_project(&code)? {
            Some(project) => Ok(to_string(&project, self.pretty_print)),
            None => Ok(format!("Project with code '{}' not found", code)),
        }
    }

    fn get_all_projects(&self) -> Result<String> {
        let projects = self.list_the_docs.get_all_projects()?;
        Ok(to_string(&projects, self.pretty_print))
    }

    fn update_project(
        &self,
        code: Option<String>,
        description: Option<String>,
        logo: Option<String>,
        file_path: Option<PathBuf>,
    ) -> Result<String> {
        let project = match file_path {
            Some(path) => from_file(path)?,
            None => patch::Project {
                code: code.ok_or(Error::InputError(
                    "Missing compulsory 'code' field".to_owned(),
                ))?,
                description,
                logo,
            },
        };
        let code = project.code.clone();

        match self.list_the_docs.update_project(&project)? {
            Some(project) => Ok(to_string(&project, self.pretty_print)),
            None => Err(Error::InputError(format!(
                "Project with code '{}' not found",
                &code
            ))),
        }
    }

    fn remove_project(&self, code: String) -> Result<String> {
        self.list_the_docs.remove_project(&code).map(|_| code)
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
                name: name.ok_or(Error::InputError(
                    "Missing compulsory 'name' field".to_owned(),
                ))?,
                is_admin,
            },
        };
        let added_user = self.list_the_docs.add_user(&user)?;
        Ok(to_string(&added_user, self.pretty_print))
    }

    fn get_user(&self, name: String) -> Result<String> {
        match self.list_the_docs.get_user(&name)? {
            Some(user) => Ok(to_string(&user, self.pretty_print)),
            None => Ok(format!("User with name '{}' not found", name)),
        }
    }

    fn get_all_users(&self) -> Result<String> {
        let users = self.list_the_docs.get_all_users()?;
        Ok(to_string(&users, self.pretty_print))
    }

    fn add_roles(
        &self,
        user_name: String,
        project_role: Option<Vec<ProjectRole>>,
        file_path: Option<PathBuf>,
    ) -> Result<String> {
        let roles: Vec<_> = match file_path {
            Some(path) => from_file(path)?,
            None => project_role
                .ok_or(Error::InputError("Missing project roles to add".to_owned()))?
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

        match self.list_the_docs.add_roles(&user_name, &roles)? {
            Some(_) => Ok("".to_owned()),
            None => Err(Error::InputError(format!(
                "User with name '{}' not found",
                &user_name
            ))),
        }
    }
}

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

fn to_string<T>(t: &T, pretty_print: bool) -> String
where
    T: serde::Serialize + Debug,
{
    if pretty_print {
        format!("{:#?}", t)
    } else {
        serde_json::to_string(&t).unwrap_or_else(|e| panic!("serde serialization failed: {}", e))
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
            url.ok_or(Error::InputError(
                "Missing compulsory url parameter".to_owned(),
            ))?,
            api_key,
        )),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_project_command() {
//         let code = "test".to_owned();
//         let list_the_docs = ListTheDocs::new("http://localhost:5000", Some("ROOT-API-KEY"));
//         remove_project(&list_the_docs, code).unwrap();
//         todo!()
//     }
// }
