use minreq;

use super::entities::patch;
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

    pub fn post<B, R>(&self, endpoint_url: &str, body: &B) -> Result<Option<R>>
    where
        B: serde::ser::Serialize + std::fmt::Debug,
        R: serde::de::DeserializeOwned,
    {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, endpoint_url].concat();
        let response = minreq::post(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(body)?
            .send()?;

        match response.status_code {
            201 => Ok(Some(response.json()?)),
            400 => Err(Error::InputError(format!(
                "Bad request -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            409 => Err(Error::InputError(format!(
                "Conflict -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            x if x >= 500 && x < 600 => Err(Error::InputError(format!(
                "The server returned an error -- Response {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn get<R>(&self, endpoint_url: &str, with_api_key: bool) -> Result<Option<R>>
    where
        R: serde::de::DeserializeOwned,
    {
        let response = if with_api_key {
            let api_key = self.api_key.as_ref().ok_or(Error::InputError(
                "API key is required and was not provided".to_owned(),
            ))?;
            minreq::get(endpoint_url)
                .with_header("Api-Key", api_key)
                .send()?
        } else {
            minreq::get(endpoint_url).send()?
        };

        match response.status_code {
            200 => Ok(Some(response.json()?)),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            x if x >= 500 && x < 600 => Err(Error::InputError(format!(
                "The server returned an error -- Response {}",
                response.as_str()?,
            ))),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    pub fn patch<B, R>(&self, endpoint_url: &str, body: &B) -> Result<Option<R>>
    where
        B: serde::ser::Serialize + std::fmt::Debug,
        R: serde::de::DeserializeOwned,
    {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, endpoint_url].concat();
        let response = minreq::patch(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(body)?
            .send()?;

        match response.status_code {
            200 => Ok(response.json()?),
            400 => Err(Error::InputError(format!(
                "Bad request -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            409 => Err(Error::InputError(format!(
                "Conflict -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            x if x >= 500 && x < 600 => Err(Error::InputError(format!(
                "The server returned an error -- Response {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    // This is a workaround because the service API does not return a JSON body in some cases
    pub fn patch_without_response<B>(&self, endpoint_url: &str, body: &B) -> Result<Option<()>>
    where
        B: serde::ser::Serialize + std::fmt::Debug,
    {
        let api_key = self.api_key.as_ref().ok_or(Error::InputError(
            "API key is required and was not provided".to_owned(),
        ))?;

        let endpoint_url = &[&self.base_url, endpoint_url].concat();
        let response = minreq::patch(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(body)?
            .send()?;

        match response.status_code {
            200 => Ok(Some(())),
            400 => Err(Error::InputError(format!(
                "Bad request -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            409 => Err(Error::InputError(format!(
                "Conflict -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            x if x >= 500 && x < 600 => Err(Error::InputError(format!(
                "The server returned an error -- Response {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }

    ////// These methods both use delete but in different ways, so for now we ////////
    ////// just keep them both.                                               ////////

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
            x if x >= 500 && x < 600 => Err(Error::InputError(format!(
                "The server returned an error -- Response {}",
                response.as_str()?,
            ))),
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
            400 => Err(Error::InputError(format!(
                "Bad request -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            404 => Ok(None),
            x if x >= 500 && x < 600 => Err(Error::InputError(format!(
                "The server returned an error -- Response {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
            _ => {
                println!("{:?}", response.as_str());
                unimplemented!()
            }
        }
    }
}
