use std::net::TcpStream;

use imap::Session;
use native_tls::TlsStream;

use crate::client::ClientInfo;

use self::domain::Domain;

pub mod accounts;
pub mod domain;
mod tls;
mod token;

pub struct IMAPSession {
    session: Session<TlsStream<TcpStream>>,
}

impl IMAPSession {
    pub fn try_new(domain: Domain, info: ClientInfo) -> Result<Self, String> {
        let domain_tuple = domain.get_domain_tuple();
        let access_token = info
            .access_token
            .clone()
            .ok_or_else(|| format!("missing access token for {}", info.email))?;
        let auth_type = domain.get_auth_type();
        if auth_type.is_empty() {
            return Err(format!("unsupported authentication type for {}", info.email));
        }

        let tls_connector = tls::tls_connector();
        let client = imap::connect((domain_tuple.0, domain_tuple.1), domain_tuple.0, &tls_connector)
            .map_err(|error| format!("failed to connect to {}: {error}", domain_tuple.0))?;

        let auth = token::Auth {
            user: info.email.clone(),
            access_token,
        };

        let session = client
            .authenticate(auth_type, &auth)
            .map_err(|(error, _)| format!("failed to authenticate {}: {error}", info.email))?;

        Ok(Self { session })
    }

    pub fn get_session(&mut self) -> &mut Session<TlsStream<TcpStream>> {
        &mut self.session
    }

    pub fn destroy_session(&mut self) {
        let _ = self.session.logout();
    }
}