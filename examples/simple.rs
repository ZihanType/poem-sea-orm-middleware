use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};
use poem_sea_orm_middleware::{LocalKeyExt, TxnMiddleware, TXN};
use sea_orm::{entity::prelude::*, Database};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

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
