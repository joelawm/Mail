/*-------------
/client/json.rs

Client Json file is for facilitating the serialization of the ClientMail struct. This struct is used to send the client information and the mailbox to the front end.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use serde::Serialize;
use crate::mail::Letter;
use super::ClientInfo;

#[derive(Debug, Clone, Serialize)]
pub struct Startup {
	pub client_info: Vec<ClientInfo>,
	pub all_inboxes: Vec<Letter>
}
