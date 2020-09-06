use crate::server::db::config::{DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS};
use crate::server::db::initializer::db_connect;
use crate::common::account::position::Position;
use crate::common::misc::return_flags::ReturnFlags;

/// Creates a position on the postgre SQL database
///
/// Takes in the position to insert to the database.
///
/// Arguments:
/// user_id - ID to create position for.
/// position - The position to use.
///
/// Example:
/// ```rust
///     match create_position(Position::default()) {
///         Ok(_) => {},
///         Err(err) => panic!("TEST_CMD_CREATE_PORTFOLIO_FAILED: {}", err)
///     }
/// ```
pub fn create_position(user_id: i64, position: Position) -> Result<(), ReturnFlags> {
    /*
     * Creates a position entry in database in portfolio_schema.positions.
     * */
    /* connect to database */
    let mut client = db_connect(DB_PORTFOLIO_USER, DB_PORTFOLIO_PASS)?;

    /* insert position */
    match client.execute("INSERT INTO portfolio_schema.positions 
                         (user_id, stock_symbol, stock_open_amount, stock_open_price, stock_open_cost, 
                         stock_close_amount, stock_close_price, open_epoch, close_epoch, is_buy, is_open) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)", 
                         &[&user_id, &position.stock_symbol, &position.stock_open_amount, &position.stock_open_price, 
                         &position.stock_open_cost, &position.stock_close_amount, &position.stock_close_price, 
                         &position.open_epoch, &position.close_epoch, &position.is_buy, &position.is_open]) {
        Ok(_rows) => Ok(()),
        Err(err) => Err(ReturnFlags::SERVER_DB_CREATE_POSITION_FAILED),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_create_position() {
        match create_position(1, Position::default()) {
            Ok(_) => {},
            Err(err) => panic!("TEST_CMD_CREATE_PORTFOLIO_FAILED: {}", err)
        }
    }
}
