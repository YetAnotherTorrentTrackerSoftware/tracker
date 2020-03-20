use crate::bencode::Bencode;
use crate::models::{AnnounceRequest, AnnounceResponse, Peer};
use actix_web::HttpRequest;
use std::net::{IpAddr, Ipv4Addr};

pub async fn handle_announce(req: HttpRequest) -> Bencode<AnnounceResponse> {
    let query = req.query_string();

    match serde_bencode::from_str::<AnnounceRequest>(query) {
        Ok(_announcement) => Bencode(AnnounceResponse::Success {
            interval: 1337,
            peers: vec![
                Peer {
                    id: String::from("leet_leesher"),
                    ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    port: 1337,
                },
                Peer {
                    id: String::from("leet_leesher"),
                    ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    port: 1337,
                },
                Peer {
                    id: String::from("leet_leesher"),
                    ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    port: 1337,
                },
            ],
        }),
        Err(e) => Bencode(AnnounceResponse::Failure {
            reason: format!("Error parsing bencode request: {}", e),
        }),
    }
}
