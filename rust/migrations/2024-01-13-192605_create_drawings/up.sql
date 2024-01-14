-- Your SQL goes here
CREATE TABLE DRAWINGS (
    id          SERIAL      NOT NULL PRIMARY KEY UNIQUE,
    lines       TEXT        NOT NULL,
    artist      TEXT        NOT NULL,
    created_at  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
)