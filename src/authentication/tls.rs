pub fn tls_connector() -> native_tls::TlsConnector {
    native_tls::TlsConnector::builder()
        .build()
        .expect("failed to build TLS connector")
}