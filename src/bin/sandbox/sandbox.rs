 #[macro_use] extern crate log;
extern crate simplelog;

use libtrader::initializer::libtrader_init;
use libtrader::db::cmd::create_company::create_company;
use libtrader::db::cmd::get_company::get_company_from_db;
use libtrader::db::cmd::create_stock::create_stock;
use libtrader::ds::generic::company::Company;
use libtrader::ds::server::global_state::GlobalState;
use libtrader::account::hash_pwd::{hash_pwd_client, hash_pwd_server};

use data_encoding::HEXUPPER;

fn main() {
    let mut state: GlobalState = match libtrader_init() {
        Ok(state) => {info!("inited state: {:?}\n", state); state},
        Err(err) => panic!("Failed with error: {}", err),
    };
    
    let mut company: Company = Company::default();
    company.id = 1;
    company.symbol = "TEST".to_string();
    company.isin = "TEST".to_string();
    company.company_name = "TEST".to_string();
    company.primary_exchange = "TEST".to_string();
    match create_company(&mut state, company) {
        Ok(()) => info!("created company"),
        Err(err) => error!("Failed to create company with error: {}", err),
    }

    match get_company_from_db(&mut state, "TEST".to_string()) {
        Ok(found_company) => info!("we found it! {:?}", found_company),
        Err(err) => error!("we must found the sacred company! err: {}", err),
    }

    match create_stock(&mut state, "test") {
        Ok(()) => info!("created stock table"),
        Err(err) => error!("failed to create stock table {}", err),
    }
   
    info!("state: {:?}\n", state);

    let mut enc = hash_pwd_client("this is my real password").unwrap();

    println!("Hash: {}", HEXUPPER.encode(&enc.0));
    println!("Salt: {}", HEXUPPER.encode(&enc.1));

    enc = hash_pwd_server(HEXUPPER.encode(&enc.0).as_str()).unwrap();

    println!("Hash: {}", HEXUPPER.encode(&enc.0));
    println!("Salt: {}", HEXUPPER.encode(&enc.1));
}

