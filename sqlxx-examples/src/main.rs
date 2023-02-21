use sqlx::postgres::PgPoolOptions;
use sqlxx_macros::Model;

#[derive(Model, Debug, sqlx::FromRow)]
pub struct User {
    id: i32,
    name: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://tradex:@localhost/sqlx")
        .await?;

    let mut u = User {
        id: 1,
        name: "vincent".to_string(),
        password: "123456".to_string(),
    };

    u.save(&pool).await;

    println!("user {:?}", u);

    let mut u = User {
        id: 0,
        name: "jack".to_string(),
        password: "123456".to_string(),
    };

    u.save(&pool).await;

    println!("user {:?}", u);

    Ok(())
}
