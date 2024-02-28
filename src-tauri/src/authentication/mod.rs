/*-------------
/authentication/mod.rs

Authentication is apart of the state of the application. It is the main struct that holds the IMAP session and the mailbox.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use std::net::TcpStream;
use native_tls::TlsStream;
use imap::Session;
use crate::client::ClientInfo;
use self::domain::Domain;

pub mod accounts;
mod token;
mod tls;
pub mod domain;

/// Encapsulates the IMAP session. 
pub struct IMAPSession {
	session: Session<TlsStream<TcpStream>>
}

impl IMAPSession {
	/// Creates a new IMAP session.
	pub fn new(domain: Domain, info: ClientInfo) -> Self {
		let domain_tuple = domain.get_domain_tuple();
		let tls_connector = tls::tls_connector();
		let client = imap::connect((domain_tuple.0, domain_tuple.1), domain_tuple.0, &tls_connector).unwrap();

		let auth = token::Auth {
			user: info.email.clone(),
			access_token: info.access_token.clone().unwrap(),
		};

		match client.authenticate(info.domain.get_auth_type(), &auth) {
			Ok(c) => return IMAPSession {session: c},
			Err((e, _unauth_client)) => {
				println!("error authenticating: {}", e);
				panic!("Failed error");
			}
		}
	}
	/// Grabs the newly created session.
	pub fn get_session(&mut self) -> &mut Session<TlsStream<TcpStream>> {
		&mut self.session
	}
	/// Destroys the session.
	pub fn destroy_session(&mut self) {
		self.session.logout().unwrap();
	}
}