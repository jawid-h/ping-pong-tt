use base64::engine::general_purpose::STANDARD as Base64Engine;
use base64::Engine;
use rcgen::CertificateParams;
use rcgen::DistinguishedName;
use rcgen::DnType;
use rcgen::KeyPair;
use rcgen::PKCS_ECDSA_P256_SHA256;
use ring::digest::digest;
use ring::digest::SHA256;
use std::error::Error;
use std::fs;
use std::io::Write;
use time::Duration;
use time::OffsetDateTime;

/// Generates a certificate and private key for use in tests.
/// The code copied from the original `wttransport` crate repository.
///
/// # Arguments
///
/// * `cert_path` - The path to the certificate file.
/// * `key_path` - The path to the certificate key file.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - The result of generating the certificate.
pub fn gen_certs(cert_path: String, key_path: String) -> Result<(), Box<dyn Error>> {
    const COMMON_NAME: &str = "localhost";

    let mut dname = DistinguishedName::new();
    dname.push(DnType::CommonName, COMMON_NAME);

    let keypair = KeyPair::generate(&PKCS_ECDSA_P256_SHA256)?;

    let digest = digest(&SHA256, &keypair.public_key_der());

    let mut cert_params = CertificateParams::new(vec![COMMON_NAME.to_string()]);

    cert_params.distinguished_name = dname;
    cert_params.alg = &PKCS_ECDSA_P256_SHA256;
    cert_params.key_pair = Some(keypair);
    cert_params.not_before = OffsetDateTime::now_utc()
        .checked_sub(Duration::days(2))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not add 2 days to the current date",
            )
        })?;
    cert_params.not_after = OffsetDateTime::now_utc()
        .checked_add(Duration::days(2))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not add 2 days to the current date",
            )
        })?;

    let certificate = rcgen::Certificate::from_params(cert_params)?;

    fs::File::create(cert_path)?.write_all(certificate.serialize_pem()?.as_bytes())?;
    fs::File::create(key_path)?.write_all(certificate.serialize_private_key_pem().as_bytes())?;

    println!("Certificate generated");
    println!("Fingerprint: {}", Base64Engine.encode(digest));

    Ok(())
}
