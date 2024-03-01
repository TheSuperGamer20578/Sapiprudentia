DROP TABLE documents;
DROP FUNCTION documents_update_last_modified;

CREATE TABLE notes
(
    id      SERIAL       NOT NULL PRIMARY KEY,
    owner   INTEGER      NOT NULL REFERENCES users ON DELETE CASCADE,
    date    DATE         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title   VARCHAR(255) NOT NULL DEFAULT '',
    subject INTEGER      REFERENCES subjects ON DELETE SET NULL,
    content JSONB        NOT NULL DEFAULT '{"type": "doc", "content": []}'
);
