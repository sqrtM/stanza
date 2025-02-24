CREATE DATABASE stanza;

CREATE EXTENSION pgcrypto;

CREATE SCHEMA activity_pub;

CREATE TABLE activity_pub.actors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    username TEXT NOT NULL,
    actor_type TEXT NOT NULL, -- e.g., Person, Service, etc.
    summary TEXT, -- short bio/description
    public_key TEXT NOT NULL, -- Public key for verifying signatures
    created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
);

INSERT INTO
    activity_pub.actors (
        id,
        username,
        actor_type,
        summary,
        public_key,
        created_at,
        updated_at
    )
VALUES
    (
        DEFAULT,
        'stanza',
        'admin',
        'admin account',
        '',
        DEFAULT,
        DEFAULT
    );

-- for testing
CREATE TABLE activity_pub.box (data JSONB);

CREATE TABLE activity_pub.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    username TEXT UNIQUE,
    password TEXT,
    email TEXT,
    profile_picture TEXT
);

INSERT INTO
    activity_pub.users
VALUES
    (DEFAULT, 'a', 'a', 'a', NULL),
    (DEFAULT, 'b', 'b', 'b', NULL),
    (DEFAULT, 'c', 'c', 'c', NULL),
    (DEFAULT, 'd', 'd', 'd', NULL),
    (DEFAULT, 'e', 'e', 'e', NULL),
    (DEFAULT, 'f', 'f', 'f', NULL),
    (DEFAULT, 'g', 'g', 'g', NULL);

SELECT
    EXISTS (
        SELECT
            1
        FROM
            activity_pub.users
        WHERE
            username = 'a'
    );
