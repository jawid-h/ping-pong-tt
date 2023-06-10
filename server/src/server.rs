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
        // Build the server configuration.
        // The configuration is happening here due to limitations of `wttransport` crate
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
                        _ = handle_bidirectional(&connection) => {
                            println!("Connection closed by client");
                            break;
                        }
                        _ = handle_unidirectional(&connection) => {
                            println!("Connection closed by client");
                            break;
                        }
                        _ = handle_datagram(&connection) => {}
                    }
                }
            });

            // Exit the loop if we are running tests.
            if cfg!(test) {
                break Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::{self};

    use client::client::{PingClient, PingClientConfig, PingClientConnectionType};
    use common::message::Message;

    use super::*;

    #[tokio::test]
    async fn test_integration_send_recieve_bidirectional() {
        let mut cert_path = env::temp_dir();
        cert_path.push("cert.pem");

        let mut key_path = env::temp_dir();
        key_path.push("key.pem");

        let pong_server_config = PongServerConfig {
            host: "127.0.0.1"
                .parse()
                .expect("failed to parse host for the server"),
            port: 4433,
            certificate_path: cert_path.to_str().unwrap().to_string(),
            certificate_key_path: key_path.to_str().unwrap().to_string(),
        };

        let pong_server = PongServer::new(pong_server_config);

        let ping_client_config = PingClientConfig {
            host: "127.0.0.1"
                .parse()
                .expect("failed to parse host for the server"),
            port: 4433,
            connection_type: PingClientConnectionType::Bidirectional,
            max_retries: 3,
            retry_timeout_millis: 1000,
        };

        let mut ping_client = PingClient::new(ping_client_config);

        let times = Some(3);

        let message = Message::new_request("Ping!".to_string());

        let (_, _) = tokio::join!(
            pong_server.serve(),
            ping_client.send_message(&message, times)
        );

        let inbox = ping_client.get_indbox();

        assert_eq!(inbox.len(), 3);
        for message in inbox {
            assert_eq!(message.get_data(), "Pong!");
        }
    }

    #[tokio::test]
    async fn test_integration_send_recieve_unidirectional() {
        let mut cert_path = env::temp_dir();
        cert_path.push("cert.pem");

        let mut key_path = env::temp_dir();
        key_path.push("key.pem");

        let pong_server_config = PongServerConfig {
            host: "127.0.0.1"
                .parse()
                .expect("failed to parse host for the server"),
            port: 4434,
            certificate_path: cert_path.to_str().unwrap().to_string(),
            certificate_key_path: key_path.to_str().unwrap().to_string(),
        };

        let pong_server = PongServer::new(pong_server_config);

        let ping_client_config = PingClientConfig {
            host: "127.0.0.1"
                .parse()
                .expect("failed to parse host for the server"),
            port: 4434,
            connection_type: PingClientConnectionType::Unidirectional,
            max_retries: 3,
            retry_timeout_millis: 1000,
        };

        let mut ping_client = PingClient::new(ping_client_config);

        let times = Some(3);

        let message = Message::new_request("Ping!".to_string());

        let (_, _) = tokio::join!(
            pong_server.serve(),
            ping_client.send_message(&message, times)
        );

        let inbox = ping_client.get_indbox();

        assert_eq!(inbox.len(), 3);
        for message in inbox {
            assert_eq!(message.get_data(), "Pong!");
        }
    }

    #[tokio::test]
    async fn test_integration_send_recieve_datagram() {
        let mut cert_path = env::temp_dir();
        cert_path.push("cert.pem");

        let mut key_path = env::temp_dir();
        key_path.push("key.pem");

        let pong_server_config = PongServerConfig {
            host: "127.0.0.1"
                .parse()
                .expect("failed to parse host for the server"),
            port: 4435,
            certificate_path: cert_path.to_str().unwrap().to_string(),
            certificate_key_path: key_path.to_str().unwrap().to_string(),
        };

        let pong_server = PongServer::new(pong_server_config);

        let ping_client_config = PingClientConfig {
            host: "127.0.0.1"
                .parse()
                .expect("failed to parse host for the server"),
            port: 4435,
            connection_type: PingClientConnectionType::Datagram,
            max_retries: 3,
            retry_timeout_millis: 1000,
        };

        let mut ping_client = PingClient::new(ping_client_config);

        let times = Some(3);

        let message = Message::new_request("Ping!".to_string());

        let (_, _) = tokio::join!(
            pong_server.serve(),
            ping_client.send_message(&message, times)
        );

        let inbox = ping_client.get_indbox();

        assert_eq!(inbox.len(), 3);
        for message in inbox {
            assert_eq!(message.get_data(), "Pong!");
        }
    }
}
