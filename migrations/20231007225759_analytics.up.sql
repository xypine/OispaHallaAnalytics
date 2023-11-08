-- Add up migration script here
CREATE TABLE IF NOT EXISTS OHA
(
    id          UUID    PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    client      TEXT                NOT NULL,
    data_raw    TEXT                NOT NULL,
    hash        TEXT                NOT NULL,
    created_at  BIGINT              NOT NULL
);
