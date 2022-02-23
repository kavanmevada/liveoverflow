use openssl::ssl::SslAcceptor;
use openssl::ssl::SslAcceptorBuilder;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;

pub fn builder() -> Result<SslAcceptorBuilder, openssl::error::ErrorStack> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(concat!(env!("OUT_DIR"), "/keystore.pem"), SslFiletype::PEM)?;
    builder.set_certificate_chain_file(concat!(env!("OUT_DIR"), "/certificate.pem"))?;
    Ok(builder)
}
