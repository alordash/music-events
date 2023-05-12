pub mod concert;
pub mod dao {
    pub mod concert_entity;
}
pub type Concert = concert::Concert;
pub type ConcertsRepository = concert::ConcertsRepository;
pub mod concert_repository_extension;