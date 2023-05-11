pub mod artist;
pub mod dao {
    pub mod artist_entity;
}
pub type Artist = artist::Artist;
pub type ArtistsRepository = artist::ArtistsRepository;