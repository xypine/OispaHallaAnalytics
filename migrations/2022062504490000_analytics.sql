CREATE TABLE IF NOT EXISTS OHA
(
    id          INTEGER PRIMARY KEY NOT NULL,
    client      TEXT                        ,
    data_raw    TEXT                NOT NULL,
    hash        TEXT                NOT NULL
);
