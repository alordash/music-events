pub mod viewer_seat;
pub mod dao {
    pub mod viewer_seat_entity;
}
pub type ViewerSeat = viewer_seat::ViewerSeat;
pub type ViewerSeatsRepository = viewer_seat::ViewerSeatsRepository;
pub mod viewer_seat_repository_extension;