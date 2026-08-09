CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT,
    age INTEGER
);

INSERT INTO users VALUES
(1, 'Alice', 20),
(2, 'Bob', 25),
(3, 'Charlie', 30);

SELECT * FROM users;

SELECT *
FROM users
WHERE age >= 25;

CREATE INDEX idx_users_age
ON users(age);

EXPLAIN SELECT *
FROM users
WHERE age = 30;

BEGIN;

UPDATE users
SET age = 31
WHERE id = 3;

ROLLBACK;

SELECT * FROM users;
