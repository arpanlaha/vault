use dotenv_codegen::dotenv;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

pub fn get_ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(dotenv!("SSL_PRIVATE_KEY_PATH"), SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(dotenv!("SSL_CERT_PATH"))
        .unwrap();

    builder
}
