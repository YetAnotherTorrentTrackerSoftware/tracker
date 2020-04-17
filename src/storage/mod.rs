use crate::models::tracker::Peer;
use async_trait::async_trait;

mod redis;

/// Represents a storage backend to store peer information for the tracker.
#[async_trait]
pub trait StorageBackend {
    type Error;

    /// Get the id of all torrents that are currently managed by this tracker.
    async fn list_torrents(&self) -> Result<Vec<String>, Self::Error>;

    /// Get a list of all peers that are currently seeding the given torrent.
    async fn get_peers(&self, torrent_id: String) -> Result<Vec<Peer>, Self::Error>;

    /// Add a peer for the given torrent.
    async fn add_peer(&self, torrent_id: String, peer: Peer) -> Result<(), Self::Error>;

    /// Remove a peer from the given torrent.
    async fn remove_peer(&self, torrent_id: String, peer: Peer) -> Result<(), Self::Error>;
}
