use crate::{authentication::domain::Domain, client::ClientInfo};

pub fn get_gnome_accounts(accounts: &mut Vec<ClientInfo>) {
    for account in gnome_online_accounts_rs::get_accounts() {
        let Some(mail) = account.mail else {
            continue;
        };

        let Some(access_token) = gnome_online_accounts_rs::get_token(account.id.as_str()) else {
            continue;
        };

        let domain = Domain::set_domain(mail.imap_host.as_str());
        if domain == Domain::Failed {
            continue;
        }

        accounts.push(ClientInfo::new(domain, mail.email, Some(access_token)));
    }
}