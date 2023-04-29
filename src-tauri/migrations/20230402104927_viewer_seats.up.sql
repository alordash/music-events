CREATE TABLE IF NOT EXISTS viewer_seats (
    id BIGSERIAL PRIMARY KEY,
    kind VARCHAR(30) NOT NULL,
    cost_rubles DECIMAL NOT NULL,
    real_number INTEGER NOT NULL,
    concert_id BIGSERIAL REFERENCES concerts(id)
);