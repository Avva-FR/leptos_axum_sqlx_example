CREATE TABLE user_table (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    pwd VARCHAR NOT NULL
);

CREATE UNIQUE INDEX username_idx ON user_table (username);