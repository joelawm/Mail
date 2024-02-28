/*-------------
/authentication/tls_connector.rs

TLS connector is a wrapper around the native_tls::TlsConnector. This is used to create a TLS connector for the client to use.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/

// Creates a TLS connector for the client to use.
pub fn tls_connector() -> native_tls::TlsConnector {
	native_tls::TlsConnector::builder().build().unwrap()
}
