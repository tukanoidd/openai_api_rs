use crate::{
    error,
    request::{
        chat_completion::{ChatCompletionResponse, CHAT_COMPLETION_URL},
        text_completion::{TextCompletionResponse, TEXT_COMPLETION_URL},
        ChatCompletionRequest, TextCompletionRequest,
    },
    APIKeysAccess,
};

macro_rules! request {
    ($($name:ident),*) => {
        paste::paste! {
            $(
                #[cfg(feature = "blocking")]
                pub fn [< request_ $name:snake _blocking >](
                    &self,
                    body: [< $name Request >],
                ) -> error::Result<[< $name Response >]> {
                    if !Self::[< $name:snake:upper S_COMPATIBLE >].contains(&self.id.as_str()) {
                        return Err(error::ModelError::[< NotCompatibleWith $name >].into());
                    }

                    let json = body.to_json()?;
                    let res = self
                        .blocking_client
                        .post([< $name:snake:upper _URL >])
                        .headers(self.common_headers())
                        .json(&json)
                        .send()?;

                    Ok(res.json()?)
                }

                pub async fn [< request_ $name:snake >](
                    &self,
                    body: [< $name Request >],
                ) -> error::Result<[< $name Response >]> {
                    if !Self::[< $name:snake:upper S_COMPATIBLE >].contains(&self.id.as_str()) {
                        return Err(error::ModelError::[< NotCompatibleWith $name >].into());
                    }

                    let json = body.to_json()?;
                    let res = self
                        .async_client
                        .post([< $name:snake:upper _URL >])
                        .headers(self.common_headers())
                        .json(&json)
                        .send()
                        .await?;

                    Ok(res.json().await?)
                }
            )*
        }
    };
}

#[derive(Debug, getset::Getters)]
pub struct Model<'client> {
    api_key: &'client String,
    org_id: &'client Option<String>,

    #[cfg(feature = "blocking")]
    blocking_client: &'client reqwest::blocking::Client,
    async_client: &'client reqwest::Client,

    #[get = "pub"]
    created: u64,
    #[get = "pub"]
    id: String,
    #[get = "pub"]
    owned_by: String,
    #[get = "pub"]
    parent: serde_json::Value, // TODO: parse this
    #[get = "pub"]
    permission: Vec<ModelPermission>,
}

impl<'client> Model<'client> {
    pub const CHAT_COMPLETIONS_COMPATIBLE: &'static [&'static str] = &[
        "gpt-4",
        "gpt-4-0314",
        "gpt-4-32k",
        "gpt-4-32k-0314",
        "gpt-3.5-turbo",
        "gpt-3.5-turbo-0301",
    ];
    pub const TEXT_COMPLETIONS_COMPATIBLE: &'static [&'static str] = &[
        "text-davinci-003",
        "text-davinci-002",
        "text-curie-001",
        "text-babbage-001",
        "text-ada-001",
        "davinci",
        "curie",
        "babbage",
        "ada",
    ];
    pub const EDITS_COMPATIBLE: &'static [&'static str] =
        &["text-davinci-edit-001", "code-davinci-edit-001"];
    pub const AUDIO_TRANSCRIPTIONS: &'static [&'static str] = &["whisper-1"];
    pub const FINE_TUNES_COMPATIBLE: &'static [&'static str] =
        &["davinci", "curie", "babbage", "ada"];
    pub const EMBEDDINGS_COMPATIBLE: &'static [&'static str] =
        &["text-embedding-ada-002", "text-search-ada-doc-001"];
    pub const MODERATIONS_COMPATIBLE: &'static [&'static str] =
        &["	text-moderation-stable", "text-moderation-latest"];

    pub fn new_parse_json(
        api_key: &'client String,
        org_id: &'client Option<String>,

        #[cfg(feature = "blocking")] blocking_client: &'client reqwest::blocking::Client,
        async_client: &'client reqwest::Client,
        json: &serde_json::Value,
    ) -> error::Result<Self> {
        let created = json
            .get("created")
            .and_then(|v| v.as_u64())
            .ok_or(error::ParseError::FieldNotFound("created".to_string()))?;
        let id = json
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or(error::ParseError::FieldNotFound("id".to_string()))?
            .to_string();
        let owned_by = json
            .get("owned_by")
            .and_then(|v| v.as_str())
            .ok_or(error::ParseError::FieldNotFound("owned_by".to_string()))?
            .to_string();
        let parent = json
            .get("parent")
            .cloned()
            .ok_or(error::ParseError::FieldNotFound("parent".to_string()))?;
        let permission = json
            .get("permission")
            .and_then(|v| v.as_array())
            .ok_or(error::ParseError::FieldNotFound("permission".to_string()).into())
            .and_then(|arr| {
                arr.iter()
                    .map(ModelPermission::parse)
                    .collect::<error::Result<Vec<ModelPermission>>>()
            })?;

        Ok(Self {
            api_key,
            org_id,

            #[cfg(feature = "blocking")]
            blocking_client,
            async_client,

            created,
            id,
            owned_by,
            parent,
            permission,
        })
    }

    request!(TextCompletion, ChatCompletion);
}

