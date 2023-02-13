# poem-sea-orm-middleware

[![Crates.io version](https://img.shields.io/crates/v/poem-sea-orm-middleware.svg?style=flat-square)](https://crates.io/crates/poem-sea-orm-middleware)

This library is the [Sea ORM](https://github.com/SeaQL/sea-orm) middleware for [Poem](https://github.com/poem-web/poem). This library is designed to make it easier for users to no longer need to manually begin transactions.

## Example

```rust
/// explicit transaction
#[handler]
async fn hello(
    Path(name): Path<String>,
    // get transaction from parameter rather than task local
    Data(txn): Data<&ArcTxn>,
) -> String {
    let txn = &**txn;

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


/// implicit transaction
#[handler]
async fn hello(Path(name): Path<String>) -> String {
    // get transaction from task local rather than passing it as a parameter
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
```

Check [examples](./examples), to see the full examples.
