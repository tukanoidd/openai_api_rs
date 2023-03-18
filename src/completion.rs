use std::{collections::BTreeMap, num::NonZeroU64};

use const_format::concatcp;

use crate::client::BASE_URL;

pub const COMPLETION_URL: &str = concatcp!(BASE_URL, "/completions");

/// Given a prompt, the model will return one or more predicted completions, and can also return
/// the probabilities of alternative tokens at each position.
pub struct CompletionRequestBodyBuilder {
    /// Required.
    ///
    /// ID of the model to use. You can use the [`crate::client::Client::list_models`] or
    /// [`crate::client::Client::list_models_blocking`] to see all of your available models,
    /// or see the [Model overview](https://platform.openai.com/docs/models/overview) for
    /// descriptions of them.
    model: String,
    /// Optional. Defaults to <|endoftext|>.
    ///
    /// The `prompt`(s) to generate completions for, encoded as a string, array of strings, array of
    /// tokens, or array of token arrays.
    ///
    /// Note that <|endoftext|> is the document separator that the model sees during training,
    /// so if a `prompt` is not specified the model will generate as if from the beginning of a new
    /// document.
    prompt: Option<Vec<String>>,
    /// Optional. Defaults to null.
    ///
    /// The `suffix` that comes after a completion of inserted text.
    suffix: Option<String>,
    /// Optional. Defaults to 16.
    ///
    /// The maximum number of [tokens](https://platform.openai.com/tokenizer) to generate in the
    /// completion. The token count of your prompt plus `max_tokens` cannot exceed the model's
    /// context length.
    ///
    /// Most models have a context length of 2048 tokens
    /// (except for the newest models, which support 4096).
    max_tokens: Option<u64>,
    /// Optional. Defaults to 1.
    ///
    /// What sampling `temperature` to use, between 0 and 2. Higher values like 0.8 will make the
    /// output more random, while lower values like 0.2 will make it more focused and deterministic.
    /// It's generally recommended to alter this or top_p but not both.
    temperature: Option<f64>,
    /// Optional. Defaults to 1.
    ///
    /// An alternative to sampling with temperature, called nucleus sampling, where the model
    /// considers the results of the tokens with `top_p` probability mass. So 0.1 means only the
    /// tokens comprising the top 10% probability mass are considered.
    ///
    /// It's generally recommended to alter this or temperature but not both.
    top_p: Option<f64>,
    /// Optional. Defaults to 1.
    ///
    /// How many completions to generate for each prompt.
    ///
    /// Note: Because this parameter generates many completions, it can quickly consume your token
    /// quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and stop.
    n: Option<NonZeroU64>,
    /// Optional. Defaults to false.
    ///
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only
    /// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format)
    /// as they become available, with the stream terminated by a data: \[DONE]
    /// message.
    stream: Option<bool>,
    /// Optional. Defaults to null.
    ///
    /// Include the log probabilities on the `logprobs` most likely tokens,
    /// as well the chosen tokens.
    ///
    /// For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens.
    /// The API will always return the logprob of the sampled token, so there may be up to
    /// `logprobs` + 1 elements in the response.
    ///
    /// The maximum value for `logprobs` is 5. If you need more than this, please contact OpenAI
    /// through their [Help center](https://help.openai.com/) and describe your use case.
    logprobs: Option<u8>,
    /// Optional. Defaults to false.
    ///
    /// Echo back the prompt in addition to the completion.
    echo: Option<bool>,
    /// Optional. Defaults to null.
    ///
    /// Up to 4 sequences where the API will stop generating further tokens.
    /// The returned text will not contain the `stop` sequence.
    stop: Option<Vec<String>>,
    /// Optional. Defaults to 0.0.
    ///
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they
    /// appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    presence_penalty: Option<f64>,
    /// Optional. Defaults to 0.0.
    ///
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing
    /// frequency in the text so far, decreasing the model's likelihood to repeat the same line
    /// verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    frequency_penalty: Option<f64>,
    /// Optional. Defaults to 1.
    ///
    /// Generates `best_of` completions server-side and returns the "best"
    /// (the one with the highest log probability per token). Results cannot be streamed.
    ///
    /// When used with `n`, `best_of` controls the number of candidate completions and n specifies
    /// how many to return – `best_of` must be greater than `n`.
    ///
    /// Note: Because this parameter generates many completions, it can quickly consume your token
    /// quota. Use carefully and ensure that you have reasonable settings for max_tokens and stop.
    best_of: Option<u64>,
    /// Optional. Defaults to null.
    ///
    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer)
    /// to an associated bias value from -100 to 100.
    /// You can use this [tokenizer](https://platform.openai.com/tokenizer?view=bpe) tool
    /// (which works for both GPT-2 and GPT-3) to convert text to token IDs.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or
    /// increase likelihood of selection; values like -100 or 100 should result in a ban or
    /// exclusive selection of the relevant token.
    ///
    /// As an example, you can pass {"50256": -100} to prevent the <|endoftext|> token from being
    /// generated.
    logit_bias: Option<BTreeMap<String, i64>>,
    /// Optional
    ///
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and
    /// detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    user: Option<String>,
}

impl CompletionRequestBodyBuilder {
    pub const DEFAULT_MAX_TOKENS: u64 = 16;
    pub const DEFAULT_TEMPERATURE: f64 = 1.0;
    pub const DEFAULT_TOP_T: f64 = 1.0;
    pub const DEFAULT_N: u64 = 1;
    pub const DEFAULT_STREAM: bool = false;
    pub const DEFAULT_ECHO: bool = false;
    pub const DEFAULT_PRESENCE_PENALTY: f64 = 0.0;
    pub const DEFAULT_FREQUENCY_PENALTY: f64 = 0.0;
    pub const DEFAULT_BEST_OF: u64 = 1;

