CREATE TABLE IF NOT EXISTS moves
(
    move_index  INTEGER             NOT NULL,
    game_hash   TEXT                NOT NULL,
    direction   INTEGER             NOT NULL,
    PRIMARY KEY (game_hash, move_index)
);