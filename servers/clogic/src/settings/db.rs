use sea_orm::{Database, DatabaseConnection};

pub async fn get_conn() -> DatabaseConnection {
    let db_url = "127.0.0.1:5432";
    let conn = Database::connect(db_url).await.unwrap();
    return conn;
}
