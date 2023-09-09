use sea_orm::{Database, DatabaseConnection};

pub async fn get_conn() -> DatabaseConnection {
    let db_url = "postgres://clogic@127.0.0.1/clogic";
    let conn = Database::connect(db_url).await.unwrap();
    return conn;
}
