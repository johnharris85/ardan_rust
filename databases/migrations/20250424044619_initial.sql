-- Add migration script here
CREATE TABLE IF NOT EXISTS messages
(
    id  INTEGER PRIMARY KEY NOT NULL,
    message TEXT NOT NULL
);

INSERT INTO messages (id, message) VALUES (1, "Hello World!");
INSERT INTO messages (id, message) VALUES (2, "Hello Galaxy!");
INSERT INTO messages (id, message) VALUES (3, "Hello Universe!");