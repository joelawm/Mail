/*-------------
/mail/inbox.rs

Inbox allows us to parse the letters from the inbox and add them to the Mail struct in a format we prefer.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use imap::types::{Fetch, ZeroCopy};
use mail_parser::MessageParser;
use super::{Letter, LetterBody, LetterInformation, Mail};

/// This function grabs the letters of the inbox and adds them to the Mail struct
/// 
/// # Arguments
/// * `msgs` - The Vec of Fetch structs.
/// * `mail` - The State of the Mail struct.
pub fn parse_letters(msgs: &ZeroCopy<Vec<Fetch>>, mail: &mut Mail) {
	for msg in msgs.iter() {
		let message = MessageParser::default().parse(msg.body().unwrap()).unwrap();

		let to = get_to(&message);
		let from = get_from(&message);
		let subject = get_subject(&message);
		let body = get_body(&message);

		let letter = Letter {from: from, to: to, subject: subject, body: body};

		// Add the letter to the Mail struct State
		mail.add_mail(letter);
	}
}

/// Get the to information from MessageParser.
/// # Arguments
/// * `message` - The Message we want to get the to from.
fn get_to(message: &mail_parser::Message) -> Vec<LetterInformation> {
	let to = match message.to() {
		Some(to) => to, 
		None => return Vec::new(),
	};

	to.iter().map(|to| {
		let address = match to.address() {
			Some(address) => address.to_string(),
			None => "(None)".to_string(),
		};
		let name = match to.name() {
			Some(name) => name.to_string(),
			None => "(None)".to_string(),
		};
		LetterInformation {address: address, name: name}
	}).collect::<Vec<LetterInformation>>()
}

/// Get the from information from MessageParser.
/// # Arguments
/// * `message` - The Message we want to get the from from.
fn get_from(message: &mail_parser::Message) -> Vec<LetterInformation> {
	let from = match message.from() {
		Some(from) => from,
		None => return Vec::new(),
	};

	from.iter().map(|from| {
		let address = match from.address() {
			Some(address) => address.to_string(),
			None => "(None)".to_string(),
		};
		let name = match from.name() {
			Some(name) => name.to_string(),
			None => "(None)".to_string(),
		};
		LetterInformation {address: address, name: name}
	}).collect::<Vec<LetterInformation>>()
}

/// Get the subject information from MessageParser.
/// # Arguments
/// * `message` - The Message we want to get the subject from.
fn get_subject(message: &mail_parser::Message) -> String {
	match message.subject() {
		Some(sub) => sub.to_string(),
		None => "(None)".to_string(),
	}
}

/// Get the body information from MessageParser.
/// # Arguments
/// * `message` - The Message we want to get the body from.
fn get_body(message: &mail_parser::Message) -> LetterBody {
	let body = match message.body_text(0) {
		Some(body) => body.to_string(),
		None => "(None)".to_string(),
	};

	let body_html = match message.body_html(0) {
		Some(body) => body.to_string(),
		None => "(None)".to_string(),
	};

	LetterBody {body: body, body_html: body_html}
}
