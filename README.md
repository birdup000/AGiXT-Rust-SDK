# AGiXT SDK RUST

The AGiXT SDK is a Rust library that provides a convenient interface for interacting with the [AGiXT](https://github.com/Josh-XT/AGiXT) API. It allows you to perform various operations such as managing agents, conversations, and prompts.

**Note: This SDK is still in development and may undergo changes.**

## Features

The AGiXT SDK currently supports the following features:

- Retrieving a list of providers and their settings
- Retrieving a list of embedding providers and embedders
- Managing agents (adding, importing, renaming, updating settings and commands, deleting)
- Retrieving a list of agents and their configurations
- Managing conversations (getting, creating, deleting, updating messages)
- Prompting agents with custom arguments
- Providing high-level methods for common tasks like `instruct`, `chat`, `smartinstruct`, and `smartchat`

## Usage

Here's an example of how to use the AGiXT SDK:

```rust
use agixtsdk::AGiXTSDK;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let sdk = AGiXTSDK::new("https://api.agixtsdk.com", Some("your_api_key"));

    // Get a list of providers
    let providers = sdk.get_providers().await.unwrap();
    println!("Providers: {:?}", providers);

    // Get a list of providers by service
    let providers_by_service = sdk.get_providers_by_service("text-generation").await.unwrap();
    println!("Providers by service: {:?}", providers_by_service);

    // Get provider settings
    let provider_settings = sdk.get_provider_settings("openai").await.unwrap();
    println!("Provider settings: {:?}", provider_settings);

    // Add a new agent
    let mut settings = HashMap::new();
    settings.insert("temperature".to_string(), 0.7.into());
    let agent_response = sdk.add_agent("my_agent", &settings).await.unwrap();
    println!("New agent response: {:?}", agent_response);
}
```

For more detailed information about the available methods and their usage, please refer to the code documentation.

## Project Link

The AGiXT project, which this SDK interacts with, can be found on GitHub: [https://github.com/Josh-XT/AGiXT](https://github.com/Josh-XT/AGiXT)
