# poem-sea-orm-middleware

[![Crates.io version](https://img.shields.io/crates/v/poem-sea-orm-middleware.svg?style=flat-square)](https://crates.io/crates/poem-sea-orm-middleware)

This library is the [Sea ORM](https://github.com/SeaQL/sea-orm) middleware for [Poem](https://github.com/poem-web/poem). This library is designed to make it easier for users to no longer need to manually begin transactions, or explicitly pass database connection parameters.

## Example

```rust
use poem_sea_orm_middleware::{LocalKeyExt, TxnMiddleware, TXN};

// define entity
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
}

#[handler]
async fn hello(Path(name): Path<String>) -> String {
    // get transaction from task local value
    let txn = &*TXN.cloned();

    let user = match Entity::find()
        .filter(Column::Name.eq(name.clone()))
        .one(txn)
        .await
        .unwrap()
    {
        Some(user) => user,
        None => return format!("not found: {name}"),
    };

    format!("hello: {}", user.name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // create database connection
    let db = Database::connect("mysql://root:123456@localhost:3306/db")
        .await
        .unwrap();

    // create transaction middleware
    let txn_middleware = TxnMiddleware::new(db);

    let app = Route::new()
        .at("/hello/:name", get(hello))
        // add middleware
        .with(txn_middleware);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
```

Check [examples](./examples), to see the full example.
