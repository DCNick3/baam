CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    claims JSONB NOT NULL
);

CREATE TABLE sessions
(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    owner_id INTEGER NOT NULL REFERENCES users(id),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP -- NULL if the session is still ongoing
);

CREATE TABLE marks
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    session_id INTEGER NOT NULL REFERENCES sessions(id),
    mark_time TIMESTAMP NOT NULL,
    is_manual BOOLEAN NOT NULL
);
