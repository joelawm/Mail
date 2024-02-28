/*-------------
/authentication/accounts/local.rs

Local accounts are the accounts that we have saved locally. This is for accounts that a OS does not save itself. We save them in secure storage and then use them to connect to the IMAP server.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use crate::client::ClientInfo;

/// Grab the local accounts from Keyring
/// # Arguments
/// * `accounts` - The accounts vector to add the local accounts to.
fn get_local_accounts(accounts: &mut Vec<ClientInfo>) {
	todo!("Get the local accounts");
}