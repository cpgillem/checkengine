-- Create the transaction table.
CREATE TABLE IF NOT EXISTS posting_group (
    id SERIAL NOT NULL PRIMARY KEY,
    posted_at TIMESTAMP NOT NULL,
    check_number TEXT,
    summary TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL
);

-- Add references to posting table.
ALTER TABLE posting ADD posting_group_id INT NOT NULL;
ALTER TABLE posting 
    ADD CONSTRAINT fk_posting_group
    FOREIGN KEY (posting_group_id)
    REFERENCES posting_group(id);