use crate::server::db::config::{DB_HOST, DB_HOST_PORT, DB_NAME};

/// Establishes a postgresql connection to the SQL database.
///
/// Creates a postgresql connection.
///
/// Arguments:
/// user - The name of the user to connect to the database with.
/// pass - The password of the user to connect to the database with.
///
/// Returns: ```postgres::Client``` on success, and a string containing the
/// reason of failure on error.
///
/// Example:
/// ```rust
/// let mut client = db_connect(DB_USER, DB_PASS)?;
/// ```
pub async fn db_connect(user: &'static str, pass: &'static str) -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    /* Generate the requested string */
    let db_connect_str = format!(
        "host={} port={} dbname={} user={} password={}",
        DB_HOST, DB_HOST_PORT, DB_NAME, user, pass
    );
    let (client, connection) = tokio_postgres::connect(db_connect_str.as_str(), tokio_postgres::NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("SQL connection error: {}", e);
        }
    });
    Ok(client)
}
