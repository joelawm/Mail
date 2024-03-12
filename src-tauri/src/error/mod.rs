/*-------------
/error/mod.rs

Handles the error enum for the application.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use thiserror::Error;

enum Error {
	#[error("Internal Error (1100).")] // IMAP Error
	IMAPFetchError,
}