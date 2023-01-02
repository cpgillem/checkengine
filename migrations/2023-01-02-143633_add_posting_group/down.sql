ALTER TABLE posting DROP CONSTRAINT fk_posting_group RESTRICT;
ALTER TABLE posting DROP COLUMN posting_group_id;

DROP TABLE IF EXISTS posting_group;