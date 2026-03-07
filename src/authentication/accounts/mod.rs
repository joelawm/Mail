use crate::client::ClientInfo;

#[cfg(target_os = "linux")]
pub mod linux;

pub fn get_client_accounts() -> Vec<ClientInfo> {
    let mut accounts = Vec::new();

    #[cfg(target_os = "linux")]
    linux::get_gnome_accounts(&mut accounts);

    accounts
}