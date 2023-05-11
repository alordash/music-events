pub mod person;
pub mod dao {
    pub mod person_entity;
}
pub type Person = person::Person;
pub type PersonsRepository = person::PersonsRepository;