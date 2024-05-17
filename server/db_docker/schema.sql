CREATE TYPE user_role AS ENUM ('supervisor', 'resident');

CREATE TYPE lat_long AS
(
    latitude  NUMERIC,
    longitude NUMERIC
);

CREATE TABLE "user"
(
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(255) NOT NULL UNIQUE CHECK (LENGTH(username) > 0),
    password    VARCHAR(255) NOT NULL, -- checking password length here is pointless because it is hashed
    first_name  VARCHAR(255) NOT NULL,
    last_name   VARCHAR(255) NOT NULL,
    room_number int CHECK ((role = 'resident' AND room_number > 0) OR role != 'resident'),
    role        user_role    NOT NULL
);

CREATE TABLE "message"
(
    id           SERIAL PRIMARY KEY,
    sender_id    INT NOT NULL REFERENCES "user" (id) ON DELETE CASCADE,
    recipient_id INT NOT NULL REFERENCES "user" (id) ON DELETE CASCADE,
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
    initial_location      lat_long     NOT NULL,
    desired_location_name VARCHAR(255) NOT NULL,
    request_content       TEXT,

    approved_by           INT          REFERENCES "user" (id) ON DELETE SET NULL,
    approved_at           TIMESTAMP,

    came_back_at          TIMESTAMP,
    came_back_approved_by INT          REFERENCES "user" (id) ON DELETE SET NULL
);
