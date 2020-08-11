#[allow(unused_imports)]
use crate::misc::gen_log::gen_log;
use crate::misc::path_exists::path_exists;
use crate::db::initializer::db_init;
use crate::ds::generic::global_state::GlobalState;

/// Initializes global logger.
///
/// Private function used by libtrader_init() to initialize the logger. Log destinations are
/// platfrom dependent.
/// On unix systems: /var/log/papertrader/
/// On windows/unkown systems: $(pwd)/log/
///
/// Returns: nothing on success, on error contains the reason of failure.
///
/// Example:
/// ```rust
///     match libtrader_init_log() {
///         Ok(()) => {},
///         Err(err) => panic!("failed initializing log, reason: {}", err)
///     };
/// ```
///
fn libtrader_init_log() -> Result<(), String> {
    info!("Started Logger.");
    #[cfg(not(debug_assertions))]
    gen_log();

    #[cfg(debug_assertions)] {
        use simplelog::*;
        use std::fs::File;

        if !path_exists("log") {
            match std::fs::create_dir("log") {
                Ok(()) => {},
                Err(err) => panic!("GEN_LOG_FAILED_DIR_CREATION: {}", err)
            };
        }
        CombinedLogger::init(vec![
                             #[cfg(debug_assertions)]
                             TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
                             #[cfg(not(debug_assertions))]
                             TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
                             WriteLogger::new(LevelFilter::Info, Config::default(), 
                                              File::create(format!("log/log-{}.txt", 
                                                                   chrono::Utc::now().to_rfc2822())).unwrap())
        ]).unwrap();
    };

    Ok(())
}

/// Generic Initialization of the library.
///
/// Public function that globaly initializes the library. Initializes log, and database.
///
/// Returns: ``GlobalState``` on success, and a string containing the reason
/// of failure.
///
/// Example:
/// ```rust
///     match libtrader_init() {
///         Ok(state) => println!("here is the initialized state: {}", state),
///         Err(err) => panic!("failed initializing libtrader, reason: {}", err)
///     };
/// ```
pub fn libtrader_init() -> Result<GlobalState, String> {
    let mut state: GlobalState = GlobalState::default();

    // Initialize log.
    #[cfg(not(test))]
    match libtrader_init_log() {
        Ok(()) => {},
        Err(err) => panic!("This should not happen!\n{}", err),
    };

    // Initialize database.
    match db_init(&mut state) {
        Ok(()) => info!("Initialized database."),
        Err(err) => return Err(format!("INIT_DB_FAILED: {}", err))
    }

    Ok(state)
}

/// Client Initialization of the library.
///
/// Public function that initializes the library, and connects to a libtrader server
/// This funciton should not return.
///
/// Example:
/// ```rust
///     libtrader_init_client()?;
/// ```
pub fn libtrader_init_client() -> Result<GlobalState, String> {
    use mio::net::TcpStream;

    use crate::network::tls_client::TlsClient;
    use crate::misc::gen_tls_client_config::gen_tls_client_config;
    use crate::misc::lookup_ipv4::lookup_ipv4;

    let addr = lookup_ipv4("0.0.0.0", 4000);
    let config = gen_tls_client_config();
    
    let sock = match TcpStream::connect(addr) {
        Ok(socket) => socket,
        Err(err) => {
            error!("LIBTRADER_INIT_CLIENT_CONNECT_FAILED: {}", err);
            return Err("could not connect to server!".to_string());
        }
    };
    let dns_name = webpki::DNSNameRef::try_from_ascii_str("localhost").unwrap();
    let mut tls_client = TlsClient::new(sock, dns_name, config);

    let mut poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(32);
    tls_client.register(poll.registry());

    loop {
        poll.poll(&mut events, None).unwrap();

        for ev in &events {
            tls_client.ready(&ev);
            tls_client.reregister(poll.registry());

            if ev.token() == mio::Token(0) && ev.is_writable() {
                use crate::account::acc_creation::acc_create;
                match acc_create(&mut tls_client, &mut  poll, "test", "test", "test") {
                    Ok(()) => println!("server returned yes"),
                    Err(err) => panic!("panik {}", err),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::db::config::{DB_USER, DB_PASS};
    use crate::db::initializer::{db_connect};
    use crate::ds::generic::global_state::GlobalState;
    use crate::ds::generic::company::Company;

   use super::*;

    #[test]
    fn test_libtrader_init_log() {
        match libtrader_init_log() {
            Ok(()) => {},
            Err(err) => panic!("TEST_INIT_LOG_FAILED: {}", err)
        };
    }

    #[test]
    fn test_libtrader_init() {
        /* connect to db */
        let mut state: GlobalState = GlobalState::default();
        let mut client = db_connect(&mut state, DB_USER, DB_PASS).unwrap();

        /* add test compnay */
        let mut company = Company::default();
        company.id = 1234;
        company.symbol = "CPP".to_string();
        company.isin = "2".to_string();
        company.company_name = "CPP".to_string();
        company.primary_exchange = "NYSE".to_string();
        company.sector = "Tech".to_string();
        company.industry = "Tech".to_string();
        company.primary_sic_code = "2".to_string();
        company.employees = 1;
        client.execute(
            "INSERT INTO public.companies VALUES ($1,$2, $3, $4, $5, $6, $7, $8, $9)",
            &[&company.id, &company.symbol, &company.isin, &company.company_name, 
            &company.primary_exchange, &company.sector, &company.industry,
            &company.primary_sic_code, &company.employees]).unwrap();

        /* test libtrader_init */
        match libtrader_init() {
            Ok(state) => assert_eq!(state.companies.is_empty(), false),
            Err(err) => panic!("TEST_INIT_FAILED: {}", err)
        }
    }
}