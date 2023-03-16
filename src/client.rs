use const_format::concatcp;

use crate::{error, model::Model, APIKeysAccess};

pub const BASE_URL: &str = "https://api.openai.com/v1";

const MODELS_LIST_URL: &str = concatcp!(BASE_URL, "/models");

pub struct Client {
    api_key: String,
    organization: Option<String>,

    #[cfg(feature = "blocking")]
    blocking_client: reqwest::blocking::Client,
    async_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        Self {
            api_key: api_key.as_ref().to_string(),
            organization: None,

            #[cfg(feature = "blocking")]
            blocking_client: reqwest::blocking::Client::new(),
            async_client: reqwest::Client::new(),
        }
    }

    pub fn organization(mut self, organization: impl AsRef<str>) -> Self {
        self.organization = Some(organization.as_ref().to_string());
        self
    }

    /// (Blocking) Lists the currently available models, and provides basic information about each one such as the owner and availability.
    #[cfg(feature = "blocking")]
    pub fn list_models_blocking(&self) -> error::Result<Vec<Model>> {
        let common_headers = self.common_headers();

        let models_response = self
            .blocking_client
            .get(MODELS_LIST_URL)
            .headers(common_headers)
            .send()?;

        let json = models_response.json::<serde_json::Value>()?;
        let data = self.models_from_response_json(json)?;

        Ok(data)
    }

    /// Lists the currently available models, and provides basic information about each one such as the owner and availability.
    pub async fn list_models(&self) -> error::Result<Vec<Model>> {
        let common_headers = self.common_headers();

        let models_response = self
            .async_client
            .get(MODELS_LIST_URL)
            .headers(common_headers)
            .send()
            .await?;

        let json = models_response.json::<serde_json::Value>().await?;
        let data = self.models_from_response_json(json)?;

        Ok(data)
    }

    fn models_from_response_json(&self, json: serde_json::Value) -> error::Result<Vec<Model>> {
        json.get("data")
            .and_then(|v| v.as_array())
            .ok_or(error::ParseError::FieldNotFound("data".to_string()).into())
            .and_then(|arr| {
                arr.iter()
                    .map(|v| {
                        Model::new_parse_json(
                            &self.api_key,
                            &self.organization,
                            #[cfg(feature = "blocking")]
                            &self.blocking_client,
                            &self.async_client,
                            v,
                        )
                    })
                    .collect::<error::Result<Vec<Model>>>()
            })
    }

    /// (Blocking) Retrieves a model instance, providing basic information about the model such as the owner
    /// and permissioning.
    ///
    /// # Arguments
    ///
    /// * `model_id`: The ID of the model to use for this request
    ///
    #[cfg(feature = "blocking")]
    pub fn retrieve_model_info_blocking(&self, model_id: impl AsRef<str>) -> error::Result<Model> {
        let url = format!("{MODELS_LIST_URL}/{}", model_id.as_ref());
        let common_headers = self.common_headers();

        let json = self
            .blocking_client
            .get(url)
            .headers(common_headers)
            .send()?
            .json::<serde_json::Value>()?;
        let data = Model::new_parse_json(
            &self.api_key,
            &self.organization,
            #[cfg(feature = "blocking")]
            &self.blocking_client,
            &self.async_client,
            &json,
        )?;

        Ok(data)
    }

    /// Retrieves a model instance, providing basic information about the model such as the owner
    /// and permissioning.
    ///
    /// # Arguments
    ///
    /// * `model_id`: The ID of the model to use for this request
    ///
    pub async fn retrieve_model_info(&self, model_id: impl AsRef<str>) -> error::Result<Model> {
        let url = format!("{MODELS_LIST_URL}/{}", model_id.as_ref());
        let common_headers = self.common_headers();

        let json = self
            .async_client
            .get(url)
            .headers(common_headers)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let data = Model::new_parse_json(
            &self.api_key,
            &self.organization,
            #[cfg(feature = "blocking")]
            &self.blocking_client,
            &self.async_client,
            &json,
        )?;

        Ok(data)
    }
}

impl APIKeysAccess for Client {
    fn get_api_key(&self) -> &String {
        &self.api_key
    }

    fn get_org_id(&self) -> &Option<String> {
        &self.organization
    }
}
