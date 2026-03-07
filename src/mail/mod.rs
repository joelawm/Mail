use std::net::TcpStream;

use imap::Session;
use native_tls::TlsStream;

mod inbox;

#[derive(Debug, Clone, PartialEq)]
pub struct Mail {
    pub mailbox_name: String,
    pub mailbox_clean_name: String,
    pub letter: Vec<Letter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Letter {
    pub id: String,
    pub from: Vec<LetterInformation>,
    pub to: Vec<LetterInformation>,
    pub bcc: Vec<LetterInformation>,
    pub cc: Vec<LetterInformation>,
    pub date: i64,
    pub subject: String,
    pub body: LetterBody,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetterInformation {
    pub address: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetterBody {
    pub body: String,
    pub body_html: String,
}

impl Mail {
    pub fn new(name: String, clean_name: String) -> Self {
        Self {
            mailbox_name: name,
            mailbox_clean_name: clean_name,
            letter: Vec::new(),
        }
    }

    pub fn init_mailbox(mut self, imap_session: &mut Session<TlsStream<TcpStream>>) -> Self {
        if imap_session.select(self.mailbox_name.as_str()).is_err() {
            return self;
        }

        if let Ok(messages) = imap_session.fetch("1:*", "(FLAGS BODY.PEEK[])") {
            inbox::parse_letters(&messages, &mut self);
        }

        self.letter.sort_by(|left, right| right.date.cmp(&left.date));
        self
    }

    pub fn add_mail(&mut self, mail: Letter) {
        self.letter.push(mail);
    }
}