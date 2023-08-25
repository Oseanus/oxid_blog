-- Formatted for SQLite
-- To create a test sqlite database create a directory db at the root level
-- and change directory into it. Then type in sqlite3 oxis_blog < ../scripts/oxid_blog_sqlite.sql.

-- Warning! The overall schema may progressively change as the software is evolving over time
-- which means that this script is always compatible with the state of its corresponding commit.

CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    hashed_password VARCHAR NOT NULL, 
    reset_password_selector VARCHAR,
    reset_password_sent_at TIMESTAMP,
    reset_password_validator_hash VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users(email, hashed_password) VALUES('test@test.com', 'abcd');

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY, 
    session_verifier VARCHAR NOT NULL, 
    user_id INT NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    otp_code_encrypted VARCHAR NOT NULL,
    otp_code_attempts INTEGER NOT NULL DEFAULT 0,
    otp_code_confirmed BOOLEAN NOT NULL DEFAULT false,
    otp_code_sent BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    created_at TIMSTAMPZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO posts (title) VALUES ('SQLite NOW function');

-- DROP TABLE users;
-- DROP TABLE sessions;