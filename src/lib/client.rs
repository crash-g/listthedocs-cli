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
            "Cannot add a project if API key is not provided".to_owned(),
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
                println!("{:?}", response);
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
                println!("{:?}", response);
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
                println!("{:?}", response);
                unimplemented!()
            }
        }
    }

    pub fn update_project(&self, project: &patch::Project) -> Result<Option<get::Project>> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "Cannot update a project if API key is not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/projects/", &project.code].concat();
        let response = minreq::patch(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(project)?
            .send()?;

        match response.status_code {
            200 => Ok(response.json()?),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            _ => {
                println!("{:?}", response);
                unimplemented!()
            }
        }
    }

    pub fn remove_project(&self, code: &str) -> Result<()> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "Cannot delete a project if API key is not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, "/api/v2/projects/", code].concat();
        let response = minreq::delete(endpoint_url)
            .with_header("Api-Key", api_key)
            .send()?;

        match response.status_code {
            200 | 404 => Ok(()),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            _ => {
                println!("{:?}", response);
                unimplemented!()
            }
        }
    }

    pub fn add_user(&self, user: &post::User) -> Result<get::User> {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "Cannot add a user if API key is not provided".to_owned(),
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
                println!("{:?}", response);
                unimplemented!()
            }
        }
    }
}
