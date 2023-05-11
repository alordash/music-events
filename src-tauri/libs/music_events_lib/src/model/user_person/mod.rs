pub mod user_person;
pub mod dao {
    pub mod user_person_entity;
}
pub type UserPerson = user_person::UserPerson;
pub type UserPersonsRepository = user_person::UserPersonsRepository;