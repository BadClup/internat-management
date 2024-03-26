CREATE TYPE user_role AS ENUM ('supervisor', 'resident');

CREATE TABLE "user"
(
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(255) NOT NULL UNIQUE CHECK (LENGTH(username) > 0),
    password    VARCHAR(255) NOT NULL, -- checking password length here is pointless because it is hashed
    first_name  VARCHAR(255) NOT NULL,
    last_name   VARCHAR(255) NOT NULL,
    room_number int          NOT NULL CHECK ( room_number > 0 ),
    role        user_role    NOT NULL
);

CREATE TABLE "message"
(
    id           SERIAL PRIMARY KEY,
    sender_id    INT REFERENCES "user" (id) ON DELETE CASCADE,
    recipient_id INT REFERENCES "user" (id) ON DELETE CASCADE,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "text_message"
(
    id         SERIAL PRIMARY KEY,
    message_id INT REFERENCES "message" (id) ON DELETE CASCADE,
    content    TEXT NOT NULL
);

CREATE TABLE "exit_request_message"
(
    id                    SERIAL PRIMARY KEY,
    message_id            INT REFERENCES "message" (id) ON DELETE CASCADE,
    current_user_location VARCHAR(255) NOT NULL,
    desired_location      VARCHAR(255) NOT NULL,
    request_content       TEXT,
    approved              BOOLEAN      NOT NULL                                  DEFAULT FALSE,
    approved_by           INT          REFERENCES "user" (id) ON DELETE SET NULL DEFAULT NULL,
    approved_at           TIMESTAMP                                              DEFAULT NULL
);