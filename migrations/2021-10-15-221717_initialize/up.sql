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
    id INTEGER PRIMARY KEY NOT NULL,
    white_id INTEGER NOT NULL,
    black_id INTEGER NOT NULL,
    FOREIGN KEY (white_id) REFERENCES users(id),
    FOREIGN KEY (black_id) REFERENCES users(id)
);
