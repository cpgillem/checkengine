ALTER TABLE posting_group ADD COLUMN member_id INT NOT NULL;
ALTER TABLE posting_group ADD CONSTRAINT fk_member FOREIGN KEY (member_id) REFERENCES member (id) ON DELETE CASCADE;