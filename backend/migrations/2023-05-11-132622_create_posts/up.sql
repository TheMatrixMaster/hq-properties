-- Your SQL goes here

CREATE TABLE posts (
  id SERIAL,
  img TEXT NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  PRIMARY KEY (id)
)

