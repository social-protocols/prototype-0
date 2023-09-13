
mod db_setup;

use anyhow::anyhow;
use anyhow::Result;
use chatgpt::types::CompletionResponse;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::env;

use crate::db_setup::setup_database;

use chatgpt::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url:String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sqlite_pool = setup_database(database_url.as_str()).await;


    let t = get_thread(1, sqlite_pool).await?;
    println!("{:?}", t);

    let args: Vec<String> = env::args().collect();

    let key = &args[1];


    



    let original_tweet = r#"
        The reason I know capitalism is inefficient is that people work much better via intrinsic motivation than extrinsic motivation

        AND it is very rare / nearly impossible to align extrinsic/intrinsic incentives in a totally abstracted race to the bottom economic framework.
        "#;

    let replies: Vec<String> = vec![
        r#"
            And yet intrinsic motivation isn’t always sufficient to produce cooperation at scale.

            I see it mostly like a layer cake:

            You need the base to be a stable structure for the provision of basic needs irrespective of how it makes you feel in the moment.

            If that works, you get to build intricate structures of artistic expression on top that produce interest in the form and look of the cake.

            A select few get to fixate their eyes on the cherry on top, which is complete realization of internal purpose in their daily lived reality.

            I do think we can produce much greater alignment of internal and external worlds, but it’s not by privileging the internal world.

            Instead, it’s by continuously seesawing between building in one and balancing it out by shifting focus to the other, and back again.
            "#.to_string(),
    ];

    // let thread = Thread{original_tweet: original_tweet.to_string(), replies: replies};

    let persona = r#"
        Name: Emily Anderson
        Age: 28
        Occupation: Financial Analyst
        Location: New York City

        Demographics:

        - Gender: Female
        - Ethnicity: Caucasian
        - Marital Status: Single
        - Education: Bachelor's degree in Finance

        Psychographics:

        - Personality Traits: Detail-oriented, analytical, ambitious, organized
        - Interests: Investment strategies, financial planning, attending financial seminars
        - Tech-savvy: Comfortable using financial software, apps, and online platforms for managing personal finances

        Goals and Motivations:

        - Career Advancement: Emily wants to climb the corporate ladder and become a financial manager.
        - Financial Independence: She aspires to have enough savings to feel secure and retire early.
        - Lifestyle: Emily values a comfortable lifestyle and wants to travel and experience new cultures.

        Challenges and Pain Points:

        - Work-life balance: Balancing her demanding job and personal life can be challenging.
        - Financial Planning: Emily is overwhelmed with investment options and needs help creating a long-term plan.
        - Limited Time: As a busy professional, she struggles to find time for personal finance management and research.

        Media Consumption:

        - Reading: Financial newsletters, industry magazines, and books on personal finance.
        - Online Sources: Researching investment options, following financial influencers

        The persona is reading an online discussion thread. The thread starts with a tweet, followed by a thread of zero or more replies to the tweet.
    "#;

    for idx in 0..=replies.len() {
        let thread = Thread {
            original_tweet: original_tweet.to_string(),
            replies: replies.iter().take(idx).map(|elem| elem.clone()).collect(),
        };
        let stance = get_stance(persona.to_string(), thread, key).await?;
        println!("{:?}", stance);
    }

    // let stance = get_stance(persona.to_string(), thread, key).await?;

    Ok(())
}

#[derive(Debug)]
enum Stance {
    Agree,
    Disagree,
}

#[derive(Serialize, Debug)]
struct Thread {
    original_tweet: String,
    replies: Vec<String>,
}

async fn get_stance(persona: String, thread: Thread, key: &str) -> Result<Stance> {
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            // .engine(ChatGPTEngine::Gpt35Turbo)
            .temperature(0.8)
            .max_tokens(1000u32)
            .build()
            .unwrap(),
    )?;

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

/*

create table post (
    id integer primary key -- row id
    , content text not null
    , parent integer references post (id) on delete cascade on update cascade -- nullable
);
*/

pub async fn get_thread(post_id: i32, sqlite_pool: SqlitePool) -> Result<Thread> {
    let original_post = sqlx::query!(
        r#"
        select content
        from post
        where id = $1
    "#,
        post_id
    )
    .fetch_one(&sqlite_pool)
    .await?;

    // use sqlx to read a vec<String> directly from the database
    let replies: Vec<String> = sqlx::query!(
        r#"
        select content
        from post
        where parent = $1
    "#,
        post_id
    )
    .fetch_all(&sqlite_pool)
    .await?
    .iter()
    .map(|row| row.content.clone())
    .collect();
    Ok(Thread {
        original_tweet: original_post.content,
        replies,
    })
}
