use either::*;

use crate::network::tls_connection::TlsConnection;
use crate::network::tls_client::TlsClient;

pub fn handle_data(conn: Either<&mut TlsConnection, &mut TlsClient>, buf: &[u8]) -> 
Result<(), String> {
    use crate::ds::message::message::Message;
    use crate::ds::message::inst::CommandInst;
    
    use crate::network::cmd::server::register::register;

    /* keep CocRust happy */
    assert_eq!(conn.is_left(), true);
    let connection = conn.left().unwrap();

    /* decode incoming message */
    let client_response: Message = match bincode::deserialize(&buf) {
        Ok(msg) => msg,
        Err(err) => {
            warn!("HANDLE_DATA_RCVD_INV_MSG: {}", err); 
            connection.closing = true; /* disconnect any unrecognized message senders */
            return Ok(());
        }
    };

    /* handle individual client instructions */
    match client_response.instruction {
        _ if client_response.instruction == CommandInst::Register as i64 => 
            register(connection, &client_response),
        _ => {}
    };
        
    Ok(())
}