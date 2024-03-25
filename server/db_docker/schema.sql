CREATE TYPE role AS ENUM ('supervisor', 'resident');

CREATE TABLE "user"
(
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(255) NOT NULL,
    password    VARCHAR(255) NOT NULL,
    first_name  VARCHAR(255) NOT NULL,
    last_name   VARCHAR(255) NOT NULL,
    room_number int          NOT NULL,
    role        role         NOT NULL
);
