CREATE TABLE documents
(
    id         SERIAL       NOT NULL PRIMARY KEY,
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title      VARCHAR(255) NOT NULL DEFAULT '',
    content    JSONB
);
