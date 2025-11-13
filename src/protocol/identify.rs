use crate::peer::PeerId;
use std::collections::HashSet;
use tokio::sync::mpsc::Sender;
use crate::protocol::Libp2pProtocolEvent;

const LOG_TARGET: &'static str = "ipfs::identify";

pub const PROTOCOL_NAME: &'static str = "/ipfs/id/1.0.0";

pub const PUSH_PROTOCOL_NAME: &'static str = "/ipfs/id/push/1.0.0";

const IDENTIFY_PAYLOAD_SIZE: usize = 2 << 11;

#[derive(Debug)]
pub enum IdentifyEvent {
    /// Peer identified.
    PeerIdentified {
        /// Remote peer ID.
        peer: PeerId,

        /// Supported protocols.
        supported_protocols: HashSet<String>,
    },
}


pub struct IpfsIdentify {
    /// TX channel for sending `Libp2pProtocolEvent`s.
    event_tx: Sender<Libp2pProtocolEvent>




}