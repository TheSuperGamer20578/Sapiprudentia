CREATE TABLE assessments
(
    id           SERIAL       NOT NULL,
    owner        INTEGER      NOT NULL REFERENCES users,
    subject      INTEGER      NOT NULL REFERENCES subjects,
    title        VARCHAR(255) NOT NULL,
    exam         BOOLEAN      NOT NULL,
    status       SMALLINT     NOT NULL DEFAULT 0,
    weight       SMALLINT     NOT NULL,
    due          DATE         NOT NULL,
    due_period   SMALLINT     NOT NULL,
    issued       DATE,
    mark_out_of  SMALLINT,
    mark         SMALLINT,
    notification VARCHAR(255),
    submission   VARCHAR(255),
    reference    VARCHAR(255)
);