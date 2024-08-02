/**
 * Package is the structure of a build specification for the build system.
 * Mirrors the package_version.rs file.
 */

export interface Client {
	info: ClientInfo,
	mailbox: [{letter: [Mail], mailbox_name: string, mailbox_clean_name: string}]
}

export interface ClientInfo {
	domain: string,
	email: string
}

export interface Mail {
	id: string,
	from: [{ address: string, name: string }],
	to: [{ address: string, name: string }],
	bcc: [{ address: string, name: string }],
	cc: [{ address: string, name: string }],
	date: BigInt,
	subject: string,
	body: {body: string, body_html: string},
	flags: [string],
}

export default Mail;