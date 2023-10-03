-- Add migration script here
CREATE TABLE IF NOT EXISTS birthdays (
  name VARCHAR(255) PRIMARY KEY NOT NULL,
  birthday DATE NOT NULL
);