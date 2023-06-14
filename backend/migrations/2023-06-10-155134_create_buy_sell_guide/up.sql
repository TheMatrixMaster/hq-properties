-- Your SQL goes here
CREATE TABLE buyers (
    id SERIAL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    phone VARCHAR,
    buy_period VARCHAR NOT NULL,
    home_type VARCHAR NOT NULL DEFAULT 'Unsure',
    bedrooms VARCHAR NOT NULL DEFAULT '0',
    location VARCHAR NOT NULL DEFAULT 'Unsure',
    other TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    CONSTRAINT uq_buyer_email UNIQUE (email)
);

CREATE TABLE sellers (
    id SERIAL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    phone VARCHAR,
    address VARCHAR NOT NULL,
    postal_code VARCHAR,
    city VARCHAR NOT NULL,
    sell_period VARCHAR NOT NULL,
    other TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    CONSTRAINT uq_seller_email UNIQUE (email)
);
