/*-------------
/accounts/domain.rs

Domain Enum allows use to set the domain for the email account. This facilitates specific mailbox names, domain names, and auth types.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum Domain {
	Gmail,
	Outlook,
	Yahoo,
	Failed,
}

impl Domain {
	/// This function returns the domain tuple for the imap and imap ports.
	pub fn get_domain_tuple(&self) -> (&'static str, u16) {
		match self {
			Domain::Gmail => ("imap.gmail.com", 993),
			Domain::Outlook => ("outlook.office365.com", 993),
			Domain::Yahoo => ("imap.mail.yahoo.com", 993),
			Domain::Failed => ("", 993),
		}
	}
	/// This function returns the domain mailboxes for the given domain.
	pub fn get_domain_mailboxes(&self) -> Vec<(String, String)> {
		match self {
			Domain::Gmail => vec![("[Gmail]/Sent Mail", "Sent"), ("[Gmail]/All Mail", "All Mail"), ("[Gmail]/Starred", "Starred"), ("[Gmail]/Trash", "Trash"), ("[Gmail]/Spam", "Spam"), ("[Gmail]/Drafts", "Drafts"), ("INBOX", "Inbox")].iter().map(|s| {(s.0.to_string(), s.1.to_string())}).collect(), 
			Domain::Outlook => vec![("Sent", "Sent")].iter().map(|s| {(s.0.to_string(), s.1.to_string())}).collect(),
			Domain::Yahoo => vec![("Sent", "Sent")].iter().map(|s| {(s.0.to_string(), s.1.to_string())}).collect(),
			Domain::Failed => vec![("", "")].iter().map(|s| {(s.0.to_string(), s.1.to_string())}).collect(),
		}
	}
	/// This function returns the domain auth type for the given domain.
	pub fn get_auth_type(&self) -> &'static str {
		match self {
			Domain::Gmail => "XOAUTH2",
			Domain::Outlook => "",
			Domain::Yahoo => "",
			Domain::Failed => "",
		}
	}
	/// This function returns the domain for the given domain string.
	/// # Arguments
	/// * `domain` - The domains imap string were trying to match.
	pub fn set_domain(domain: &str) -> Domain {
		match domain {
			"imap.gmail.com" => Domain::Gmail,
			"outlook.office365.com" =>  Domain::Outlook,
			"imap.mail.yahoo.com" => Domain::Yahoo,
			_ => Domain::Failed,
		}
	}
}