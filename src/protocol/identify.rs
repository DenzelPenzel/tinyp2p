use crate::DEFAULT_CHANNEL_SIZE;
use crate::peer::PeerId;
use crate::protocol::Libp2pProtocolEvent;
use crate::transport::{Connection, TransportEvent};
use multiaddr::Multiaddr;
use std::collections::HashSet;
use tokio::sync::mpsc::{Receiver, Sender, channel};

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

    /// Open substream to remote peer.
    OpenSubstream { peer: PeerId },
}

pub struct IpfsIdentify {
    /// TX channel for sending `Libp2pProtocolEvent`s.
    event_tx: Sender<Libp2pProtocolEvent>,

    /// RX channel for receiving `TransportEvent`s.
    transport_rx: Receiver<TransportEvent>,

    /// Read buffer for incoming messages.
    read_buffer: Vec<u8>,

    /// Listen addresses of the local node.
    listen_addresses: Vec<Multiaddr>,

    /// Protocols supported by the local node.
    protocols: Vec<String>,
}

impl IpfsIdentify {
    pub fn start(
        listen_addresses: Vec<Multiaddr>,
        protocols: Vec<String>,
        event_tx: Sender<Libp2pProtocolEvent>,
    ) -> Sender<TransportEvent> {
        let (tx, rx) = channel(64);
        let identify = Self {
            listen_addresses,
            protocols,
            event_tx,
            transport_rx: rx,
            read_buffer: vec![0u8; IDENTIFY_PAYLOAD_SIZE],
        };

        tokio::spawn(identify.run());

        tx
    }


    /// Handle inbound query by answering to it with local node's information.
    async fn handle_inbound_query(&mut self, substream: Box<dyn Connection>) {
        tracing::trace!(target: LOG_TARGET, "handle inbound stream");

        let identify = identify_schema::I

    }

    async fn run(mut self) {
        tracing::debug!(target: LOG_TARGET, "start ipfs ping event loop");

        while let Some(event) = self.transport_rx.recv().await {
            match event {
                TransportEvent::SubstreamOpened(proto, peer, direction, mut substream) => {
                    tracing::trace!(target: LOG_TARGET, ?peer, "ipfs identify substream opened");
                }
                TransportEvent::ConnectionEstablished(peer) => {
                    tracing::trace!(target: LOG_TARGET, target = ?peer, "initiate identify query");

                    let error = self
                        .event_tx
                        .send(Libp2pProtocolEvent::Identify(
                            IdentifyEvent::OpenSubstream { peer },
                        ))
                        .await;

                    if let Err(err) = error {
                        tracing::debug!(
                            target: LOG_TARGET,
                            "channel to `litep2p` closed, closing identify"
                        );
                        return;
                    }
                }
                event => {
                    tracing::info!(target: LOG_TARGET, ?event, "ignoring `TransportEvent`");
                }
            }
        }
    }
}
