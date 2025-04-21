use std::collections::HashMap;

use reqwest::{Error, Response};


pub enum RequestType {
    GET,
    POST,
    PUT
}

#[derive(Debug)]

pub enum HttpError {
    MissingRequestType,
    RequestFailed(Error),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for HttpError {}

impl From<Error> for HttpError {
    fn from(err: Error) -> Self {
        HttpError::RequestFailed(err)
    }
}

// Add missing fields
pub struct Http {
    url: String,
    request_type: Option<RequestType>,
    params: Option<HashMap<String, String>>,
    headers: Option<HashMap<String, String>>,
}

impl Http {
    pub fn new(
        url: String,
        request_type: Option<RequestType>,
        params: Option<HashMap<String, String>>,
        headers: Option<HashMap<String, String>>
    ) -> Self {
        Http {
            url,
            request_type: Some(request_type.unwrap_or(RequestType::GET)), // Use GET by Deafault
            params: Some(params.unwrap_or_default()),
            headers: Some(headers.unwrap_or_default()),
        }
    }

    pub async fn make_request(&self) -> Result<Response, HttpError> {
        let client = reqwest::Client::new();
        match self.request_type {
            Some(RequestType::GET) => {
                let res = client.get(&self.url).send().await?;
                Ok(res)
            },
            Some(RequestType::POST) => {
                let res = client.post(&self.url).send().await?;
                Ok(res)
            },
            Some(_) => {
                Err(HttpError::MissingRequestType)
            }
            None => {
                Err(HttpError::MissingRequestType)
            }
        }
    }
}
