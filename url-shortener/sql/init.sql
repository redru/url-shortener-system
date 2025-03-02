CREATE TABLE IF NOT EXISTS urls (
    id BIGINT PRIMARY KEY,
    long_url VARCHAR(2048) NOT NULL,
    short_url VARCHAR(50) UNIQUE NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_short_url ON urls(short_url);

CREATE INDEX IF NOT EXISTS idx_long_url ON urls(short_url);