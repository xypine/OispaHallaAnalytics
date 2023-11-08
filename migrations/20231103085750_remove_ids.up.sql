-- Add up migration script here
-- Remove id column (currently the primary key), make the hash column the primary key

ALTER TABLE OHA DROP CONSTRAINT IF EXISTS oha_pkey;
ALTER TABLE OHA ADD PRIMARY KEY (hash);
ALTER TABLE OHA DROP COLUMN id;