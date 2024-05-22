# Changelog

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
