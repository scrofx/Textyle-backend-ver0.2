-- Your SQL goes here
CREATE TABLE user_histories (
    username VARCHAR PRIMARY KEY REFERENCES users(username),
    requests JSONB[] NOT NULL
);