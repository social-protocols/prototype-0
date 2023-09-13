use anyhow::{Context, Result};
use std::env;
use chatgpt::types::CompletionResponse;

// use crate::db_setup::setup_database;

use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // let sqlite_pool = setup_database(&command_line_args.database).await;

    let args: Vec<String> = env::args().collect();

    let key = &args[1];

    let client = ChatGPT::new(key)?;

    let response: CompletionResponse = client
        .send_message("Describe in five words the Rust programming language.")
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}

