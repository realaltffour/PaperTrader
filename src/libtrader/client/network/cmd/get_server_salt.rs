use ring::digest;

use std::io;

use crate::common::message::inst::CommandInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;
use crate::common::misc::return_flags::ReturnFlags;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;

/// Issues a command to the connected TLS server to obtain a salt.
///
/// All salts returned are of size ```digest::SHA512_OUTPUT_LEN/2```, 32 bytes.
/// Should be used in contexts that return ```io::Result```.
/// Should be used in Async contexts.
///
/// Arguments:
/// socket - The TLS stream to use for the salt.
///
/// Returns: a ```io::Result<[u8; 32]>```.
///
/// Example:
/// ```rust
///     let server_salt: [u8; digest::SHA512_OUTPUT_LEN/2] = get_server_salt(tls_client)?;
/// ```
pub async fn get_server_salt(
    socket: &mut TlsStream<TcpStream>,
) -> io::Result<[u8; digest::SHA512_OUTPUT_LEN / 2]> {
    /*
     * request to generate a salt from the server.
     * */
    let message = message_builder(
        MessageType::Command,
        CommandInst::GenHashSalt as i64,
        0,
        0,
        0,
        Vec::new(),
    );
    socket
        .write_all(&bincode::serialize(&message).unwrap())
        .await?;

    let mut buf = Vec::with_capacity(4096);
    socket.read_buf(&mut buf).await?;

    let ret_msg: Message = bincode::deserialize(&buf).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{}", ReturnFlags::ClientGenSaltFailed),
        )
    })?;

    if assert_msg(
        &ret_msg,
        MessageType::DataTransfer,
        true,
        1,
        false,
        0,
        true,
        1,
        true,
        digest::SHA512_OUTPUT_LEN / 2,
    ) {
        Ok(*array_ref!(ret_msg.data, 0, digest::SHA512_OUTPUT_LEN / 2))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}", ReturnFlags::ClientReqSaltInvMsg),
        ))
    }
}
