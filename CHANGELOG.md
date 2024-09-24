# Changelog

## [0.7.0] 2024-09-24

All the code has been changed.

- feat: support nested transaction.
- feat: expose task-local variable `DATA_SOURCES` for easy customization of transaction management.
- **breaking**: the way to create middleware is defined by the

     ```rust
     let data_sources = DataSources::with_default(db).await;
     let middleware = SeaOrmMiddleware::new(data_sources);
     ```
     change to
     ```rust
     let middleware = SeaOrmMiddleware::with_default(db);
     ```

- **breaking**: rename `Connection` struct to `DataSource`.
- **breaking**: change the definition of the `Error` enum.
- **breaking**: `commit` and `rollback` methods take parameters of type `Transaction`.
- **breaking**: rename `new_txn` method to `create_txn`.

## [0.6.0] 2024-08-03

- dep: update `sea-orm` 1.

## [0.5.2] 2024-05-22

- fix an issue where all requests shared a single data sources context.

## [0.5.1] 2024-05-03

- simplify code.

## [0.5.0] 2024-05-01

- remove `Connection::unmanaged_txn` method.
- add `commit`, `rollback`, `data_sources` functions.

## [0.4.0] 2024-04-30

- remove `Implicitxxx` and `Explicitxxx` types.
- add `SeaOrmMiddleware`, `DataSources`, `Connection`.

## [0.2.3] 2023-03-27

- downgrade `sea-orm` to 0.11.2.

## [0.2.2] 2023-03-26

- update to `sea-orm` 0.12.0.

## [0.2.1] 2023-02-14

- make `Tokio` optional.

## [0.2.0] 2023-02-13

- rename `TxnMiddleware` to `ImplicitDbMiddleware`.
- rename `TxnEndpoint` to `ImplicitDbEndpoint`.
- add `ExplicitDbMiddleware`, `ExplicitDbEndpoint`.

## [0.1.0] 2023-02-08

- add `LocalKeyExt`, `TXN`, `TxnMiddleware`, `TxnEndpoint`.
