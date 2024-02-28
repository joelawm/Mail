/*-------------
/authentication/token.rs

Token File is for the authentication of the IMAP server. This is used to authenticate the user with the server.
This is a simple struct that holds the user and the access token.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/

pub struct Auth {
	pub user: String,
	pub access_token: String,
}

impl imap::Authenticator for Auth {
	type Response = String;

	/// Process the authentication.
	/// # Arguments
	/// * `data` - A slice of u8.
	fn process(&self, _: &[u8]) -> Self::Response {
		format!("user={}\x01auth=Bearer {}\x01\x01", self.user, self.access_token)
	}
}