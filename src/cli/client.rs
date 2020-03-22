use minreq;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::fmt::Debug;

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
        B: Serialize + Debug,
        R: DeserializeOwned,
    {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            Error::InputError("API key is required and was not provided".to_owned())
        })?;

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
            403 => Err(Error::InputError(format!(
                "Forbidden -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
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
        R: DeserializeOwned,
    {
        let endpoint_url = &[&self.base_url, endpoint_url].concat();
        let response = if with_api_key {
            let api_key = self.api_key.as_ref().ok_or_else(|| {
                Error::InputError("API key is required and was not provided".to_owned())
            })?;
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
        B: Serialize + Debug,
        R: DeserializeOwned,
    {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            Error::InputError("API key is required and was not provided".to_owned())
        })?;

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
            403 => Err(Error::InputError(format!(
                "Forbidden -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                body
            ))),
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

    pub fn remove(&self, endpoint_url: &str, is_404_error: bool) -> Result<()> {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            Error::InputError("API key is required and was not provided".to_owned())
        })?;

        let endpoint_url = &[&self.base_url, endpoint_url].concat();
        let response = minreq::delete(endpoint_url)
            .with_header("Api-Key", api_key)
            .send()?;

        match response.status_code {
            200 => Ok(()),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            403 => Err(Error::InputError(format!(
                "Forbidden -- Response: {}",
                response.as_str()?,
            ))),
            404 if is_404_error => Err(Error::InputError(format!(
                "Not found -- Response: {}",
                response.as_str()?
            ))),
            404 => Ok(()),
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

    // For now, roles behave differently so we have specific methods for them

    pub fn add_roles(&self, user_name: &str, roles: &[patch::ProjectRole]) -> Result<()> {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            Error::InputError("API key is required and was not provided".to_owned())
        })?;

        let endpoint_url = &[&self.base_url, "/api/v2/users/", &user_name, "/roles"].concat();
        let response = minreq::patch(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(&roles)?
            .send()?;

        match response.status_code {
            200 => Ok(()),
            400 => Err(Error::InputError(format!(
                "Bad request -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            403 => Err(Error::InputError(format!(
                "Forbidden -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
            404 => Err(Error::InputError(format!(
                "Not found -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
            409 => Err(Error::InputError(format!(
                "Conflict -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
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

    pub fn remove_roles(&self, user_name: &str, roles: &[patch::ProjectRole]) -> Result<()> {
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            Error::InputError("API key is required and was not provided".to_owned())
        })?;

        let endpoint_url = &[&self.base_url, "/api/v2/users/", &user_name, "/roles"].concat();
        let response = minreq::delete(endpoint_url)
            .with_header("Api-Key", api_key)
            .with_json(&roles)?
            .send()?;

        match response.status_code {
            200 => Ok(()),
            400 => Err(Error::InputError(format!(
                "Bad request -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
            401 => Err(Error::InputError("Authorization failed".to_owned())),
            403 => Err(Error::InputError(format!(
                "Forbidden -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
            404 => Err(Error::InputError(format!(
                "Not found -- Response: {} -- Original request: {:#?}",
                response.as_str()?,
                roles
            ))),
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
