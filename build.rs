use std::{path::Path, fs::File, io::Write};

use openssl::{
    asn1::Asn1Time,
    bn::{BigNum, MsbOption},
    error::ErrorStack,
    hash::MessageDigest,
    pkey::{PKey, Private},
    rsa::Rsa,
    x509::{
        extension::{BasicConstraints, KeyUsage, SubjectKeyIdentifier},
        X509NameBuilder, X509,
    },
};

fn main() {
    // Compile SASS to CSS
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let sass = Path::new(&project_dir)
        .join("assets")
        .join("stylesheet.scss");
    //println!("cargo:rerun-if-changed={}", scss_path.as_path().display());

    if let Some(css) = sass.parent().map(|p| p.join("stylesheet.css")) {
        if let (Ok(mut file), Ok(compiled)) = (File::create(&css), sass::compile_file(&sass, sass::Options::default())) {
            file.write_all(compiled.as_bytes())
                .expect("Error writting css!");
        }
    }


    let out_dir = std::env::var("OUT_DIR").unwrap();
    //println!("cargo:rerun-if-changed={}", out_dir);
    if let Ok((cert, store)) = mk_ca_cert() {
        let cert = cert.to_pem().unwrap();
        let store = store.private_key_to_pem_pkcs8().unwrap();

        std::io::Write::write_all(
            &mut std::fs::File::create(std::path::PathBuf::from(&out_dir).join("keystore.pem"))
                .expect("Unable to create file"),
            &store,
        )
        .expect("Unable to write data");

        std::io::Write::write_all(
            &mut std::fs::File::create(std::path::PathBuf::from(&out_dir).join("certificate.pem"))
                .expect("Unable to create file"),
            &cert,
        )
        .expect("Unable to write data");
    }
}

fn mk_ca_cert() -> Result<(X509, PKey<Private>), ErrorStack> {
    let rsa = Rsa::generate(2048)?;
    let key_pair = PKey::from_rsa(rsa)?;

    let mut x509_name = X509NameBuilder::new()?;
    x509_name.append_entry_by_text("C", "US")?;
    x509_name.append_entry_by_text("ST", "TX")?;
    x509_name.append_entry_by_text("O", "Some CA organization")?;
    x509_name.append_entry_by_text("CN", "ca test")?;
    let x509_name = x509_name.build();

    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?;
    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(&x509_name)?;
    cert_builder.set_issuer_name(&x509_name)?;
    cert_builder.set_pubkey(&key_pair)?;
    let not_before = Asn1Time::days_from_now(0)?;
    cert_builder.set_not_before(&not_before)?;
    let not_after = Asn1Time::days_from_now(365)?;
    cert_builder.set_not_after(&not_after)?;

    cert_builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;
    cert_builder.append_extension(
        KeyUsage::new()
            .critical()
            .key_cert_sign()
            .crl_sign()
            .build()?,
    )?;

    let subject_key_identifier =
        SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;
    cert_builder.append_extension(subject_key_identifier)?;

    cert_builder.sign(&key_pair, MessageDigest::sha256())?;
    let cert = cert_builder.build();

    Ok((cert, key_pair))
}
