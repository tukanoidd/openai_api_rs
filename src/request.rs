use std::{collections::BTreeMap, num::NonZeroU64};

use macros::rq;

use crate::request::chat_completion::ChatMessage;

pub mod chat_completion;
pub mod completion;

///
#[rq(
    TextCompletion = "Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position.",
    ChatCompletion = "Given a chat conversation, the model will return a chat completion response."
)]
pub struct RequestBody {
    /// Required.
    ///
    /// ID of the model to use. You can use the [`crate::client::Client::list_models`] or
    /// [`crate::client::Client::list_models_blocking`] to see all of your available models,
    /// or see the [Model overview](https://platform.openai.com/docs/models/overview) for
    /// descriptions of them.
    #[rq(on(all(req)))]
    model: String,
    /// Required.
    ///
    /// The messages to generate chat completions for, in the
    /// [chat format](https://platform.openai.com/docs/guides/chat/introduction).
    #[rq(on(ChatCompletion(req)))]
    messages: Option<Vec<ChatMessage>>,
    /// Optional. Defaults to <|endoftext|>.
    ///
    /// The `prompt`(s) to generate completions for, encoded as a string, array of strings, array of
    /// tokens, or array of token arrays.
    ///
    /// Note that <|endoftext|> is the document separator that the model sees during training,
    /// so if a `prompt` is not specified the model will generate as if from the beginning of a new
    /// document.
    #[rq(on(TextCompletion))]
    prompt: Option<Vec<String>>,
    /// Optional. Defaults to null.
    ///
    /// The `suffix` that comes after a completion of inserted text.
    #[rq(on(TextCompletion))]
    suffix: Option<String>,
    /// Optional. Defaults to 16.
    ///
    /// The maximum number of [tokens](https://platform.openai.com/tokenizer) to generate in the
    /// completion. The token count of your prompt plus `max_tokens` cannot exceed the model's
    /// context length.
    ///
    /// Most models have a context length of 2048 tokens
    /// (except for the newest models, which support 4096).
    #[rq(on(TextCompletion, ChatCompletion))]
    max_tokens: Option<u64>,
    /// Optional. Defaults to 1.
    ///
    /// What sampling `temperature` to use, between 0 and 2. Higher values like 0.8 will make the
    /// output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// It's generally recommended to alter this or top_p but not both.
    #[rq(on(TextCompletion, ChatCompletion))]
    temperature: Option<f64>,
    /// Optional. Defaults to 1.
    ///
    /// An alternative to sampling with temperature, called nucleus sampling, where the model
    /// considers the results of the tokens with `top_p` probability mass. So 0.1 means only the
    /// tokens comprising the top 10% probability mass are considered.
    ///
    /// It's generally recommended to alter this or temperature but not both.
    #[rq(on(TextCompletion, ChatCompletion))]
    top_p: Option<f64>,
    /// Optional. Defaults to 1.
    ///
    /// How many completions to generate for each prompt.
    ///
    /// Note: Because this parameter generates many completions, it can quickly consume your token
    /// quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and stop.
    #[rq(on(TextCompletion, ChatCompletion))]
    n: Option<NonZeroU64>,
    /// Optional. Defaults to false.
    ///
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only
    /// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format)
    /// as they become available, with the stream terminated by a data: (DONE)
    /// message.
    #[rq(on(TextCompletion, ChatCompletion))]
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
    #[rq(on(TextCompletion))]
    logprobs: Option<u8>,
    /// Optional. Defaults to false.
    ///
    /// Echo back the prompt in addition to the completion.
    #[rq(on(TextCompletion))]
    echo: Option<bool>,
    /// Optional. Defaults to null.
    ///
    /// Up to 4 sequences where the API will stop generating further tokens.
    /// The returned text will not contain the `stop` sequence.
    #[rq(on(TextCompletion, ChatCompletion))]
    stop: Option<Vec<String>>,
    /// Optional. Defaults to 0.0.
    ///
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they
    /// appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[rq(on(TextCompletion, ChatCompletion))]
    presence_penalty: Option<f64>,
    /// Optional. Defaults to 0.0.
    ///
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing
    /// frequency in the text so far, decreasing the model's likelihood to repeat the same line
    /// verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[rq(on(TextCompletion, ChatCompletion))]
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
    #[rq(on(TextCompletion))]
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
    #[rq(on(TextCompletion, ChatCompletion))]
    logit_bias: Option<BTreeMap<String, i64>>,
    /// Optional
    ///
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and
    /// detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[rq(on(TextCompletion, ChatCompletion))]
    user: Option<String>,
}
