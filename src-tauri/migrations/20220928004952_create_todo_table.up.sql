-- Add up migration script here
CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY,
    description TEXT,
    is_complete INTEGER
)