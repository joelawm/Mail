/*-------------
/authentication/accounts/linux.rs

Linux accounts are the accounts related to the Linux OS. We do not handle the storage of the accounts, we're just simply grabbing the accounts from the OS and adding them to the accounts vector.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use crate::{client::ClientInfo, authentication::domain::Domain};

/// Grab the gnome Online Accounts
/// # Arguments
/// * `accounts` - The accounts vector to add the gnome accounts to.
pub fn get_gnome_accounts(accounts: &mut Vec<ClientInfo>) {
	for account in gnome_online_accounts_rs::get_accounts() {
		if account.mail.is_some() {
			let mail = account.mail.unwrap();

			// Settings
			let email = mail.email;
			let access_token = gnome_online_accounts_rs::get_token(account.id.as_str()).unwrap();
			let domain = Domain::set_domain(mail.imap_host.as_str());

			let client_info = ClientInfo::new(domain, email, Some(access_token), None);
			accounts.push(client_info);
		}
	}
}
