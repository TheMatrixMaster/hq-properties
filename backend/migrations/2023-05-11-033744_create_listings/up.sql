-- Your SQL goes here

CREATE type MARKET_STATUS as ENUM ('sold', 'sale', 'rent');

CREATE TABLE listings (
  id SERIAL,
  city VARCHAR NOT NULL,
  address VARCHAR NOT NULL,
  bedrooms SMALLINT NOT NULL DEFAULT 0,
  bathrooms SMALLINT NOT NULL DEFAULT 0,
  area INT NOT NULL DEFAULT 0,
  price INT NOT NULL DEFAULT 0,
  market_st MARKET_STATUS NOT NULL DEFAULT 'sold',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);

CREATE TABLE listing_images (
  id SERIAL,
  listing_id INT NOT NULL,
  url TEXT NOT NULL,
  priority SMALLINT NOT NULL,
  tag VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  CONSTRAINT fk_listing FOREIGN KEY (listing_id) REFERENCES listings(id),
  CONSTRAINT uq_priority UNIQUE (listing_id, priority)
);
