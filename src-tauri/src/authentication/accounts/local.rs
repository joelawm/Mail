/*-------------
/authentication/accounts/local.rs

Local accounts are the accounts that we have saved locally. This is for accounts that a OS does not save itself. We save them in secure storage and then use them to connect to the IMAP server.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use keyring::Entry;

/// Grab the local accounts from Keyring
/// # Arguments
/// * `email` - The email of the account we are trying to get.
fn get_local_accounts(email: &str) -> String {
	let entry = Entry::new("mail", email).unwrap();
    entry.get_password().unwrap()
}