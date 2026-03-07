use std::thread;

use crate::{authentication, authentication::domain::Domain, mail::{Letter, Mail}};

#[derive(Debug, Clone)]
pub struct Client {
    pub info: ClientInfo,
    pub mailbox: Vec<Mail>,
}

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub domain: Domain,
    pub email: String,
    pub access_token: Option<String>,
}

impl Client {
    pub fn new(client_info: ClientInfo) -> Self {
        Self {
            info: client_info,
            mailbox: Vec::new(),
        }
    }

    pub fn init(mut self) -> Self {
        let mut handles = Vec::new();

        for (mailbox_name, mailbox_clean_name) in self.info.domain.clone().get_domain_mailboxes() {
            let info = self.info.clone();
            let domain = info.domain.clone();

            handles.push(thread::spawn(move || {
                let mail = Mail::new(mailbox_name, mailbox_clean_name);
                let Ok(mut imap_session) = authentication::IMAPSession::try_new(domain, info) else {
                    return mail;
                };

                let initialized_mailbox = mail.init_mailbox(imap_session.get_session());
                imap_session.destroy_session();
                initialized_mailbox
            }));
        }

        for handle in handles {
            if let Ok(mailbox) = handle.join() {
                self.mailbox.push(mailbox);
            }
        }

        self.mailbox.sort_by(|left, right| left.mailbox_clean_name.cmp(&right.mailbox_clean_name));
        self
    }

    pub fn get_mailbox(&self, mailbox_name: &str) -> Vec<Letter> {
        self.mailbox
            .iter()
            .find(|mailbox| mailbox.mailbox_name == mailbox_name)
            .map(|mailbox| mailbox.letter.clone())
            .unwrap_or_default()
    }
}

impl ClientInfo {
    pub fn new(domain: Domain, email: String, access_token: Option<String>) -> Self {
        Self {
            domain,
            email,
            access_token,
        }
    }
}

pub fn load_clients() -> Vec<Client> {
    let mut handles = Vec::new();

    for account in authentication::accounts::get_client_accounts() {
        handles.push(thread::spawn(move || Client::new(account).init()));
    }

    let mut clients = Vec::new();
    for handle in handles {
        if let Ok(client) = handle.join() {
            clients.push(client);
        }
    }

    clients.sort_by(|left, right| left.info.email.cmp(&right.info.email));
    clients
}

pub fn collect_all_inboxes(clients: &[Client]) -> Vec<Letter> {
    let mut letters = clients
        .iter()
        .flat_map(|client| client.get_mailbox("INBOX"))
        .collect::<Vec<_>>();

    letters.sort_by(|left, right| right.date.cmp(&left.date));
    letters
}