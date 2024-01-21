CREATE TABLE subjects
(
    id                  SERIAL       NOT NULL PRIMARY KEY,
    owner               INTEGER      NOT NULL REFERENCES users,
    name                VARCHAR(255) NOT NULL,
    class               VARCHAR(16)  NOT NULL,
    active              BOOLEAN      NOT NULL DEFAULT TRUE,
    google_classroom_id VARCHAR(16)
);
