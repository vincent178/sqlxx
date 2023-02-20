use sqlx::postgres::PgPoolOptions;
use sqlxx_macros::Model;

#[derive(Model, Debug, sqlx::FromRow)]
pub struct User {
    id: i64,
    email: String,
    auth_token: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://tradex:@localhost/tradex")
        .await?;

    let mut u = User {
        id: 1,
        email: "vincent.huang@goat.com".to_string(),
        auth_token: "123456".to_string(),
    };

    u.save(&pool).await;

    println!("user {:?}", u);

    Ok(())
}
