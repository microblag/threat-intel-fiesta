CREATE TABLE credentials
(
    id              INTEGER PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users (user_id),
    credential_kind INTEGER NOT NULL,
    value           TEXT    NOT NULL
)