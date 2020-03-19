use serde::{Serialize, Deserialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnounceRequest {
    pub info_hash: [u8; 20],
    pub peer_id: String,
    pub port: u16,
    pub uploaded: u32,
    pub downloaded: u32,
    pub left: u32,
    #[serde(default = "Event::default")]
    pub event: Event
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AnnounceResponse {
    Success {
        interval: u32,
        peers: Vec<Peer>
    },
    Failure {
        #[serde(rename = "failure_reason")]
        reason: String,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    pub id: String,
    pub ip: IpAddr,
    pub port: u16
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Event {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "empty")]
    Empty
}

impl Default for Event {
    fn default() -> Self {
        Event::Empty
    }
}