CREATE TYPE user_role AS ENUM ('supervisor', 'resident');

CREATE TYPE coordinates AS
(
    latitude  DOUBLE PRECISION,
    longitude DOUBLE PRECISION
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
    reply_to     INT REFERENCES "message" (id) ON DELETE SET NULL,
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

    came_back_at          TIMESTAMP,
    came_back_approved_by INT          REFERENCES "user" (id) ON DELETE SET NULL
);

CREATE TABLE "room_rating"
(
    id         SERIAL PRIMARY KEY,
    room_number INT CHECK (room_number > 0) NOT NULL,
    user_id    INT REFERENCES "user" (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    points int NOT NULL
);

CREATE TABLE "dish"
(
    id SERIAL PRIMARY KEY,
    dish_name VARCHAR(255) NOT NULL
);

CREATE TABLE "meal"
(
    id SERIAL PRIMARY KEY,
    dish_id  INT REFERENCES "dish" (id) ON DELETE CASCADE,
    served_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "meal_rating_question" (
    id SERIAL PRIMARY KEY,
    question VARCHAR(255) NOT NULL
);

CREATE TABLE "meal_rating" (
    id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    meal_id INT REFERENCES "meal" (id) ON DELETE CASCADE NOT NULL,
    user_id INT REFERENCES "user" (id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    points int NOT NULL
);

CREATE TABLE "meal_rating_part" (
    id SERIAL PRIMARY KEY,
    meal_rating_id INT REFERENCES "meal_rating" (id) ON DELETE CASCADE NOT NULL,
    rating_question_id INT REFERENCES "meal_rating_question" (id) ON DELETE CASCADE NOT NULL,
    points int NOT NULL,
    description TEXT
);

CREATE TYPE meal_subrating_type AS ( 
    id INT, 
    question VARCHAR(255), 
    points INT, 
    description TEXT
); 

CREATE TYPE meal_rating_part_type AS ( 
    question_id INT, 
    points INT, 
    description TEXT
); 

CREATE FUNCTION get_meal_subratings(meal_rating_id integer) RETURNS json AS $$
    WITH subratings as (
        SELECT
            mrp.id,
            mrq.question,
            mrp.points,
            mrp.description
        FROM "meal_rating_part" mrp
        JOIN "meal_rating_question" mrq
        ON mrq.id = mrp.rating_question_id
        WHERE mrp.meal_rating_id = $1
    )
    SELECT json_agg(subratings) FROM subratings;
$$ LANGUAGE SQL;

CREATE TYPE meal_rating_type AS ( 
    id INT, 
    created_at VARCHAR(255), 
    points INT, 
    subratings json
); 

CREATE FUNCTION get_meal_ratings(meal_id integer) RETURNS json AS $$
    WITH ratings as (
        SELECT 
            id,
            points,
            created_at,
            get_meal_subratings(id) as subratings
        FROM "meal_rating"
        WHERE meal_id = $1
    )
    SELECT json_agg(ratings) FROM ratings;
$$ LANGUAGE SQL;