CREATE TABLE IF NOT EXISTS participants (
    id BIGSERIAL PRIMARY KEY,
    concert_id BIGSERIAL REFERENCES concerts(id),
    group_id BIGSERIAL REFERENCES groups(id)
);