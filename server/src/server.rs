use std::net::{IpAddr, SocketAddr};

use wtransport::{tls::Certificate, Endpoint, ServerConfig};

use crate::{
    error::{ServerError, ServerSetupError},
    handler::{handle_bidirectional, handle_datagram, handle_unidirectional},
};

/// The configuration for the server.
/// This struct is used to configure the server.
///
/// # Fields
///
/// * `host` - The host to bind the server to.
/// * `port` - The port to bind the server to.
/// * `certificate_path` - The path to the certificate file.
/// * `certificate_key_path` - The path to the certificate key file.
pub struct PongServerConfig {
    pub host: IpAddr,
    pub port: u16,
    pub certificate_path: String,
    pub certificate_key_path: String,
}

/// The Pong server.
/// This struct is used to create and run the server.
///
/// # Fields
///
/// * `config` - The configuration for the server.
pub struct PongServer {
    config: PongServerConfig,
}

impl PongServer {
    /// Creates a new Pong server.
    /// This function will create a new Pong server with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the server.
    ///
    /// # Returns
    ///
    /// * `Self` - The created Pong server.
    pub fn new(config: PongServerConfig) -> Self {
        Self { config }
    }

    /// Asynchronously serve incoming connections.
    ///
    /// # Returns
    ///
    /// * `Result<(), ServerError>` - The result of running the server.
    pub async fn serve(&self) -> Result<(), ServerError> {
        let certificate = Certificate::load(
            &self.config.certificate_path,
            &self.config.certificate_key_path,
        )
        .map_err(|_| {
            ServerError::SetupError(ServerSetupError::CertificateSetupError {
                cert_path: self.config.certificate_path.clone(),
                key_path: self.config.certificate_key_path.clone(),
            })
        })?;

        let config = ServerConfig::builder()
            .with_bind_address(SocketAddr::new(self.config.host, self.config.port))
            .with_certificate(certificate);

        let server = Endpoint::server(config)
            .map_err(|_| ServerError::SetupError(ServerSetupError::EndpointCreationError))?;

        loop {
            println!("Waiting for incoming connection...");

            let maybe_acception = server.accept().await;

            if maybe_acception.is_none() {
                return Err(ServerError::ConnectionError(
                    common::error::ConnectionError::ClosedLocally,
                ));
            }

            tokio::spawn(async move {
                let connection = maybe_acception.unwrap().await.unwrap();

                println!("Waiting for data from client...");
                loop {
                    tokio::select! {
                        _ = handle_bidirectional(&connection) => {}
                        _ = handle_unidirectional(&connection) => {}
                        _ = handle_datagram(&connection) => {}
                    }
                }
            });
        }
    }
}