-- Your SQL goes here
CREATE TABLE test_logs (
    id SERIAL PRIMARY KEY,
    log TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
