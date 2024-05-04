use reqwest::{header, Client, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AGiXTSDK {
    base_uri: String,
    headers: header::HeaderMap,
}

impl AGiXTSDK {
    pub fn new(base_uri: &str, api_key: Option<&str>) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        if let Some(key) = api_key {
            let api_key = key.replace("Bearer ", "").replace("bearer ", "");
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("{}", api_key)).unwrap(),
            );
        }

        let base_uri = if base_uri.ends_with('/') {
            base_uri.to_string()
        } else {
            format!("{}/", base_uri)
        };

        AGiXTSDK {
            base_uri,
            headers,
        }
    }

    async fn handle_error(&self, error: reqwest::Error) -> String {
        println!("Error: {:?}", error);
        "Unable to retrieve data.".to_string()
    }

    pub async fn get_providers(&self) -> Result<Vec<String>> {
        let url = format!("{}api/provider", self.base_uri);
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<ProvidersResponse>()
            .await?;
        Ok(response.providers)
    }

    pub async fn get_providers_by_service(&self, service: &str) -> Result<Vec<String>> {
        let url = format!("{}api/providers/service/{}", self.base_uri, service);
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<ProvidersResponse>()
            .await?;
        Ok(response.providers)
    }

    pub async fn get_provider_settings(&self, provider_name: &str) -> Result<HashMap<String, serde_json::Value>> {
        let url = format!("{}api/provider/{}", self.base_uri, provider_name);
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<ProviderSettingsResponse>()
            .await?;
        Ok(response.settings)
    }

    pub async fn get_embed_providers(&self) -> Result<Vec<String>> {
        let url = format!("{}api/embedding_providers", self.base_uri);
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<ProvidersResponse>()
            .await?;
        Ok(response.providers)
    }

    pub async fn get_embedders(&self) -> Result<HashMap<String, serde_json::Value>> {
        let url = format!("{}api/embedders", self.base_uri);
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<EmbeddersResponse>()
            .await?;
        Ok(response.embedders)
    }

    pub async fn add_agent(&self, agent_name: &str, settings: &HashMap<String, serde_json::Value>) -> Result<serde_json::Value> {
        let url = format!("{}api/agent", self.base_uri);
        let response = Client::new()
            .post(url)
            .headers(self.headers.clone())
            .json(&AddAgentRequest {
                agent_name,
                settings,
            })
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn import_agent(
        &self,
        agent_name: &str,
        settings: &HashMap<String, serde_json::Value>,
        commands: &HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let url = format!("{}api/agent/import", self.base_uri);
        let response = Client::new()
            .post(url)
            .headers(self.headers.clone())
            .json(&ImportAgentRequest {
                agent_name,
                settings,
                commands,
            })
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn rename_agent(&self, agent_name: &str, new_name: &str) -> Result<serde_json::Value> {
        let url = format!("{}api/agent/{}", self.base_uri, agent_name);
        let response = Client::new()
            .patch(url)
            .headers(self.headers.clone())
            .json(&RenameAgentRequest { new_name })
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn update_agent_settings(
        &self,
        agent_name: &str,
        settings: &HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let url = format!("{}api/agent/{}", self.base_uri, agent_name);
        let response = Client::new()
            .put(url)
            .headers(self.headers.clone())
            .json(&UpdateAgentSettingsRequest {
                settings,
                agent_name,
            })
            .send()
            .await?
            .json::<UpdateAgentSettingsResponse>()
            .await?;
        Ok(response.message)
    }

    pub async fn update_agent_commands(
        &self,
        agent_name: &str,
        commands: &HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let url = format!("{}api/agent/{}/commands", self.base_uri, agent_name);
        let response = Client::new()
            .put(url)
            .headers(self.headers.clone())
            .json(&UpdateAgentCommandsRequest {
                commands,
                agent_name,
            })
            .send()
            .await?
            .json::<UpdateAgentCommandsResponse>()
            .await?;
        Ok(response.message)
    }

    pub async fn delete_agent(&self, agent_name: &str) -> Result<String> {
        let url = format!("{}api/agent/{}", self.base_uri, agent_name);
        let response = Client::new()
            .delete(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<DeleteAgentResponse>()
            .await?;
        Ok(response.message)
    }

    pub async fn get_agents(&self) -> Result<Vec<serde_json::Value>> {
        let url = format!("{}api/agent", self.base_uri);
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<GetAgentsResponse>()
            .await?;
        Ok(response.agents)
    }

    pub async fn get_agentconfig(&self, agent_name: &str) -> Result<serde_json::Value> {
        let url = format!("{}api/agent/{}", self.base_uri, agent_name);
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<GetAgentConfigResponse>()
            .await?;
        Ok(response.agent)
    }

    pub async fn get_conversations(&self, agent_name: &str) -> Result<Vec<String>> {
        let url = if agent_name.is_empty() {
            format!("{}api/conversations", self.base_uri)
        } else {
            format!("{}api/{}/conversations", self.base_uri, agent_name)
        };
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<GetConversationsResponse>()
            .await?;
        Ok(response.conversations)
    }

    pub async fn get_conversation(
        &self,
        agent_name: &str,
        conversation_name: &str,
        limit: usize,
        page: usize,
    ) -> Result<Vec<serde_json::Value>> {
        let url = format!("{}api/conversation", self.base_uri);
        let response = Client::new()
            .get(url)
            .headers(self.headers.clone())
            .json(&GetConversationRequest {
                conversation_name,
                agent_name,
                limit,
                page,
            })
            .send()
            .await?
            .json::<GetConversationResponse>()
            .await?;
        Ok(response.conversation_history)
    }

    pub async fn new_conversation(
        &self,
        agent_name: &str,
        conversation_name: &str,
        conversation_content: &[serde_json::Value],
    ) -> Result<Vec<serde_json::Value>> {
        let url = format!("{}api/conversation", self.base_uri);
        let response = Client::new()
            .post(url)
            .headers(self.headers.clone())
            .json(&NewConversationRequest {
                conversation_name,
                agent_name,
                conversation_content,
            })
            .send()
            .await?
            .json::<NewConversationResponse>()
            .await?;
        Ok(response.conversation_history)
    }

    pub async fn delete_conversation(
        &self,
        agent_name: &str,
        conversation_name: &str,
    ) -> Result<String> {
        let url = format!("{}api/conversation", self.base_uri);
        let response = Client::new()
            .delete(url)
            .headers(self.headers.clone())
            .json(&DeleteConversationRequest {
                conversation_name,
                agent_name,
            })
            .send()
            .await?
            .json::<DeleteConversationResponse>()
            .await?;
        Ok(response.message)
    }

    pub async fn delete_conversation_message(
        &self,
        agent_name: &str,
        conversation_name: &str,
        message: &str,
    ) -> Result<String> {
        let url = format!("{}api/conversation/message", self.base_uri);
        let response = Client::new()
            .delete(url)
            .headers(self.headers.clone())
            .json(&DeleteConversationMessageRequest {
                message,
                agent_name,
                conversation_name,
            })
            .send()
            .await?
            .json::<DeleteConversationMessageResponse>()
            .await?;
        Ok(response.message)
    }

    pub async fn update_conversation_message(
        &self,
        agent_name: &str,
        conversation_name: &str,
        message: &str,
        new_message: &str,
    ) -> Result<String> {
        let url = format!("{}api/conversation/message", self.base_uri);
        let response = Client::new()
            .put(url)
            .headers(self.headers.clone())
            .json(&UpdateConversationMessageRequest {
                message,
                new_message,
                agent_name,
                conversation_name,
            })
            .send()
            .await?
            .json::<UpdateConversationMessageResponse>()
            .await?;
        Ok(response.message)
    }

    pub async fn prompt_agent(
        &self,
        agent_name: &str,
        prompt_name: &str,
        prompt_args: &HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let url = format!("{}api/agent/{}/prompt", self.base_uri, agent_name);
        let response = Client::new()
            .post(url)
            .headers(self.headers.clone())
            .json(&PromptAgentRequest {
                prompt_name,
                prompt_args,
            })
            .send()
            .await?
            .json::<PromptAgentResponse>()
            .await?;
        Ok(response.response)
    }

    pub async fn instruct(
        &self,
        agent_name: &str,
        user_input: &str,
        conversation: &str,
    ) -> Result<String> {
        self.prompt_agent(
            agent_name,
            "instruct",
            &HashMap::from([
                ("user_input".to_string(), serde_json::Value::String(user_input.to_string())),
                ("disable_memory".to_string(), serde_json::Value::Bool(true)),
                ("conversation_name".to_string(), serde_json::Value::String(conversation.to_string())),
            ]),
        )
        .await
    }

    pub async fn chat(
        &self,
        agent_name: &str,
        user_input: &str,
        conversation: &str,
        context_results: usize,
    ) -> Result<String> {
        self.prompt_agent(
            agent_name,
            "Chat",
            &HashMap::from([
                ("user_input".to_string(), serde_json::Value::String(user_input.to_string())),
                ("context_results".to_string(), serde_json::Value::Number(context_results.into())),
                ("conversation_name".to_string(), serde_json::Value::String(conversation.to_string())),
                ("disable_memory".to_string(), serde_json::Value::Bool(true)),
            ]),
        )
        .await
    }

    pub async fn smartinstruct(
        &self,
        agent_name: &str,
        user_input: &str,
        conversation: &str,
    ) -> Result<String> {
        self.instruct(agent_name, user_input, conversation).await
    }

    pub async fn smartchat(
        &self,
        agent_name: &str,
        user_input: &str,
        conversation: &str,
    ) -> Result<String> {
        self.chat(agent_name, user_input, conversation, 1).await
    }
}

#[derive(Deserialize)]
struct ProvidersResponse {
    providers: Vec<String>,
}

#[derive(Deserialize)]
struct ProviderSettingsResponse {
    settings: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct EmbeddersResponse {
    embedders: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
struct AddAgentRequest<'a> {
    agent_name: &'a str,
    settings: &'a HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
struct ImportAgentRequest<'a> {
    agent_name: &'a str,
    settings: &'a HashMap<String, serde_json::Value>,
    commands: &'a HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
struct RenameAgentRequest<'a> {
    new_name: &'a str,
}

#[derive(Serialize)]
struct UpdateAgentSettingsRequest<'a> {
    settings: &'a HashMap<String, serde_json::Value>,
    agent_name: &'a str,
}

#[derive(Deserialize)]
struct UpdateAgentSettingsResponse {
    message: String,
}

#[derive(Serialize)]
struct UpdateAgentCommandsRequest<'a> {
    commands: &'a HashMap<String, serde_json::Value>,
    agent_name: &'a str,
}

#[derive(Deserialize)]
struct UpdateAgentCommandsResponse {
    message: String,
}

#[derive(Deserialize)]
struct DeleteAgentResponse {
    message: String,
}

#[derive(Deserialize)]
struct GetAgentsResponse {
    agents: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct GetAgentConfigResponse {
    agent: serde_json::Value,
}

#[derive(Deserialize)]
struct GetConversationsResponse {
    conversations: Vec<String>,
}

#[derive(Serialize)]
struct GetConversationRequest<'a> {
    conversation_name: &'a str,
    agent_name: &'a str,
    limit: usize,
    page: usize,
}

#[derive(Deserialize)]
struct GetConversationResponse {
    conversation_history: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct NewConversationRequest<'a> {
    conversation_name: &'a str,
    agent_name: &'a str,
    conversation_content: &'a [serde_json::Value],
}

#[derive(Deserialize)]
struct NewConversationResponse {
    conversation_history: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct DeleteConversationRequest<'a> {
    conversation_name: &'a str,
    agent_name: &'a str,
}

#[derive(Deserialize)]
struct DeleteConversationResponse {
    message: String,
}

#[derive(Serialize)]
struct DeleteConversationMessageRequest<'a> {
    message: &'a str,
    agent_name: &'a str,
    conversation_name: &'a str,
}

#[derive(Deserialize)]
struct DeleteConversationMessageResponse {
    message: String,
}

#[derive(Serialize)]
struct UpdateConversationMessageRequest<'a> {
    message: &'a str,
    new_message: &'a str,
    agent_name: &'a str,
    conversation_name: &'a str,
}

#[derive(Deserialize)]
struct UpdateConversationMessageResponse {
    message: String,
}

#[derive(Serialize)]
struct PromptAgentRequest<'a> {
    prompt_name: &'a str,
    prompt_args: &'a HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct PromptAgentResponse {
    response: String,
}