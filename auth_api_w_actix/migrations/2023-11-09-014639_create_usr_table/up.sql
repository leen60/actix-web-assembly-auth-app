CREATE TABLE usr (
    id          VARCHAR(255) PRIMARY KEY,
    name       VARCHAR(255) NOT NULL,
    email       VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP
);