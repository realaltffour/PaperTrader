use log::warn;
use std::io;

use crate::common::message::inst::DataTransferInst;
use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;
use crate::common::misc::return_flags::ReturnFlags;

use crate::server::account::retrieval_transaction::acc_retrieve_transaction;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn retrieve_transactions(
    sql_conn: &tokio_postgres::Client,
    tls_connection: &mut TlsStream<TcpStream>,
    message: &Message,
) -> std::io::Result<()> {
    /* assert recieved message */
    if !assert_msg(
        message,
        MessageType::DataTransfer,
        true,
        1,
        false,
        0,
        false,
        0,
        false,
        0,
    ) && message.instruction == DataTransferInst::GetUserTransactionHist as i64
        && message.data.len() != 0
    {
        warn!("RETRIEVE_TRANSACTION_INVALID_MESSAGE");
        return tls_connection.shutdown().await;
    }

    /* call acc_retrieve_transaction() server version */
    match acc_retrieve_transaction(sql_conn, tls_connection, message).await {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("RETRIEVE_TRANSACTION_FAILED: {}", err);
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{}: {}", ReturnFlags::ServerRetrieveTransactionFailed, err),
            ))
        }
    }
}
