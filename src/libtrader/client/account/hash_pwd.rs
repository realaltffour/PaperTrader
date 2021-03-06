use ring::rand::SecureRandom;
use ring::{digest, rand};

use crate::common::account::hash::hash;

/// Generates a client password hash from a raw password.
///
/// Takes in a raw password, outputs a hashed version of the client password to be sent to the
/// server with the returned client random bits that make up the whole client salt. This function
/// is to be used on client side account creation. The result from this function is not be stored
/// directly on the database, result must be run through the server side hashing again.
///
/// Arguments:
/// pass - The raw user password to be hashed.
/// server_salt - The server's part sent of the salt.
///
/// Returns: a tuple containing the client hash and full salt, nothing on failure.
///
/// Example:
/// ```rust
///     let enc = hash_pwd("this is my real password!", server_salt).unwrap();
///     println!("Client Pass Hash: {}", HEXUPPER.encode(&enc.0));
///     println!("Client Pass Salt: {}", HEXUPPER.encode(&enc.1));
/// ```
pub fn hash_pwd(
    pass: &Vec<u8>,
    server_salt: [u8; digest::SHA512_OUTPUT_LEN / 2],
) -> (
    [u8; digest::SHA512_OUTPUT_LEN],
    [u8; digest::SHA512_OUTPUT_LEN],
) {
    // client hash, full salt
    let rng = rand::SystemRandom::new();

    let mut client_salt = [0u8; digest::SHA512_OUTPUT_LEN / 2];
    rng.fill(&mut client_salt).unwrap();

    let salt = [server_salt, client_salt].concat();

    let hash = hash(pass, &salt, 250_000);

    (hash, *array_ref!(salt, 0, digest::SHA512_OUTPUT_LEN))
}

#[cfg(test)]
mod test {
    use super::*;
    use data_encoding::HEXUPPER;

    #[test]
    fn test_account_hash_pwd_client() {
        let pass = "goodlilpassword";

        /* generate server salt */
        let rng = rand::SystemRandom::new();
        let mut server_salt = [0u8; digest::SHA512_OUTPUT_LEN / 2];
        rng.fill(&mut server_salt).unwrap();

        /* ensure that hash_pwd_client() works */
        let output = hash_pwd(&pass.as_bytes().to_vec(), server_salt);
        assert_ne!(output.0.len(), 0);
        assert_ne!(output.1.len(), 0);

        /* ensure that hash_pwd_client() doesn't generate same output
         * with the same server salt.
         * */
        let mut enc0 = hash_pwd(&pass.as_bytes().to_vec(), server_salt);
        let mut enc1 = hash_pwd(&pass.as_bytes().to_vec(), server_salt);
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));

        /* ensure that hash_pwd_client() generates different output
         * with different server salts.
         * */
        // Generate new server salt.
        let mut server_salt2 = [0u8; digest::SHA512_OUTPUT_LEN / 2];
        rng.fill(&mut server_salt2).unwrap();

        enc0 = hash_pwd(&pass.as_bytes().to_vec(), server_salt);
        enc1 = hash_pwd(&pass.as_bytes().to_vec(), server_salt);
        assert_ne!(HEXUPPER.encode(&enc0.0), HEXUPPER.encode(&enc1.0));
        assert_ne!(HEXUPPER.encode(&enc0.1), HEXUPPER.encode(&enc1.1));
    }
}
