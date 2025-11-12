use crate::protocol::identify::IdentifyEvent;
use crate::protocol::ping::PingEvent;

pub mod identify;
pub mod ping;

pub enum Libp2pProtocolEvent {
    /// Identify events.
    Identify(IdentifyEvent),

    /// Ping events.
    Ping(PingEvent),
}
