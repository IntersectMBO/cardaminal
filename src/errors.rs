use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error to connect the source {0}")]
    SourceConnection(String),
    #[error("Error to chain sync {0}")]
    SourceChainsync(String),
}

impl From<pallas::network::facades::Error> for Error {
    fn from(value: pallas::network::facades::Error) -> Self {
        Error::SourceConnection(value.to_string())
    }
}

impl From<pallas::network::miniprotocols::chainsync::ClientError> for Error {
    fn from(value: pallas::network::miniprotocols::chainsync::ClientError) -> Self {
        Error::SourceChainsync(value.to_string())
    }
}
