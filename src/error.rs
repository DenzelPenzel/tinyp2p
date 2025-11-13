use crate::peer::PeerId;
use multihash::{Multihash, MultihashGeneric};
use std::{
    error, fmt,
    io::{self, ErrorKind},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Peer `{0}` does not exist")]
    PeerDoesntExist(PeerId),
    #[error("Protocol `{0}` not supported")]
    ProtocolNotSupported(String),
    #[error("Address error: `{0}`")]
    AddressError(AddressError),
    #[error("Parse error: `{0}`")]
    ParseError(ParseError),
    #[error("I/O error: `{0}`")]
    IoError(ErrorKind),
    #[error("Negotiation error: `{0}`")]
    NegotiationError(NegotiationError),
    #[error("Substream error")]
    SubstreamError(SubstreamError),
    #[error("Essential task closed")]
    EssentialTaskClosed,
    #[error("Unknown error occurred")]
    Unknown,
}

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("Invalid protocol")]
    InvalidProtocol,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid multihash: `{0:?}`")]
    InvalidMultihash(Multihash),
    #[error("Failed to decode protobuf message: `{0:?}`")]
    ProstDecodeError(prost::DecodeError),
    #[error("Decoding error: `{0:?}`")]
    DecodingError(DecodingError),
}

#[derive(Debug, thiserror::Error)]
pub enum SubstreamError {
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("yamux error: `{0}`")]
    YamuxError(yamux::ConnectionError),
}

#[derive(Debug, thiserror::Error)]
pub enum NegotiationError {
    #[error("multistream-select error: `{0:?}`")]
    MultistreamSelectError(multistream_select::NegotiationError),
    #[error("multistream-select error: `{0:?}`")]
    SnowError(snow::Error),
    #[error("Connection closed while negotiating")]
    ConnectionClosed,
    #[error("`PeerId` missing from Noise handshake")]
    PeerIdMissing,
}

impl From<MultihashGeneric<64>> for Error {
    fn from(hash: MultihashGeneric<64>) -> Self {
        Error::ParseError(ParseError::InvalidMultihash(hash))
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IoError(value.kind())
    }
}

#[derive(Debug)]
pub struct DecodingError {
    msg: String,
    source: Option<Box<dyn error::Error + Send + Sync>>,
}
