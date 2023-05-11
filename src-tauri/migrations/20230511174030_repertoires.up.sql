CREATE TABLE IF NOT EXISTS repertoires (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    order_number INTEGER NOT NULL,
    participant_id BIGSERIAL REFERENCES participants(id)
);