INSERT INTO "user" (username, password, first_name, last_name, room_number, role)
VALUES ('bstrama', 'fa585d89c851dd338a70dcf535aa2a92fee7836dd6aff1226583e88e0996293f16bc009c652826e0fc5c706695a03cddce372f139eff4d13959da6f1f5d3eabe', 'Bartłomiej', 'Strama', 1, 'resident'),
('kierowniczka', 'fa585d89c851dd338a70dcf535aa2a92fee7836dd6aff1226583e88e0996293f16bc009c652826e0fc5c706695a03cddce372f139eff4d13959da6f1f5d3eabe', 'Bogumiła', 'Kapturkiewicz', NULL, 'supervisor'),

-- password: 1234
('johnny', 'd404559f602eab6fd602ac7680dacbfaadd13630335e951f097af3900e9de176b6db28512f2e000b9d04fba5133e8b1c6e8df59db3a8ab9d60be4b97cc9e81db', 'John', 'Cena', 206, 'resident');

INSERT INTO "room_rating" (room_number, user_id, created_at, points) VALUES
(206, 2, '2020-01-01T00:00:00Z', 2),
(1, 2, '2020-01-01T00:00:00Z', 4),
(210, 2, '2020-01-01T00:00:00Z', 3);

INSERT INTO "dish" (dish_name) VALUES
('kurczak z ryżem'),
('kurczak z kurczakiem');

INSERT INTO "meal" (dish_id, served_at) VALUES
(1, '2020-01-01T00:00:00Z'),
(2, '2020-01-02T00:00:00Z'),
(2, '2020-01-03T00:00:00Z');

INSERT INTO "meal_rating_question" (question) VALUES
('Smakowało?'),
('Ciepłe?'),
('Długo trzeba było czekać?');

INSERT INTO "meal_rating" (meal_id, user_id, created_at, points) VALUES
(1, 1, '2020-01-03T03:00:00Z', 4),
(2, 1, '2020-01-03T03:00:00Z', 0);

INSERT INTO "meal_rating_part" (meal_rating_id, rating_question_id, points, description) VALUES
(1, 1, 6, NULL),
(1, 2, 2, NULL),
(1, 3, 9, 'Kolacja było, ale nikogo poza mną więc ez'),
(2, 1, 0, 'TRAGEDIA'),
(2, 3, 0, NULL);