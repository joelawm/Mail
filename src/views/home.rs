use chrono::{Local, TimeZone};
use crate::{mail::Letter, mail_app};
use dioxus_native::prelude::*;

#[derive(Clone, PartialEq)]
enum MailboxScope {
    AllInboxes,
    Mailbox {
        email: String,
        mailbox_name: String,
    },
}

#[component]
pub fn Home() -> Element {
    let app_data = use_resource(move || async move {
        tokio::task::spawn_blocking(mail_app::load_app_data).await
    });
    let mut selected_scope = use_signal(|| MailboxScope::AllInboxes);
    let mut selected_letter_id = use_signal(|| None::<String>);

    let Some(app_data_result) = &*app_data.read_unchecked() else {
        return rsx! {
            div { class: "mail-shell",
                div { class: "mail-topbar",
                    div { class: "mail-topbar__identity",
                        p { class: "eyebrow", "Dioxus Desktop" }
                        h1 { "Mail" }
                        p { class: "subtle", "Loading accounts and mailboxes in the background." }
                    }
                }

                div { class: "empty-card empty-card--preview",
                    h3 { "Starting application" }
                    p { "The window is up. Mail sources are loading now, so the first launch may take a moment if IMAP needs to connect." }
                }
            }
        };
    };

    let Ok(app_data) = app_data_result else {
        return rsx! {
            div { class: "mail-shell",
                div { class: "mail-topbar",
                    div { class: "mail-topbar__identity",
                        p { class: "eyebrow", "Dioxus Desktop" }
                        h1 { "Mail" }
                        p { class: "subtle", "Startup hit a background task failure." }
                    }
                }

                div { class: "empty-card empty-card--preview",
                    h3 { "Startup failed" }
                    p { "A background mailbox-loading task panicked before the UI could finish initializing. Check the terminal output for details." }
                }
            }
        };
    };

    let current_scope = selected_scope.read().clone();
    let letters = letters_for_scope(app_data, &current_scope);
    let active_letter = selected_letter(&letters, selected_letter_id.read().clone());
    let active_letter_id = active_letter.as_ref().map(|letter| letter.id.clone());
    let unread_count = app_data
        .all_inboxes
        .iter()
        .filter(|letter| is_unread(letter))
        .count();
    let scope_heading = scope_title(app_data, &current_scope);

    let preview = match active_letter.clone() {
        Some(letter) => rsx! {
            div { class: "preview-shell",
                div { class: "pane-header pane-header--preview",
                    p { class: "eyebrow", "Preview" }
                    h2 { "{letter.subject}" }
                    p { class: "subtle", "From {display_sender(&letter)} to {display_recipient(&letter)}" }
                }

                if has_html_body(&letter) {
                    div {
                        class: "preview-html",
                        dangerous_inner_html: "{letter.body.body_html}"
                    }
                } else {
                    pre { class: "preview-text", "{letter.body.body}" }
                }
            }
        },
        None => rsx! {
            div { class: "empty-card empty-card--preview",
                h3 { "Choose a message" }
                p { "Select any message from the middle column to render its body here." }
            }
        },
    };

    rsx! {
        div { class: "mail-shell",
            div { class: "mail-topbar",
                div { class: "mail-topbar__identity",
                    p { class: "eyebrow", "Dioxus Desktop" }
                    h1 { "Mail" }
                    p { class: "subtle", "Version {app_data.version} on {app_data.target}" }
                }

                div { class: "mail-topbar__stats",
                    div { class: "stat-card",
                        span { class: "stat-card__label", "Accounts" }
                        strong { "{app_data.clients.len()}" }
                    }
                    div { class: "stat-card",
                        span { class: "stat-card__label", "Messages" }
                        strong { "{app_data.all_inboxes.len()}" }
                    }
                    div { class: "stat-card",
                        span { class: "stat-card__label", "Unread" }
                        strong { "{unread_count}" }
                    }
                }
            }

            div { class: "mail-body",
                aside { class: "mail-pane mail-pane--accounts",
                    div { class: "pane-header",
                        p { class: "eyebrow", "Mailboxes" }
                        h2 { "Sources" }
                    }

                    button {
                        class: if current_scope == MailboxScope::AllInboxes {
                            "mailbox-button mailbox-button--selected"
                        } else {
                            "mailbox-button"
                        },
                        onclick: move |_| {
                            selected_scope.set(MailboxScope::AllInboxes);
                            selected_letter_id.set(None);
                        },
                        span { class: "mailbox-button__title", "All Inboxes" }
                        span { class: "mailbox-button__meta", "{app_data.all_inboxes.len()} messages" }
                    }

                    if app_data.clients.is_empty() {
                        div { class: "empty-card",
                            h3 { "No accounts found" }
                            p { "This build reads Linux GNOME Online Accounts directly. Add a mail account there and relaunch the app." }
                        }
                    } else {
                        for client in &app_data.clients {
                            div { class: "account-card",
                                div { class: "account-card__header",
                                    h3 { "{client.info.email}" }
                                    p { "{client.mailbox.len()} mailboxes" }
                                }

                                div { class: "account-card__mailboxes",
                                    for mailbox in &client.mailbox {
                                        MailboxButton {
                                            title: mailbox.mailbox_clean_name.clone(),
                                            meta: format!("{} messages", mailbox.letter.len()),
                                            selected: current_scope == MailboxScope::Mailbox {
                                                email: client.info.email.clone(),
                                                mailbox_name: mailbox.mailbox_name.clone(),
                                            },
                                            scope: MailboxScope::Mailbox {
                                                email: client.info.email.clone(),
                                                mailbox_name: mailbox.mailbox_name.clone(),
                                            },
                                            selected_scope,
                                            selected_letter_id,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                section { class: "mail-pane mail-pane--messages",
                    div { class: "pane-header",
                        p { class: "eyebrow", "Current View" }
                        h2 { "{scope_heading}" }
                        p { class: "subtle", "{letters.len()} loaded messages" }
                    }

                    div { class: "message-list",
                        if letters.is_empty() {
                            div { class: "empty-card",
                                h3 { "Nothing to show" }
                                p { "This mailbox loaded successfully but did not return any messages." }
                            }
                        } else {
                            for letter in &letters {
                                MessageButton {
                                    letter: letter.clone(),
                                    selected: active_letter_id.as_ref() == Some(&letter.id),
                                    selected_letter_id,
                                }
                            }
                        }
                    }
                }

                section { class: "mail-pane mail-pane--preview", {preview} }
            }
        }
    }
}

fn letters_for_scope(app_data: &mail_app::AppData, scope: &MailboxScope) -> Vec<Letter> {
    match scope {
        MailboxScope::AllInboxes => app_data.all_inboxes.clone(),
        MailboxScope::Mailbox { email, mailbox_name } => app_data
            .clients
            .iter()
            .find(|client| client.info.email == *email)
            .map(|client| client.get_mailbox(mailbox_name))
            .unwrap_or_default(),
    }
}

fn selected_letter(letters: &[Letter], selected_id: Option<String>) -> Option<Letter> {
    selected_id
        .and_then(|id| letters.iter().find(|letter| letter.id == id).cloned())
        .or_else(|| letters.first().cloned())
}

fn scope_title(app_data: &mail_app::AppData, scope: &MailboxScope) -> String {
    match scope {
        MailboxScope::AllInboxes => "All Inboxes".to_string(),
        MailboxScope::Mailbox { email, mailbox_name } => app_data
            .clients
            .iter()
            .find(|client| client.info.email == *email)
            .and_then(|client| {
                client
                    .mailbox
                    .iter()
                    .find(|mailbox| mailbox.mailbox_name == *mailbox_name)
                    .map(|mailbox| format!("{} / {}", client.info.email, mailbox.mailbox_clean_name))
            })
            .unwrap_or_else(|| "Mailbox".to_string()),
    }
}

fn display_sender(letter: &Letter) -> &str {
    letter
        .from
        .first()
        .map(|sender| {
            if sender.name.is_empty() || sender.name == "(None)" {
                sender.address.as_str()
            } else {
                sender.name.as_str()
            }
        })
        .unwrap_or("Unknown sender")
}

fn display_recipient(letter: &Letter) -> &str {
    letter
        .to
        .first()
        .map(|recipient| {
            if recipient.name.is_empty() || recipient.name == "(None)" {
                recipient.address.as_str()
            } else {
                recipient.name.as_str()
            }
        })
        .unwrap_or("Unknown recipient")
}

fn preview_text(letter: &Letter) -> String {
    let preview = letter
        .body
        .body
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("No preview available")
        .trim();

    preview.chars().take(140).collect()
}

fn has_html_body(letter: &Letter) -> bool {
    let body = letter.body.body_html.trim();
    !(body.is_empty() || body == "(None)")
}

fn is_unread(letter: &Letter) -> bool {
    !letter.flags.iter().any(|flag| flag == "\\Seen")
}

fn format_timestamp(timestamp: i64) -> String {
    Local
        .timestamp_opt(timestamp, 0)
        .single()
        .map(|date| date.format("%b %d %H:%M").to_string())
        .unwrap_or_else(|| timestamp.to_string())
}

#[component]
fn MailboxButton(
    title: String,
    meta: String,
    selected: bool,
    scope: MailboxScope,
    selected_scope: Signal<MailboxScope>,
    selected_letter_id: Signal<Option<String>>,
) -> Element {
    rsx! {
        button {
            class: if selected {
                "mailbox-button mailbox-button--selected"
            } else {
                "mailbox-button"
            },
            onclick: move |_| {
                selected_scope.set(scope.clone());
                selected_letter_id.set(None);
            },
            span { class: "mailbox-button__title", "{title}" }
            span { class: "mailbox-button__meta", "{meta}" }
        }
    }
}

#[component]
fn MessageButton(
    letter: Letter,
    selected: bool,
    selected_letter_id: Signal<Option<String>>,
) -> Element {
    let message_id = letter.id.clone();

    rsx! {
        button {
            class: if selected {
                "message-card message-card--selected"
            } else {
                "message-card"
            },
            onclick: move |_| selected_letter_id.set(Some(message_id.clone())),
            div { class: "message-card__row",
                div { class: if is_unread(&letter) {
                    "message-dot message-dot--unread"
                } else {
                    "message-dot"
                } }
                h3 { "{display_sender(&letter)}" }
                span { class: "message-date", "{format_timestamp(letter.date)}" }
            }
            p { class: "message-subject", "{letter.subject}" }
            p { class: "message-preview", "{preview_text(&letter)}" }
        }
    }
}
