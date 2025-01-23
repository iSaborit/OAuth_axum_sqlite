CREATE TABLE tokens (
    id INTEGER PRIMARY KEY,
    user_id INTEGER,
    client_access_token TEXT NOT NULL,
    server_access_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    access_token_expiration TIMESTAMP NOT NULL,
    refresh_token_expiration TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

