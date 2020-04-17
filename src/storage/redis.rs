use crate::models::tracker::Peer;
use crate::storage::StorageBackend;
use actix::prelude::*;
use actix_redis::RedisActor;
use async_trait::async_trait;
use thiserror::Error;

struct RedisBackend {
    redis: Addr<RedisActor>,
}

impl RedisBackend {
    pub fn new(actor_addr: Addr<RedisActor>) -> Self {
        Self { redis: actor_addr }
    }
}

#[async_trait]
impl StorageBackend for RedisBackend {
    type Error = RedisBackendError;

    async fn list_torrents(&self) -> Result<Vec<String>, Self::Error> {
        unimplemented!()
    }

    async fn get_peers(&self, torrent_id: String) -> Result<Vec<Peer>, Self::Error> {
        unimplemented!()
    }

    async fn add_peer(&self, torrent_id: String, peer: Peer) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn remove_peer(&self, torrent_id: String, peer: Peer) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug, Error)]
enum RedisBackendError {
    #[error("redis error: {0}")]
    Redis(actix_redis::Error),
}
