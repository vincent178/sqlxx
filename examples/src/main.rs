use sqlx::postgres::PgPoolOptions;
use sqlxx::Model;

#[derive(Model, Debug, sqlx::FromRow)]
pub struct User {
    id: i64,
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

    // update
    let mut u = User {
        id: 1,
        name: "vincent".to_string(),
        password: "123456".to_string(),
    };

    u.save(&pool).await;

    println!("user {:?}", u);

    // create
    let mut u = User {
        id: 0,
        name: "jack".to_string(),
        password: "123456".to_string(),
    };

    u.save(&pool).await.unwrap();

    println!("user {:?}", u);

    let u = User::find_by_id(&pool, u.id).await.unwrap();

    println!("user {:?}", u);

    // User::delete_by_id(&pool, u.id).await;

    u.delete(&pool).await.unwrap();

    let users = User::all(&pool).await.unwrap();

    println!("users {:?}", users);

    Ok(())
}
