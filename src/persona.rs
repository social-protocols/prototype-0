use anyhow::Result;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::SqlitePool;

/*

create table persona (
    id integer primary key -- row id
    , description string not null
);
*/

// Declare persona type corresponding to persona table above

#[derive(Serialize, Debug, FromRow)]
pub struct Persona {
    id: i64,
    description: String,
}

pub async fn get_personas(sqlite_pool: SqlitePool) -> Result<Vec<Persona>> {
    // use sqlx to return a vec<Result> directly from the database
    let personas: Vec<Persona> = sqlx::query_as::<_, Persona>(
        r#"
            select id, description
            from persona
        "#,
    )
    .fetch_all(&sqlite_pool)
    .await?;
    Ok(personas)
}
