mod db_setup;
mod persona;
mod prompts;

use anyhow::Result;
use serde::Serialize;
use sqlx::SqlitePool;
use std::env;

use crate::db_setup::setup_database;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sqlite_pool = setup_database(database_url.as_str()).await;

    let full_thread = get_thread(1, sqlite_pool).await?;

    let args: Vec<String> = env::args().collect();

    let key = &args[1];

    // let thread = Thread{original_tweet: original_tweet.to_string(), replies: replies};

    let persona = prompts::generate_persona(key).await?;
    println!("{}", persona);

    for idx in 0..=full_thread.replies.len() {
        let thread = Thread {
            original_tweet: full_thread.original_tweet.to_string(),
            replies: full_thread
                .replies
                .iter()
                .take(idx)
                .map(|elem| elem.clone())
                .collect(),
        };
        let stance = prompts::get_stance(persona.to_string(), thread, key).await?;
        println!("{:?}", stance);
    }

    Ok(())
}

#[derive(Debug)]
pub enum Stance {
    Agree,
    Disagree,
}

#[derive(Serialize, Debug)]
pub struct Thread {
    original_tweet: String,
    replies: Vec<String>,
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