impl<'client> APIKeysAccess for Model<'client> {
    fn get_api_key(&self) -> &String {
        self.api_key
    }

    fn get_org_id(&self) -> &Option<String> {
        self.org_id
    }
}

#[derive(Debug, getset::Getters)]
pub struct ModelPermission {
    #[get = "pub"]
    allow_create_engine: bool,
    #[get = "pub"]
    allow_fine_tuning: bool,
    #[get = "pub"]
    allow_logprobs: bool,
    #[get = "pub"]
    allow_sampling: bool,
    #[get = "pub"]
    allow_search_indices: bool,
    #[get = "pub"]
    allow_view: bool,
    #[get = "pub"]
    created: u64,
    #[get = "pub"]
    group: serde_json::Value, // TODO: parse this
    #[get = "pub"]
    id: String,
    #[get = "pub"]
    is_blocking: bool,
    #[get = "pub"]
    organization: String,
}

impl ModelPermission {
    pub fn parse(json: &serde_json::Value) -> error::Result<Self> {
        let allow_create_engine = json
            .get("allow_create_engine")
            .and_then(|v| v.as_bool())
            .ok_or(error::ParseError::FieldNotFound(
                "allow_create_engine".to_string(),
            ))?;
        let allow_fine_tuning = json
            .get("allow_fine_tuning")
            .and_then(|v| v.as_bool())
            .ok_or(error::ParseError::FieldNotFound(
                "allow_fine_tuning".to_string(),
            ))?;
        let allow_logprobs = json.get("allow_logprobs").and_then(|v| v.as_bool()).ok_or(
            error::ParseError::FieldNotFound("allow_logprobs".to_string()),
        )?;
        let allow_sampling = json.get("allow_sampling").and_then(|v| v.as_bool()).ok_or(
            error::ParseError::FieldNotFound("allow_sampling".to_string()),
        )?;
        let allow_search_indices = json
            .get("allow_search_indices")
            .and_then(|v| v.as_bool())
            .ok_or(error::ParseError::FieldNotFound(
                "allow_search_indices".to_string(),
            ))?;
        let allow_view = json
            .get("allow_view")
            .and_then(|v| v.as_bool())
            .ok_or(error::ParseError::FieldNotFound("allow_view".to_string()))?;
        let created = json
            .get("created")
            .and_then(|v| v.as_u64())
            .ok_or(error::ParseError::FieldNotFound("created".to_string()))?;
        let group = json
            .get("group")
            .cloned()
            .ok_or(error::ParseError::FieldNotFound("group".to_string()))?;
        let id = json
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or(error::ParseError::FieldNotFound("id".to_string()))?
            .to_string();
        let is_blocking = json
            .get("is_blocking")
            .and_then(|v| v.as_bool())
            .ok_or(error::ParseError::FieldNotFound("is_blocking".to_string()))?;
        let organization = json
            .get("organization")
            .and_then(|v| v.as_str())
            .ok_or(error::ParseError::FieldNotFound("organization".to_string()))?
            .to_string();

        Ok(Self {
            allow_create_engine,
            allow_fine_tuning,
            allow_logprobs,
            allow_sampling,
            allow_search_indices,
            allow_view,
            created,
            group,
            id,
            is_blocking,
            organization,
        })
    }
}
