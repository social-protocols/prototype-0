use anyhow::anyhow;
use anyhow::Result;
use chatgpt::types::CompletionResponse;
use serde::Deserialize;

use chatgpt::prelude::*;

use crate::Stance;
use crate::Thread;

pub async fn generate_persona(key: &str) -> Result<String> {
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            // .engine(ChatGPTEngine::Gpt35Turbo)
            .temperature(0.8)
            .max_tokens(1000u32)
            .build()
            .unwrap(),
    )?;

    let prompt = format!(
        r#"
        	Generate a random persona
        "#
    );

    let response: CompletionResponse = client.send_message(prompt).await?;
    Ok(response.message().content.clone())
}

pub async fn get_stance(persona: String, thread: Thread, key: &str) -> Result<Stance> {
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            // .engine(ChatGPTEngine::Gpt35Turbo)
            .temperature(0.8)
            .max_tokens(1000u32)
            .build()
            .unwrap(),
    )?;

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct ChatGPTResult {
        reasoning: String,
        verdict: String,
    }

    let serialized_thread = serde_json::to_string(&thread).unwrap();

    let prompt = format!(
        r#"
            Simulate the following persona.

            {persona}

            Take a deep breath and reason step by step about the original tweet. Conclude your reasoning with your verdict about the original tweet by simply stating "agree" or "disagree".
            Reply with a JSON with the following shape:

            {{
                "reasoning": "string",
                "verdict": "string",
            }}

            {serialized_thread}
        "#
    );

    let response: CompletionResponse = client.send_message(prompt).await?;

    let parsed_response: ChatGPTResult = serde_json::from_str(response.message().content.as_str())?;

    println!("{:?}", parsed_response);

    match parsed_response.verdict.to_lowercase().as_str() {
        "disagree" => Ok(Stance::Disagree),
        "agree" => Ok(Stance::Agree),
        _ => Err(anyhow!(
            "Could decode stance for response: {:?}",
            parsed_response
        )),
    }
}
