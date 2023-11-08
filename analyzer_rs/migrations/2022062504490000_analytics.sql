CREATE TABLE IF NOT EXISTS games
(
    client      TEXT                        ,
    data_raw    TEXT                NOT NULL,
    hash        TEXT  PRIMARY KEY   NOT NULL,
    created_at  INTEGER             
);
