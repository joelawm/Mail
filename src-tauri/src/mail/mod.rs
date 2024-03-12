/*-------------
/mail/mod.rs

Mail is apart of the state of the application. It is the main struct that holds the mailbox and the letters.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use std::net::TcpStream;
use imap::Session;
use native_tls::TlsStream;
use serde::{Deserialize, Serialize};

mod inbox;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mail {
	pub mailbox_name: String,
	pub letter: Vec<Letter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Letter {
	pub from: Vec<LetterInformation>,
	pub to: Vec<LetterInformation>,
	pub bcc: Vec<LetterInformation>,
	pub cc: Vec<LetterInformation>,
	pub date: i64,
	pub subject: String,
	pub body: LetterBody,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LetterInformation {
	pub address: String,
	pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LetterBody {
	pub body: String,
	pub body_html: String,
}

impl Mail {
	/// Create a default Mail struct
	pub fn new(name: String) -> Self {
		Self {mailbox_name: name, letter: Vec::new()}
	}

	/// Export the Mail struct to be sent to frontend
	fn export_mail(&self) -> Vec<Letter> {
		todo!()
	}

	/// Import the Mail struct from the frontend
	/// # Arguments
	/// * `mail` - The Vec of Letters to import.
	fn import_mail(&self, mail: Vec<Letter>) {
		todo!()
	}

	/// Fill the mailbox with the letters from the imap session.
	/// # Arguments
	/// * `imap_session` - The imap session to fill the mailbox with.
	pub async fn init_mailbox(mut self, imap_session: &mut Session<TlsStream<TcpStream>>) -> Self {
		match imap_session.select(self.mailbox_name.as_str()) {
			Ok(_) => {},
			Err(e) => println!("Error selecting mailbox: {}", e)
		}

		let mut ptr_beg = 1;
		let mut ptr_end = 15;

		loop {
			match imap_session.fetch(format!("{}:{}", ptr_beg, ptr_end), "BODY[]") {
				Ok(msgs) => {
					inbox::parse_letters(&msgs, &mut self);

					if msgs.len() == 0 {
						break;
					} else {
						break;
						// Removing these will make the application download all the data
						// ptr_beg = ptr_end;
						// ptr_end += 15;
					}
				},
				Err(e) => println!("Error Fetching email: {}", e)
			};
		}

		// Sort the mail by date
		self.letter.sort_by(|a, b| b.date.cmp(&a.date));

		self.to_owned()
	}

	/// Add a mail to the Mail struct
	/// # Arguments
	/// * `mail` - The Letter to add to the Mail struct.
	pub fn add_mail(&mut self, mail: Letter) {
		self.letter.push(mail);
	}

	/// Remove a mail from the Mail struct
	/// # Arguments
	/// * `index` - The index of the Letter to remove from the Mail struct.
	pub fn remove_mail(&mut self, index: usize) {
		self.letter.remove(index);
	}
}
