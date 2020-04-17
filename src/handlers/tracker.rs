use crate::bencode::Bencode;
use crate::models::tracker::{AnnounceRequest, AnnounceResponse, Peer};
use crate::AppState;
use actix_web::{web, HttpRequest};
use std::net::{IpAddr, Ipv4Addr};

pub async fn handle_announce(
    req: HttpRequest,
    _app_state: web::Data<AppState>,
) -> Bencode<AnnounceResponse> {
    let query = req.query_string();

    match serde_urlencoded::from_str::<AnnounceRequest>(query) {
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
            reason: format!("Error parsing query string: {}", e),
        }),
    }
}
