-- doing it properly is hard so we just drop all existing tables before we start.
-- This is not a problem for us because we don't have any data in the database yet.
DO $$ DECLARE
    r RECORD;
BEGIN
    -- if the schema you operate on is not "current", you will want to
    -- replace current_schema() in query with 'schematodeletetablesfrom'
    -- *and* update the generate 'DROP...' accordingly.
    FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema() AND tablename != '__diesel_schema_migrations') LOOP
            EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
        END LOOP;
END $$;

CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    -- name used to log in, designed to be machine-readable
    username TEXT NOT NULL UNIQUE,
    -- name that can be used to display to the user, smth like "John Doe"
    -- probably we can allow the user to change it
    -- NULL if we haven't received their name yet (can happen when an unknown user is added to the session)
    name TEXT
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
    is_manual BOOLEAN NOT NULL,
    UNIQUE (user_id, session_id)
);
