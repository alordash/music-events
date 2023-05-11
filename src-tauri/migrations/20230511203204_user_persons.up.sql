CREATE TABLE IF NOT EXISTS user_persons (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGSERIAL REFERENCES users(id),
    person_id BIGSERIAL REFERENCES persons(id)
);