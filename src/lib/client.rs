use minreq;

use super::entities::{get, patch, post};
use super::error::{Error, Result};

pub struct ListTheDocs {
    base_url: String,
    api_key: Option<String>,
}

impl ListTheDocs {
    pub fn new(url: String, api_key: Option<String>) -> ListTheDocs {
        ListTheDocs {
            base_url: url,
            api_key,
        }
    }

    pub fn add_project(&self, project: &post::Project) -> Result<get::Project> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/projects"].concat();
        let response = minreq::post(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(project)?
            .send()?;

        match response.status_code {
            201 => Ok(response.json()?),
            400 => Err(Error::InputError(format!(
                "Bad request: {:?} -- Response: {}",
                project,
                response.as_str()?
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            409 => Err(Error::InputError(format!(
                "Conflict: {:?} -- Response: {}",
                project,
                response.as_str()?
            ))),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn get_project(&self, code: &str) -> Result<Option<get::Project>> {
        let endpoint_url = &[&self.base_url, "/api/v2/projects/", code].concat();
        let response = minreq::get(endpoint_url).send()?;

        match response.status_code {
            200 => Ok(Some(response.json()?)),
            404 => Ok(None),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn get_all_projects(&self) -> Result<Vec<get::Project>> {
        let endpoint_url = &[&self.base_url, "/api/v2/projects"].concat();
        let response = minreq::get(endpoint_url).send()?;

        match response.status_code {
            200 => Ok(response.json()?),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn update_project(
        &self,
        code: &str,
        project: &patch::Project,
    ) -> Result<Option<get::Project>> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/projects/", code].concat();
        let response = minreq::patch(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(project)?
            .send()?;

        match response.status_code {
            200 => Ok(response.json()?),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn remove_project(&self, code: &str) -> Result<()> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/projects/", code].concat();
        let response = minreq::delete(endpoint_url)
            .with_header("Api-Key", api_key)
            .send()?;

        match response.status_code {
            200 | 404 => Ok(()),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn add_version(&self, code: &str, version: &post::Version) -> Result<Option<get::Project>> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/projects/", code, "/versions"].concat();
        let response = minreq::post(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(version)?
            .send()?;

        match response.status_code {
            201 => Ok(Some(response.json()?)),
            400 => Err(Error::InputError(format!(
                "Bad request: {:?} -- Response: {}",
                version,
                response.as_str()?
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            409 => Err(Error::InputError(format!(
                "Conflict: {:?} -- Response: {}",
                version,
                response.as_str()?
            ))),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn add_user(&self, user: &post::User) -> Result<get::User> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/users"].concat();
        let response = minreq::post(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(user)?
            .send()?;

        match response.status_code {
            201 => Ok(response.json()?),
            400 => Err(Error::InputError(format!(
                "Bad request: {:?} -- Response: {}",
                user,
                response.as_str()?
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            409 => Err(Error::InputError(format!(
                "Conflict: {:?} -- Response: {}",
                user,
                response.as_str()?
            ))),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn get_user(&self, name: &str) -> Result<Option<get::User>> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/users/", name].concat();
        let response = minreq::get(endpoint_url)
            .with_header("Api-Key", api_key)
            .send()?;

        match response.status_code {
            200 => Ok(Some(response.json()?)),
            404 => Ok(None),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn get_all_users(&self) -> Result<Vec<get::User>> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/users"].concat();
        let response = minreq::get(endpoint_url)
            .with_header("Api-Key", api_key)
            .send()?;

        match response.status_code {
            200 => Ok(response.json()?),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn add_roles(
        &self,
        user_name: &str,
        roles: &Vec<patch::ProjectRole>,
    ) -> Result<Option<()>> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/users/", &user_name, "/roles"].concat();
        let response = minreq::patch(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(roles)?
            .send()?;

        match response.status_code {
            200 => Ok(Some(())),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn get_roles(&self, user_name: &str) -> Result<Option<Vec<get::Role>>> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/users/", user_name, "/roles"].concat();
        let response = minreq::get(endpoint_url)
            .with_header("Api-Key", api_key)
            .send()?;

        match response.status_code {
            200 => Ok(Some(response.json()?)),
            404 => Ok(None),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn remove_roles(
        &self,
        user_name: &str,
        roles: &Vec<patch::ProjectRole>,
    ) -> Result<Option<()>> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/users/", &user_name, "/roles"].concat();
        let response = minreq::delete(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(roles)?
            .send()?;

        match response.status_code {
            200 => Ok(Some(())),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }
}
