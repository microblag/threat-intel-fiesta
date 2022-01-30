CREATE TABLE user
(
    id        INTEGER            NOT NULL PRIMARY KEY,
    user_name VARCHAR(64) UNIQUE NOT NULL
);

CREATE TABLE credential
(
    id              INTEGER NOT NULL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES user (id),
    credential_kind INTEGER NOT NULL,
    value           TEXT    NOT NULL
)