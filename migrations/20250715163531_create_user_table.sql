-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
      id serial NOT NULL PRIMARY KEY,
      email VARCHAR(255) UNIQUE NOT NULL, 
      password_hash VARBINARY(255) NOT NULL
);