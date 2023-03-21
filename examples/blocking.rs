use openai_api_rs::request::chat_completion::{ChatMessage, ChatRole};
use openai_api_rs::request::{ChatCompletionRequest, Request};
use openai_api_rs::{client::Client, request::TextCompletionRequest};

fn main() {
    // Get the API key from the environment (incl. .enf file)
    let api_key = dotenvy::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    // Create the client
    let client = Client::new(api_key);

    // Get all models
    // let models = client.list_models_blocking().unwrap();

    // Get text-davinci-003 model
    let text_davinci_model = client
        .retrieve_model_info_blocking("text-davinci-003")
        .unwrap();

    // Init the completion request for this model and configure it
    let completion_request = TextCompletionRequest::init(&text_davinci_model)
        .with_prompt(vec!["This is a test".to_string()]);

    // Request the completion
    let completion = completion_request.request_blocking().unwrap();

    println!("{:#?}", completion);

    // Get text-davinci-003 model
    let gpt35_turbo_model = client
        .retrieve_model_info_blocking("gpt-3.5-turbo")
        .unwrap();

    // Init the text completion request for this model and configure it
    let completion_request = TextCompletionRequest::init(&gpt35_turbo_model)
        .with_prompt(vec!["This is a test".to_string()]);

    // Request the text completion, expecting an error since this model is not supposed to be compatible
    // with completions
    match completion_request.request_blocking() {
        Ok(completion) => panic!("Expected error, got {:?}", completion),
        Err(err) => println!("Got expected error: {}", err),
    };

    // Init the chat completion request for this model and configure it
    let chat_completion_request = ChatCompletionRequest::init(
        &gpt35_turbo_model,
        vec![ChatMessage {
            role: ChatRole::User,
            content: "Hello, how are you?".to_string(),
        }],
    );

    // Request the chat completion
    let response = chat_completion_request.request_blocking().unwrap();

    // Print out the chat completion response
    println!("{:#?}", response);
}
