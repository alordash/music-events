CREATE TABLE IF NOT EXISTS group_artists (
    id BIGSERIAL PRIMARY KEY,
    group_id BIGSERIAL REFERENCES groups(id),
    artist_id BIGSERIAL REFERENCES artists(id),
    role VARCHAR(50) NOT NULL
);