#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Domain {
    Gmail,
    Outlook,
    Yahoo,
    Failed,
}

impl Domain {
    pub fn get_domain_tuple(&self) -> (&'static str, u16) {
        match self {
            Domain::Gmail => ("imap.gmail.com", 993),
            Domain::Outlook => ("outlook.office365.com", 993),
            Domain::Yahoo => ("imap.mail.yahoo.com", 993),
            Domain::Failed => ("", 993),
        }
    }

    pub fn get_domain_mailboxes(&self) -> Vec<(String, String)> {
        match self {
            Domain::Gmail => vec![
                ("[Gmail]/Sent Mail", "Sent"),
                ("[Gmail]/All Mail", "All Mail"),
                ("[Gmail]/Starred", "Starred"),
                ("[Gmail]/Trash", "Trash"),
                ("[Gmail]/Spam", "Spam"),
                ("[Gmail]/Drafts", "Drafts"),
                ("INBOX", "Inbox"),
            ],
            Domain::Outlook => vec![("Inbox", "Inbox"), ("Sent", "Sent")],
            Domain::Yahoo => vec![("Inbox", "Inbox"), ("Sent", "Sent")],
            Domain::Failed => Vec::new(),
        }
        .into_iter()
        .map(|(mailbox_name, mailbox_clean_name)| {
            (mailbox_name.to_string(), mailbox_clean_name.to_string())
        })
        .collect()
    }

    pub fn get_auth_type(&self) -> &'static str {
        match self {
            Domain::Gmail => "XOAUTH2",
            Domain::Outlook => "XOAUTH2",
            Domain::Yahoo => "XOAUTH2",
            Domain::Failed => "",
        }
    }

    pub fn set_domain(domain: &str) -> Domain {
        match domain {
            "imap.gmail.com" => Domain::Gmail,
            "outlook.office365.com" => Domain::Outlook,
            "imap.mail.yahoo.com" => Domain::Yahoo,
            _ => Domain::Failed,
        }
    }
}