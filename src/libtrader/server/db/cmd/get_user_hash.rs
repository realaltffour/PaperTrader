use crate::common::misc::return_flags::ReturnFlags;

use crate::server::db::cmd::user_exists::user_exists;

pub async fn get_user_hash(
    sql_conn: &tokio_postgres::Client,
    username: &str,
    is_email: bool,
) -> Result<String, ReturnFlags> {
    /* check that user exists*/
    if user_exists(sql_conn, username).await {
        if is_email {
            for row in
                &sql_conn.query("SELECT username, email_hash FROM accounts_schema.accounts WHERE username LIKE $1", 
                              &[&username]).await.unwrap() {
                    return Ok(row.get(1));
                }
        } else {
            for row in
                &sql_conn.query("SELECT username, pass_hash FROM accounts_schema.accounts WHERE username LIKE $1", 
                              &[&username]).await.unwrap() {
                    return Ok(row.get(1));
                }
        }
    }

    Err(ReturnFlags::ServerDbUserHashNotFound)
}
