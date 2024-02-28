/*-------------
/authentication/accounts/mod.rs

Accounts allows us to grab the accounts based on the OS.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use super::ClientInfo;

pub mod linux;
pub mod local;

/// Get the OS Store Email Accounts
pub fn get_client_accounts() -> Vec<ClientInfo> {
	let mut accounts: Vec<ClientInfo> = Vec::new();

	// Gnome Online Accounts
	linux::get_gnome_accounts(&mut accounts);
	//get_local_accounts(&mut accounts);

	accounts
}
