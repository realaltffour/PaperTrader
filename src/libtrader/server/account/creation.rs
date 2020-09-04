use data_encoding::HEXUPPER;

use crate::common::message::message::Message;
use crate::common::account::portfolio::Portfolio;
use crate::common::misc::return_flags::ReturnFlags;

use crate::server::account::hash_email::hash_email;
use crate::server::account::hash_pwd::hash_pwd;
use crate::server::ds::account::Account;
use crate::server::db::initializer::db_connect;
use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};

pub fn acc_create(message: &Message) -> Result<(), ReturnFlags> {
    /*
     * Parse account data
     * */
    /* get json data */
    let stringified_data = std::str::from_utf8(&message.data).unwrap().to_string();
    let data = json::parse(&stringified_data).unwrap();
    /* get email, password salts and client hashes */
    let email_hash = HEXUPPER.decode(data["email_hash"].as_str().unwrap().to_string().as_bytes()).unwrap();
    let email_client_salt = HEXUPPER.decode(data["email_client_salt"].as_str().unwrap().to_string().as_bytes()).unwrap();
    let password_hash = HEXUPPER.decode(data["password_hash"].as_str().unwrap().to_string().as_bytes()).unwrap();
    let password_client_salt = HEXUPPER.decode(data["password_client_salt"].as_str().unwrap().to_string().as_bytes()).unwrap();

    /* get username */
    let username: String = data["username"].as_str().unwrap().to_string();

    /* generate account struct */
    let mut account: Account = Account {
        username: username,

        email_hash: "".to_string(),
        server_email_salt: "".to_string(),
        client_email_salt: HEXUPPER.encode(&email_client_salt),

        pass_hash: "".to_string(),
        server_pass_salt: "".to_string(),
        client_pass_salt: HEXUPPER.encode(&password_client_salt),

        is_pass: true,
        portfolio: Portfolio::default(),
        transactions: Vec::new(),
    };

    /* 
     * check if username is available in the database 
     * */
    /* connect to database */
    let mut client = db_connect(DB_ACC_USER, DB_ACC_PASS)?;

    /* search for an account with same name */
    for _ in &client.query(
        "SELECT username FROM accounts_schema.accounts WHERE username LIKE $1", &[&account.username]).unwrap() {
        return Err(ReturnFlags::SERVER_ACC_USER_EXISTS);
    }

    /*
     * Hash the email and password.
     * */
    /* hash the email */
    let email_server_hash = hash_email(&email_hash);
    account.email_hash = HEXUPPER.encode(&email_server_hash.0);
    account.server_email_salt = HEXUPPER.encode(&email_server_hash.1);
    /* hash the password */
    let password_server_hash = hash_pwd(&password_hash);
    account.pass_hash = HEXUPPER.encode(&password_server_hash.0);
    account.server_pass_salt = HEXUPPER.encode(&password_server_hash.1);

    /*
     * Write the account to the database.
     * */
    match client.execute("INSERT INTO accounts_schema.accounts \
        (username, email_hash, server_email_salt, client_email_salt, pass_hash, server_pass_salt, client_pass_salt)
        VALUES \
        ($1, $2, $3, $4, $5, $6, $7)",
        &[&account.username, 
        &account.email_hash, &account.server_email_salt, &account.client_email_salt, 
        &account.pass_hash, &account.server_pass_salt, &account.client_pass_salt]) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(ReturnFlags::SERVER_DB_WRITE_FAILED),
    }
}
