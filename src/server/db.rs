pub type DbPool = sqlx::SqlitePool;

const DEFAULT_DB_URI: &str = "sqlite:db/analytics.db";

fn get_db_uri() -> String {
    match std::env::var("DATABASE_URL") {
        Err(_) => DEFAULT_DB_URI.to_string(),
        Ok( uri ) => uri,
    }
}

pub async fn connect_db() -> sqlx::Pool<sqlx::Sqlite> {
    let uri = get_db_uri();

    match DbPool::connect(&uri).await {
        Ok( db ) => {
            println!("Succesfully connected to db at {}", uri);
            db
        },
        Err( err ) => {
            println!("WARNING: could not connect to sqlite db, using failsafe in-memory db instead ({}).", err);

            connect_backup_db().await
        },
    }
}

pub async fn connect_backup_db() -> sqlx::Pool<sqlx::Sqlite> {
    DbPool::connect("sqlite::memory:").await.expect("Couldn't create in-memory db")
}