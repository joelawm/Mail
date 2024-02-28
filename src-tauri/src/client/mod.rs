/*-------------
/client/mod.rs

Client is apart of the state of the application. It is the main struct that holds the IMAP session and the mailbox. 
The mailbox is a vector of Mail structs. The ClientInfo struct holds the domain, email, password, and access token.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use std::sync::Arc;
use serde::Serialize;
use tokio::{sync::Mutex, task::JoinSet};
use crate::authentication;
use crate::mail::Mail;
use crate::authentication::domain::Domain;

mod json;
pub mod commands;

#[derive(Debug, Clone)]
pub struct ClientState {
	pub client: Arc<Mutex<Vec<Client>>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Client {
	pub info: ClientInfo,
	pub mailbox: Vec<Mail>
}

#[derive(Debug, Serialize, Clone)]
pub struct ClientInfo {
	pub domain: Domain,
	pub email: String,
	#[serde(skip_serializing)]
	pub password: Option<String>,
	#[serde(skip_serializing)]
	pub access_token: Option<String>,
}

impl Client {
	pub fn new(client_info: ClientInfo) -> Self {
		Self {info: client_info, mailbox: Vec::new()}
	}

	/// Initialize the mailbox by grabbing all the mail for each one.
	/// # Arguments
	/// * `in_userid` - A i32 of the user ID.
	pub async fn init(mut self) -> Self {
		let domain = self.info.domain.clone().get_domain_mailboxes();

		let mut set = JoinSet::<Mail>::new();

		for mailbox in domain {
			let self_info = self.info.clone();
			let self_domain = self.info.domain.clone();
			
			set.spawn(async move {
				let mut imap_session = authentication::IMAPSession::new(self_domain, self_info);
				let mail = Mail::new(mailbox.to_string());
				let init = mail.init_mailbox(imap_session.get_session()).await;

				// Close the session and return the mailbox.
				imap_session.destroy_session();
				init
			});
		}

		while let Some(res) = set.join_next().await {
			match res {
				Ok(res) => self.create_mailbox(res),
				Err(e) => println!("Error getting mailbox: {}", e),
			};
		}

		self
	}

	/// Create the mailbox.
	/// # Arguments
	/// * `mailbox` - The mailbox to create.
	pub fn create_mailbox(&mut self, mailbox: Mail) {
		self.mailbox.push(mailbox)
	}

	/// Kills the idle session.
	pub async fn destroy_session(&mut self) {
		todo!()
	}
}

impl ClientInfo {
	pub fn new(domain: Domain, email: String, access_token: Option<String>, password: Option<String>) -> Self {
		Self {domain: domain, access_token: access_token, password: password, email: email}
	}
}

impl ClientState {
	pub fn default() -> Self {
		Self {client: Arc::new(Mutex::new(Vec::new()))}
	}
}