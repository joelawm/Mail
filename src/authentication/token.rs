pub struct Auth {
    pub user: String,
    pub access_token: String,
}

impl imap::Authenticator for Auth {
    type Response = String;

    fn process(&self, _: &[u8]) -> Self::Response {
        format!("user={}\x01auth=Bearer {}\x01\x01", self.user, self.access_token)
    }
}