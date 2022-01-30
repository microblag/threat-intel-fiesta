CREATE TABLE users
(
    id        INTEGER            NOT NULL PRIMARY KEY,
    user_name VARCHAR(64) UNIQUE NOT NULL
)