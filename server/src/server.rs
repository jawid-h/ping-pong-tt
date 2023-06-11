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
    use rand::{distributions::Alphanumeric, Rng};

    use super::*;

    fn setup_certificates() -> (String, String) {
        let cert_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let key_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let mut cert_path = env::temp_dir();
        cert_path.push(cert_name);

        let mut key_path = env::temp_dir();
        key_path.push(key_name);

        let cert_path_string = cert_path
            .into_os_string()
            .into_string()
            .expect("failed to construct certificate file path");
        let key_path_string = key_path
            .into_os_string()
            .into_string()
            .expect("failed to construct certificate key file path");

        common::utils::gen_certs::gen_certs(cert_path_string.clone(), key_path_string.clone())
            .expect("failed to generate certificate files");

        (cert_path_string, key_path_string)
    }

    fn setup_client_server(host: String, port: u16) -> (PongServer, PingClient) {
        let (cert_path, key_path) = setup_certificates();

        let pong_server_config = PongServerConfig {
            host: host.parse().expect("failed to parse host for the server"),
            port,
            certificate_path: cert_path,
            certificate_key_path: key_path,
        };

        let pong_server = PongServer::new(pong_server_config);

        let ping_client_config = PingClientConfig {
            host: host.parse().expect("failed to parse host for the server"),
            port,
            connection_type: PingClientConnectionType::Bidirectional,
            max_retries: 3,
            retry_timeout_millis: 1000,
        };

        let ping_client = PingClient::new(ping_client_config);

        (pong_server, ping_client)
    }

    #[tokio::test]
    async fn test_integration_send_recieve_bidirectional() {
        let (pong_server, mut ping_client) = setup_client_server("127.0.0.1".to_string(), 4433);

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
        let (pong_server, mut ping_client) = setup_client_server("127.0.0.1".to_string(), 4434);

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
        let (pong_server, mut ping_client) = setup_client_server("127.0.0.1".to_string(), 4435);

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
