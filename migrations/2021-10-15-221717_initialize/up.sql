-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    first_name VARCHAR(32) NOT NULL,
    last_name VARCHAR(32) NOT NULL,
    hash VARCHAR(86) NOT NULL,

    erau_id INTEGER,
    signup_date TIMESTAMP NOT NULL,
    is_officer BOOLEAN NOT NULL DEFAULT FALSE,
    chess_com_username VARCHAR(32) NOT NULL,
    email VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE games (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    white_id INTEGER NOT NULL,
    black_id INTEGER NOT NULL,

    white_points REAL NOT NULL,--Either 0, 0.5 or 1.0
    black_points REAL NOT NULL,--Same. Both should sum to 1.0 for done games, or 0.0
    pgn TEXT,--Chess game in protable game format
    scorecard_image BLOB,--Base64 jpeg image of game

    game_end TIMESTAMP NOT NULL,
    game_entered TIMESTAMP NOT NULL,

    FOREIGN KEY (white_id) REFERENCES users(id),
    FOREIGN KEY (black_id) REFERENCES users(id)
);
