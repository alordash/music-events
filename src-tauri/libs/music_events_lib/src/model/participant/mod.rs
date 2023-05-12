pub mod participant;
pub mod dao {
    pub mod participant_entity;
}
pub type Participant = participant::Participant;
pub type ParticipantsRepository = participant::ParticipantsRepository;
pub mod participant_repository_extension;