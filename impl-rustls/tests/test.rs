use std::io;

#[test]
fn test_google() {
    tls_api_test::test_google::<tls_api_rustls::TlsConnector>()
}

#[test]
fn connect_bad_hostname() {
    let err = tls_api_test::connect_bad_hostname::<tls_api_rustls::TlsConnector>();
    let err: Box<io::Error> = err.into_inner().downcast().expect("io::Error");
    let err: &rustls::TLSError = err
        .get_ref()
        .expect("cause")
        .downcast_ref()
        .expect("rustls::TLSError");
    match err {
        rustls::TLSError::WebPKIError(webpki::Error::CertNotValidForName) => {}
        err => panic!("wrong error: {:?}", err),
    }
}

#[test]
fn connect_bad_hostname_ignored() {
    tls_api_test::connect_bad_hostname_ignored::<tls_api_rustls::TlsConnector>()
}

#[test]
fn server() {
    tls_api_test::server::<tls_api_rustls::TlsConnector, tls_api_rustls::TlsAcceptor>();
}

#[test]
fn alpn() {
    tls_api_test::alpn::<tls_api_rustls::TlsConnector, tls_api_rustls::TlsAcceptor>();
}
