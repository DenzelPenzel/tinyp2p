use crate::error::Error;
use crate::peer::PeerId;
use futures::{AsyncRead, AsyncWrite};
use multiaddr::Multiaddr;
use std::fmt::Debug;

pub trait Connection: AsyncRead + AsyncWrite + Unpin + Send + Debug + 'static {}

impl<T: AsyncRead + AsyncWrite + Unpin + Send + Debug + 'static> Connection for T {}

#[derive(Debug)]
pub enum Direction {
    Inbound,
    Outbound,
}

#[derive(Debug)]
pub enum TransportEvent {
    SubstreamOpened(String, PeerId, Direction, Box<dyn Connection>),
    SubstreamClosed(String, Direction),
    ConnectionEstablished(PeerId),
    ConnectionClosed(PeerId),
    DialFailure(Multiaddr),
    SubstreamOpenFailed(String, PeerId, Error),
}
