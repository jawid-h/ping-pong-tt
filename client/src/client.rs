use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    time::Duration,
};

use common::{error::ConnectionError, message::Message};
use wtransport::{ClientConfig, Endpoint};

use crate::{
    error::{ClientError, ClientSetupError},
    handler::{send_bidirectional, send_datagram, send_unidirectional},
};

/// Represents the type of connection the `PingClient` will establish.
///
/// * `Bidirectional` - Data can be sent and received.
/// * `Unidirectional` - Data can only be sent or only be received, but not both.
/// * `Datagram` - Data will be sent using the Datagram protocol (typically UDP).
pub enum PingClientConnectionType {
    Bidirectional,
    Unidirectional,
    Datagram,
}

/// Represents the configuration for a `PingClient`.
///
/// # Fields
/// * `host` - IP address of the server to connect to.
/// * `port` - Port of the server to connect to.
/// * `connection_type` - Specifies the type of connection to establish.
/// * `max_retries` - Maximum number of connection attempts.
/// * `retry_timeout_millis` - Amount of time (in milliseconds) to wait between connection attempts.
pub struct PingClientConfig {
    pub host: IpAddr,
    pub port: u16,
    pub connection_type: PingClientConnectionType,
    pub max_retries: u16,
    pub retry_timeout_millis: u64,
}

/// Represents a `PingClient` used to send Ping! messages to the server.
///
/// The `PingClient` uses the settings from a `PingClientConfig` to control its behavior.
pub struct PingClient {
    config: PingClientConfig,
}

impl PingClient {
    /// Creates a new `PingClient` instance.
    ///
    /// # Arguments
    /// * `config` - A `PingClientConfig` object that contains the configuration settings for the `PingClient`.
    ///
    /// # Returns
    /// Returns a `PingClient` instance.
    pub fn new(config: PingClientConfig) -> Self {
        Self { config }
    }

    /// Asynchronously sends a message to a server using the client's connection settings.
    ///
    /// # Arguments
    /// * `message` - The `Message` instance to be sent.
    /// * `times` - The number of times to attempt sending the message.
    ///
    /// # Returns
    /// * `Result` - An empty `Ok` result if the message is sent successfully, or a `ClientError` if an error occurs.
    pub async fn send_message(
        &self,
        message: &Message,
        times: Option<u32>,
    ) -> Result<(), ClientError> {
        // Building the client configuration with the bind address and no certificate validation
        // The configuration is happening here due to limitations of `wttransport` crate
        let config = ClientConfig::builder()
            .with_bind_address(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0))
            .with_no_cert_validation();

        let endpoint = Endpoint::client(config)
            .map_err(|_| ClientError::SetupError(ClientSetupError::EndpointCreationError))?;

        // Handle retry logic in case of endpoint connection failure
        let mut retries = 0;
        let connection = loop {
            let maybe_connecting = endpoint.connect(
                SocketAddr::new(self.config.host, self.config.port),
                "localhost",
            );

            if maybe_connecting.is_err() {
                println!("connection failed, retrying...");

                tokio::time::sleep(Duration::from_millis(self.config.retry_timeout_millis)).await;

                if retries > self.config.max_retries {
                    return Err(ClientError::ConnectionError(
                        ConnectionError::MaxRetriesReached {
                            retry_count: retries,
                        },
                    ));
                }

                retries += 1;

                continue;
            }

            let connecting = maybe_connecting.unwrap();

            // The retry logic duplicates here because the `connecting.await` call can fail as well
            // and we would like to handle this case as well
            let maybe_connection = connecting.await;

            if maybe_connection.is_err() {
                println!("connection failed, retrying...");

                tokio::time::sleep(Duration::from_millis(self.config.retry_timeout_millis)).await;

                if retries > self.config.max_retries {
                    return Err(ClientError::ConnectionError(
                        ConnectionError::MaxRetriesReached {
                            retry_count: retries,
                        },
                    ));
                }

                retries += 1;

                continue;
            }

            break maybe_connection.unwrap();
        };

        match self.config.connection_type {
            PingClientConnectionType::Bidirectional => {
                send_bidirectional(&connection, message, times).await?;
            }
            PingClientConnectionType::Unidirectional => {
                send_unidirectional(&connection, message, times).await?;
            }
            PingClientConnectionType::Datagram => {
                send_datagram(&connection, message, times).await?;
            }
        }

        Ok(())
    }
}
