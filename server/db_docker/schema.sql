CREATE TYPE user_role AS ENUM ('supervisor', 'resident');

CREATE TYPE coordinates AS
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
    initial_location      coordinates     NOT NULL,
    desired_location_name VARCHAR(255) NOT NULL,
    request_content       TEXT,

    approved_by           INT          REFERENCES "user" (id) ON DELETE SET NULL,
    approved_at           TIMESTAMP,

    comeback_at          TIMESTAMP,
    comeback_approved_by INT          REFERENCES "user" (id) ON DELETE SET NULL
);

-- catering

CREATE TABLE "dish"
(
    id SERIAL PRIMARY KEY,
    dish_name VARCHAR(255) NOT NULL
);

CREATE TABLE "catering"
(
    id SERIAL PRIMARY KEY,
    dish_id  INT REFERENCES "dish" (id) ON DELETE CASCADE,
    served_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- rating

CREATE TABLE "rating"
(
    id         SERIAL PRIMARY KEY,
    user_id    INT REFERENCES "user" (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    stars      INT NOT NULL
);

CREATE TABLE "room_rating"
(
    id         SERIAL PRIMARY KEY,
    rating_id  INT REFERENCES "rating" (id) ON DELETE CASCADE,
    room_number INT CHECK (room_number > 0)
);

CREATE TABLE "catering_rating"
(
    id         SERIAL PRIMARY KEY,
    rating_id  INT REFERENCES "rating" (id) ON DELETE CASCADE,
    catering_id    INT REFERENCES "catering" (id) ON DELETE CASCADE
);