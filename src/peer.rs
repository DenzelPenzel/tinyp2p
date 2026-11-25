use multihash::Multihash;
use std::fmt;
use tokio::sync::mpsc::Sender;

const LOG_TARGET: &'static str = "ipfs::identify";

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PeerId {
    multihash: Multihash,
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_base58().fmt(f)
    }
}

impl PeerId {
    pub fn start(&mut self) {}

    async fn run(self) {}

    pub fn to_bytes(&self) -> Vec<u8> {
        self.multihash.to_bytes()
    }

    pub fn to_base58(&self) -> String {
        bs58::encode(self.to_bytes()).into_string()
    }
}
