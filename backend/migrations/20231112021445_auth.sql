CREATE TABLE users
(
    id                      SERIAL              NOT NULL PRIMARY KEY,
    username                VARCHAR(16) UNIQUE  NOT NULL,
    password                VARCHAR(128)         NOT NULL,
    name                    VARCHAR(255)        NOT NULL,
    email                   VARCHAR(255) UNIQUE NOT NULL,
    account_type            INTEGER             NOT NULL DEFAULT 0,
    created_at              TIMESTAMP           NOT NULL DEFAULT NOW(),
    require_password_change BOOLEAN             NOT NULL DEFAULT TRUE
);

-- Username: admin
-- Password: admin
INSERT INTO users (username, password, name, email, account_type)
VALUES ('admin', '$argon2id$v=19$m=16,t=2,p=1$WnNUZjg2M1UyZ3N0NkRmUg$JkJE0ZSy5h1/T/54wh7Gew', 'Admin', 'admin@example.com', 1);

CREATE TABLE sessions
(
    id              SERIAL    NOT NULL PRIMARY KEY,
    user_id         INTEGER REFERENCES users (id) ON DELETE CASCADE,
    last_ip         VARCHAR(45),
    last_user_agent VARCHAR(255),
    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    last_seen       TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE documents
    ADD COLUMN owner INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE DEFAULT 1;
ALTER TABLE documents
    ALTER COLUMN owner DROP DEFAULT;
