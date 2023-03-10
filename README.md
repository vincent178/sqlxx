sqlxx
=====

sqlxx is a Rust library that extends the sqlx library with a custom macro that provides convenient functions for common database operations. With sqlxx, you can easily save, find, and delete records in your database using Rust structs.

## Installation

To use sqlxx in your Rust project, simply add it to your Cargo.toml file:

```toml
[dependencies]
sqlxx = "0.1.0"
```

## Usage

To use the save, find_by_id, and delete_by_id functions provided by sqlxx, you'll need to define a Rust struct that represents a record in your database, and deriveit with `Model` attributes:

```rust
use sqlxx::Model;

#[derive(Model)]
struct User {
    id: i32,
    name: String,
    email: String,
}
```

Once you've defined your struct, you can use the Model macro to generate the save, find_by_id, and delete_by_id functions:

```rust
use sqlxx::Model;

#[derive(Model)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn main() -> Result<(), sqlx::Error> {
    let mut user = User {
        id: 0,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    user.save(db).await;

    let loaded_user = User::find_by_id(db, user.id).await;
    loaded_user.delete(db).await;

    Ok(())
}
```

The `save` function will insert a new record into the database if the id field is None, or update an existing record if the id field has a value. The find_by_id function will load a record from the database with the given ID, and the delete_by_id function will delete a record from the database with the given ID.

## Limitations

Currently, sqlxx only supports PostgreSQL. Support for other databases may be added in the future.

## Contributing

Contributions to sqlxx are welcome! If you find a bug or would like to suggest a new feature, please open an issue on the GitHub repository. Pull requests are also welcome.

## License

sqlxx is licensed under the MIT License.
