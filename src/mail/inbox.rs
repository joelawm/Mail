use imap::types::{Fetch, ZeroCopy};
use mail_parser::MessageParser;

use super::{Letter, LetterBody, LetterInformation, Mail};

pub fn parse_letters(messages: &ZeroCopy<Vec<Fetch>>, mail: &mut Mail) {
    for message in messages.iter() {
        let Some(body) = message.body() else {
            continue;
        };
        let Some(parsed) = MessageParser::default().parse(body) else {
            continue;
        };
        let Some(id) = parsed.message_id() else {
            continue;
        };

        let letter = Letter {
            id: id.to_string(),
            from: get_people(parsed.from()),
            to: get_people(parsed.to()),
            bcc: get_people(parsed.bcc()),
            cc: get_people(parsed.cc()),
            date: parsed.date().map(|date| date.to_timestamp()).unwrap_or_default(),
            subject: parsed.subject().unwrap_or("(None)").to_string(),
            body: LetterBody {
                body: parsed.body_text(0).map(|body| body.into_owned()).unwrap_or_else(|| "(None)".to_string()),
                body_html: parsed.body_html(0).map(|body| body.into_owned()).unwrap_or_else(|| "(None)".to_string()),
            },
            flags: message.flags().iter().map(|flag| flag.to_string()).collect(),
        };

        mail.add_mail(letter);
    }
}

fn get_people(addresses: Option<&mail_parser::Address>) -> Vec<LetterInformation> {
    let Some(addresses) = addresses else {
        return Vec::new();
    };

    addresses
        .iter()
        .map(|address| LetterInformation {
            address: address.address.as_deref().unwrap_or_default().to_string(),
            name: address.name.as_deref().unwrap_or_default().to_string(),
        })
        .collect()
}