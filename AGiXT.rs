impl AGiXTClient {
    pub async fn get_agents(&self) -> Result<Vec<Agent>, reqwest::Error> {
        let url = format!("{}/api/agent", self.base_uri);
        let response = self
            .client
            .get(&url)
            .headers(self.headers.as_ref().unwrap().clone())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let agents = response["agents"]
            .as_array()
            .ok_or(reqwest::Error::new(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid response format".to_string(),
            ))?
            .iter()
            .map(|agent| serde_json::from_value(agent.clone()))
            .collect::<Result<Vec<Agent>, _>>()?;

        Ok(agents)
    }

    pub async fn add_agent(
        &self,
        agent_name: &str,
        settings: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/api/agent", self.base_uri);
        let body = serde_json::json!({
            "agent_name": agent_name,
            "settings": settings,
        });

        let response = self
            .client
            .post(&url)
            .headers(self.headers.as_ref().unwrap().clone())
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response)
    }
}