    pub fn new(model: impl AsRef<str>) -> Self {
        Self {
            model: model.as_ref().to_string(),
            prompt: None,
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }

    pub fn to_json(self) -> serde_json::Value {
        let mut res = serde_json::Map::new();

        res.insert("model".to_string(), serde_json::Value::String(self.model));

        if let Some(prompt) = self.prompt {
            res.insert(
                "prompt".to_string(),
                serde_json::Value::Array(
                    prompt.into_iter().map(serde_json::Value::String).collect(),
                ),
            );
        }

        if let Some(suffix) = self.suffix {
            res.insert("suffix".to_string(), serde_json::Value::String(suffix));
        }

        if let Some(max_tokens) = self.max_tokens {
            res.insert(
                "max_tokens".to_string(),
                serde_json::Value::Number(serde_json::Number::from(max_tokens)),
            );
        }

        if let Some(temperature) = self.temperature {
            res.insert(
                "temperature".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(temperature).unwrap()),
            );
        }

        if let Some(top_p) = self.top_p {
            res.insert(
                "top_p".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(top_p).unwrap()),
            );
        }

        if let Some(n) = self.n {
            res.insert(
                "n".to_string(),
                serde_json::Value::Number(serde_json::Number::from(n.get())),
            );
        }

        if let Some(stream) = self.stream {
            res.insert("stream".to_string(), serde_json::Value::Bool(stream));
        }

        if let Some(logprobs) = self.logprobs {
            res.insert(
                "logprobs".to_string(),
                serde_json::Value::Number(serde_json::Number::from(logprobs)),
            );
        }

        if let Some(echo) = self.echo {
            res.insert("echo".to_string(), serde_json::Value::Bool(echo));
        }

        if let Some(stop) = self.stop {
            res.insert(
                "stop".to_string(),
                serde_json::Value::Array(stop.into_iter().map(serde_json::Value::String).collect()),
            );
        }

        if let Some(presence_penalty) = self.presence_penalty {
            res.insert(
                "presence_penalty".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(presence_penalty).unwrap()),
            );
        }

        if let Some(frequency_penalty) = self.frequency_penalty {
            res.insert(
                "frequency_penalty".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(frequency_penalty).unwrap()),
            );
        }

        if let Some(best_of) = self.best_of {
            res.insert(
                "best_of".to_string(),
                serde_json::Value::Number(serde_json::Number::from(best_of)),
            );
        }

        if let Some(logit_bias) = self.logit_bias {
            res.insert(
                "logit_bias".to_string(),
                serde_json::Value::Object(
                    logit_bias
                        .into_iter()
                        .map(|(k, v)| (k, serde_json::Value::Number(serde_json::Number::from(v))))
                        .collect(),
                ),
            );
        }

        if let Some(user) = self.user {
            res.insert("user".to_string(), serde_json::Value::String(user));
        }

        serde_json::Value::Object(res)
    }

    pub fn add_prompt(mut self, prompt: impl AsRef<str>) -> Self {
        match &mut self.prompt {
            None => return self.prompts([prompt.as_ref().to_string()]),
            Some(sprompt) => sprompt.push(prompt.as_ref().to_string()),
        }

        self
    }

    pub fn add_prompts(mut self, prompts: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        match &mut self.prompt {
            None => return self.prompts(prompts),
            Some(sprompt) => sprompt.extend(prompts.into_iter().map(|p| p.as_ref().to_string())),
        }

        self
    }

    pub fn prompts(mut self, prompts: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.prompt = Some(
            prompts
                .into_iter()
                .map(|p| p.as_ref().to_string())
                .collect(),
        );

        self
    }

    pub fn suffix(mut self, suffix: impl AsRef<str>) -> Self {
        self.suffix = Some(suffix.as_ref().to_string());

        self
    }

    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);

        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);

        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);

        self
    }

    pub fn n(mut self, n: NonZeroU64) -> Self {
        self.n = Some(n);

        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);

        self
    }

    pub fn logprobs(mut self, logprobs: u8) -> Self {
        self.logprobs = Some(logprobs);

        self
    }

    pub fn echo(mut self, echo: bool) -> Self {
        self.echo = Some(echo);

        self
    }

    pub fn add_stop(mut self, stop: impl AsRef<str>) -> Self {
        match &mut self.stop {
            None => return self.stops([stop.as_ref().to_string()]),
            Some(sstop) => sstop.push(stop.as_ref().to_string()),
        }

        self
    }

    pub fn add_stops(mut self, stop: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        match &mut self.stop {
            None => return self.stops(stop),
            Some(sstop) => sstop.extend(stop.into_iter().map(|s| s.as_ref().to_string())),
        }

        self
    }

    pub fn stops(mut self, stop: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.stop = Some(stop.into_iter().map(|s| s.as_ref().to_string()).collect());

        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);

        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);

        self
    }

    pub fn best_of(mut self, best_of: u64) -> Self {
        self.best_of = Some(best_of);

        self
    }

    pub fn logit_bias(mut self, logit_bias: impl IntoIterator<Item = (String, i64)>) -> Self {
        self.logit_bias = Some(logit_bias.into_iter().collect());

        self
    }

    pub fn user(mut self, user: impl AsRef<str>) -> Self {
        self.user = Some(user.as_ref().to_string());

        self
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct TextCompletionResponse {
    pub choices: Vec<TextCompletionChoice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: TextCompletionUsage,
}

#[derive(Debug, serde::Deserialize)]
pub struct TextCompletionChoice {
    pub finish_reason: String,
    pub index: u64,
    pub logprobs: Option<u8>,
    pub text: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TextCompletionUsage {
    pub completion_tokens: u64,
    pub prompt_tokens: u64,
    pub total_tokens: u64,
}
