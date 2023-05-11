pub mod user;
pub mod dao {
    pub mod user_entity;
}
pub type User = user::User;
pub type UsersRepository = user::UsersRepository;
pub mod user_repository_extension;