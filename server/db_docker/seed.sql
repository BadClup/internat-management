INSERT INTO "user" (username, password, first_name, last_name, room_number, role)
VALUES ('bstrama', 'fa585d89c851dd338a70dcf535aa2a92fee7836dd6aff1226583e88e0996293f16bc009c652826e0fc5c706695a03cddce372f139eff4d13959da6f1f5d3eabe', 'Bartłomiej', 'Strama', 1, 'resident'),
('kierowniczka', 'fa585d89c851dd338a70dcf535aa2a92fee7836dd6aff1226583e88e0996293f16bc009c652826e0fc5c706695a03cddce372f139eff4d13959da6f1f5d3eabe', 'Bogumiła', 'Kapturkiewicz', NULL, 'supervisor'),
('johnny', 'd404559f602eab6fd602ac7680dacbfaadd13630335e951f097af3900e9de176b6db28512f2e000b9d04fba5133e8b1c6e8df59db3a8ab9d60be4b97cc9e81db', 'John', 'Cena', 206, 'resident');
-- passwords:
-- bstrama, kierowniczka: 12345678
-- johnny: 1234

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

INSERT INTO "message" (sender_id, recipient_id, reply_to, created_at)
VALUES
(1, 2, NULL, '2023-10-19 09:00:00'),
(2, 1, 1, '2023-10-19 09:05:00'),
(1, 2, 2, '2023-10-19 09:10:00'),
(2, 1, NULL, '2023-10-19 09:20:00'),
(1, 2, 4, '2023-10-19 09:30:00');

INSERT INTO "text_message" (message_id, content)
VALUES
(1, 'Hey, how are you?'), 
(2, 'I am good, thanks! How about you?'), 
(3, 'I am doing well, thanks for asking!'), 
(4, 'Do you have time to chat later today?'), 
(5, 'Yes, I will be free in the afternoon.');

INSERT INTO "exit_request_message" (message_id, initial_location, desired_location_name, request_content, approved_by, approved_at, came_back_at, came_back_approved_by)
VALUES
(1, '(34.0522,-118.2437)', 'Los Angeles', 'Need to exit for an urgent meeting.', 2, '2023-10-19 10:00:00', '2023-10-19 12:00:00', 2),
(3, '(40.7128,-74.0060)', 'New York', 'Requesting exit for personal reasons.', 2, '2023-10-20 09:30:00', '2023-10-20 11:30:00', 2),
(5, '(51.5074,-0.1278)', 'London', 'Leaving to attend a conference.', 2, '2023-10-21 08:15:00', NULL, NULL);
