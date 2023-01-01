CREATE TABLE IF NOT EXISTS posting (
    id SERIAL NOT NULL PRIMARY KEY,
    posted_at TIMESTAMP NOT NULL,
    check_number TEXT,
    summary TEXT NOT NULL,
    from_register_id INT NOT NULL,
    to_register_id INT NOT NULL,
    amount BIGINT NOT NULL,
    CONSTRAINT fk_from_register
        FOREIGN KEY (from_register_id)
        REFERENCES register(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_to_register
        FOREIGN KEY (to_register_id)
        REFERENCES register(id)
        ON DELETE CASCADE
);