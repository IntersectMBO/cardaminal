use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    SourceConnection(String),
    #[error("{0}")]
    SourceChainsync(String),
    #[error("{0}")]
    SourceDecode(String),
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

impl From<pallas::ledger::traverse::Error> for Error {
    fn from(value: pallas::ledger::traverse::Error) -> Self {
        Error::SourceDecode(value.to_string())
    }
}
