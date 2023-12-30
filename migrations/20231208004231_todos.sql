CREATE TABLE todos
(
    id        SERIAL       NOT NULL PRIMARY KEY,
    owner     INTEGER      NOT NULL REFERENCES users ON DELETE CASCADE,
    title     VARCHAR(255) NOT NULL,
    completed BOOLEAN      NOT NULL DEFAULT FALSE,
    subject   INTEGER      REFERENCES subjects ON DELETE SET NULL,
    parent    INTEGER      REFERENCES todos ON DELETE CASCADE,
    due       DATE,
    archived  BOOLEAN      NOT NULL DEFAULT FALSE
);
