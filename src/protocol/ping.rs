use crate::protocol::Libp2pProtocolEvent;
use crate::transport::TransportEvent;
use futures::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{Receiver, Sender, channel};

/// Log target for the file.
const LOG_TARGET: &str = "ipfs::ping";

/// Events emitted by [`IpfsPing`].
#[derive(Debug)]
pub enum PingEvent {}

/// IPFS Ping protocol handler
pub struct IpfsPing {
    event_tx: Sender<Libp2pProtocolEvent>,
    transport_rx: Receiver<TransportEvent>,
    read_buffer: Vec<u8>,
}

impl IpfsPing {
    pub fn start(event_tx: Sender<Libp2pProtocolEvent>) -> Sender<TransportEvent> {
        let (transport_tx, transport_rx) = channel(64usize);
        let ping = Self {
            event_tx,
            transport_rx,
            read_buffer: vec![0; 32],
        };

        tokio::spawn(ping.run());
        transport_tx
    }

    async fn run(mut self) {
        tracing::debug!(target: LOG_TARGET, "start ipfs ping event loop");

        while let Some(event) = self.transport_rx.recv().await {
            match event {
                TransportEvent::SubstreamOpened(protocol, peer, direction, mut substream) => {
                    tracing::trace!(
                        target: LOG_TARGET,
                        ?peer,
                        ?direction,
                        "ipfs ping substream opened"
                    );

                    match substream.read_exact(&mut self.read_buffer[..]).await {
                        Ok(_) => {
                            tracing::trace!(
                                target: LOG_TARGET,
                                ?peer,
                                data = ?&self.read_buffer[..],
                                "ping payload read from substream"
                            );

                            if let Err(err) = substream.write(&self.read_buffer).await {
                                tracing::trace!(
                                    target: LOG_TARGET,
                                    ?peer,
                                    "failed to write data to substream"
                                );
                            }
                        }
                        Err(e) => {
                            tracing::trace!(
                                target: LOG_TARGET,
                                ?peer,
                                "failed to read data from substream"
                            );
                        }
                    }
                }
                event => {
                    tracing::info!(target: LOG_TARGET, ?event, "ignoring `TransportEvent`");
                }
            }
        }
    }
}
