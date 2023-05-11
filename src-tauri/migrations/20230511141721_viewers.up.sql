CREATE TABLE IF NOT EXISTS viewers (
    id BIGSERIAL PRIMARY KEY,
    person_id BIGSERIAL REFERENCES persons(id),
    viewer_seat_id BIGSERIAL REFERENCES viewer_seats(id)
);