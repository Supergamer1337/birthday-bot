-- Add migration script here
CREATE TABLE birthdays (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  birthday DATE NOT NULL
);