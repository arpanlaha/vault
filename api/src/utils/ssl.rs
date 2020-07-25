use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::env;

pub fn get_ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            env::var("VAULT_SSL_PRIVATE_KEY_PATH")
                .expect("VAULT_SSL_PRIVATE_KEY_PATH environment variable not found"),
            SslFiletype::PEM,
        )
        .unwrap();
    builder
        .set_certificate_chain_file(
            env::var("VAULT_SSL_CERT_PATH")
                .expect("VAULT_SSL_CERT_PATH environment variable not found"),
        )
        .unwrap();

    builder
}
