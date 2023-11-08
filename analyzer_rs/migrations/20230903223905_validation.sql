-- Add migration script here
CREATE TABLE IF NOT EXISTS validations
(
    game_hash   TEXT  PRIMARY KEY   NOT NULL,
    score       INTEGER             NOT NULL,
    score_end   INTEGER             NOT NULL,
    score_margin INTEGER            NOT NULL,
    breaks      INTEGER             NOT NULL
);
