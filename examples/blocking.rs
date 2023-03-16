use openai_api_rs::client::Client;

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
    let completion_request = text_davinci_model
        .init_completion_request_builder()
        .add_prompt("This is a test");

    // Request the completion
    let completion = text_davinci_model
        .request_completion_blocking(completion_request)
        .unwrap();

    println!("{:#?}", completion);

    // Get text-davinci-003 model
    let gpt35_turbo_model = client
        .retrieve_model_info_blocking("gpt-3.5-turbo")
        .unwrap();

    // Init the completion request for this model and configure it
    let completion_request = gpt35_turbo_model
        .init_completion_request_builder()
        .add_prompt("This is a test");

    // Request the completion, expecting an error since this model is not supposed to be compatible
    // with completions
    match gpt35_turbo_model.request_completion_blocking(completion_request) {
        Ok(completion) => panic!("Expected error, got {:?}", completion),
        Err(err) => println!("Got expected error: {}", err),
    };
}
