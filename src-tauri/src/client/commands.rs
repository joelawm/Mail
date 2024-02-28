/*-------------
/client/commands.rs

Client Commands are commands that are used for the frontend to interact with the rust state via IPC.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use tauri::State;
use tokio::task::JoinSet;
use crate::authentication::accounts;
use crate::mail::Letter;
use super::{Client, Mail, ClientState};

#[tauri::command]
pub async fn client_connect(clients: State<'_, ClientState>) -> Result<Vec<Client>, ()> {
	let accounts = accounts::get_client_accounts();

	let mut set = JoinSet::<Client>::new();

	for account in accounts {
		set.spawn(async move {
			Client::new(account).init().await
		});
	}

	while let Some(res) = set.join_next().await {
		match res {
			Ok(res) => clients.client.lock().await.push(res),
			Err(e) => println!("Error getting mailbox: {}", e),
		};
	}

	let mut ret: Vec<Client> = Vec::new();
	for client in clients.client.lock().await.iter() {
		let mut mail: Vec<Mail> = Vec::new();

		for mailbox in &client.mailbox {
			if mailbox.mailbox_name == "INBOX" {
				mail.push(Mail { mailbox_name: mailbox.mailbox_name.clone(), letter: mailbox.letter.clone() });
			} else {
				mail.push(Mail { mailbox_name: mailbox.mailbox_name.clone(), letter: Vec::new() });
			}
		}
		ret.push(Client { info: client.info.clone(), mailbox: mail })
	}
	Ok(ret)
}

#[tauri::command]
/// Get all mailboxes
pub async fn get_all_inboxes(clients: State<'_, ClientState>) -> Result<Vec<Letter>, ()> {
	let mut letters: Vec<Letter> = Vec::new();

	for client in clients.client.lock().await.iter() {
		for mailbox in &client.mailbox {
			if mailbox.mailbox_name == "INBOX" {
				letters.append(&mut mailbox.letter.clone());
			}
		}
	}

	Ok(letters)
}

#[tauri::command]
/// Select a mailbox
pub async fn get_mailbox(email: String, mailbox: String, clients: State<'_, ClientState>) -> Result<Vec<Letter>, ()> {
	for client in clients.client.lock().await.iter() {
		if client.info.email == email {
			for mail_box in &client.mailbox {
				if mail_box.mailbox_name == mailbox {
					return Ok(mail_box.letter.clone());
				}
			}
		}
	}
	Ok(Vec::new())
}

#[tauri::command]
/// Disconnect the IMAP client
pub async fn client_disconnect(clients: State<'_, ClientState>) -> Result<(), ()> {
	clients.client.lock().await[0].destroy_session().await;
	Ok(())
}