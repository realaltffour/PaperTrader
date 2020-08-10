extern crate log;
extern crate simplelog;

#[cfg(feature="server")]
use libtrader::initializer::libtrader_init_server;
#[cfg(feature="client")]
use libtrader::initializer::libtrader_init_client;

fn main() {
    #[cfg(feature="server")]
    libtrader_init_server().unwrap();

    #[cfg(feature="client")]
    libtrader_init_client().unwrap();
}

