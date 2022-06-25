CREATE TABLE IF NOT EXISTS OHA
(
    id          INTEGER PRIMARY KEY NOT NULL,
    data_raw    TEXT                NOT NULL,
    data_parsed TEXT                NOT NULL,
    hash        TEXT                NOT NULL
);