use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use sqlx::{postgres::PgPoolOptions, Pool};
use std::env;

pub struct Conn {
    pool: Pool<Postgres>,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    name: String,
    age: i32,
}

impl Conn {
    pub async fn new() -> Self {
        let db_user = env::var("DATABASE_USER").unwrap();
        let db_password = env::var("DATABASE_PASSWORD").unwrap();
        let db_host = env::var("DATABASE_HOST").unwrap();
        let db_port = env::var("DATABASE_PORT").unwrap();
        let db_name = env::var("DATABASE_NAME").unwrap();

        let dsn = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );

        let pool = PgPoolOptions::new().max_connections(5).connect(&dsn).await;
        match pool {
            Ok(p) => {
                println!("dsn: {}", dsn);
                Self { pool: p }
            }
            Err(e) => panic!("failed to connect database {:?}", e),
        }
    }

    pub async fn users(&self) -> Result<Option<Vec<User>>, sqlx::Error> {
        let rows = sqlx::query_as::<_, User>("SELECT id, name, age FROM users")
            .fetch_all(&self.pool)
            .await;
        match rows {
            Ok(rows) => Ok(Some(
                rows.into_iter()
                    .map(|row| User {
                        id: row.id,
                        name: row.name,
                        age: row.age,
                    })
                    .collect(),
            )),
            Err(e) => Err(e),
        }
    }

    pub async fn user_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let row =
            sqlx::query_as::<_, User>("SELECT id, name, age FROM users WHERE id = $1 LIMIT 1")
                .bind(id)
                .fetch_one(&self.pool)
                .await;
        match row {
            Ok(row) => Ok(Some(User {
                id: row.id,
                name: row.name,
                age: row.age,
            })),
            Err(e) => Err(e),
        }
    }
}
