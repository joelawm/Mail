use std::env;

use crate::{client, client::Client, mail::Letter};

#[derive(Clone)]
pub struct AppData {
    pub clients: Vec<Client>,
    pub all_inboxes: Vec<Letter>,
    pub version: &'static str,
    pub target: &'static str,
}

pub fn load_app_data() -> AppData {
    let clients = client::load_clients();
    let all_inboxes = client::collect_all_inboxes(&clients);

    AppData {
        clients,
        all_inboxes,
        version: env!("CARGO_PKG_VERSION"),
        target: env::consts::OS,
    }
}