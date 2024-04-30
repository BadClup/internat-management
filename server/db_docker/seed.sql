INSERT INTO "user" (id, username, password, first_name, last_name, room_number, role)
VALUES (0, 'kierowniczka', 'fa585d89c851dd338a70dcf535aa2a92fee7836dd6aff1226583e88e0996293f16bc009c652826e0fc5c706695a03cddce372f139eff4d13959da6f1f5d3eabe', 'Bogumiła', 'Kapturkiewicz', NULL, 'supervisor');

INSERT INTO "user" (id, username, password, first_name, last_name, room_number, role)
VALUES (1, 'bstrama', 'fa585d89c851dd338a70dcf535aa2a92fee7836dd6aff1226583e88e0996293f16bc009c652826e0fc5c706695a03cddce372f139eff4d13959da6f1f5d3eabe', 'Bartłomiej', 'Strama', 1, 'resident');

-- password: 1234
INSERT INTO "user" (id, username, password, first_name, last_name, room_number, role)
VALUES (2, 'johnny', 'd404559f602eab6fd602ac7680dacbfaadd13630335e951f097af3900e9de176b6db28512f2e000b9d04fba5133e8b1c6e8df59db3a8ab9d60be4b97cc9e81db', 'John', 'Cena', 206, 'resident');

INSERT INTO "dish" (id, dish_name) VALUES
(1, 'kurczak z ryżem'),
(2, 'kurczak z kurczakiem');

INSERT INTO "catering" (id, dish_id, served_at) VALUES
(1, 1, '2020-01-01T00:00:00Z'),
(2, 2, '2020-01-02T00:00:00Z'),
(3, 2, '2020-01-03T00:00:00Z');

INSERT INTO "rating" (id, user_id, created_at, stars) VALUES
(1, 1, '2020-01-01T00:00:00Z', 2),
(2, 1, '2020-01-01T00:00:00Z', 4),
(3, 1, '2020-01-01T00:00:00Z', 3),
(4, 2, '2020-01-04T00:00:00Z', 4),
(5, 2, '2020-01-05T00:00:00Z', 4),
(6, 2, '2020-01-06T00:00:00Z', 4);

INSERT INTO "room_rating" (id, rating_id, room_number) VALUES
(1, 1, 206),
(2, 2, 1),
(3, 3, 210);

INSERT INTO "catering_rating" (id, rating_id, catering_id) VALUES
(1, 4, 1),
(2, 5, 2),
(3, 6, 3);