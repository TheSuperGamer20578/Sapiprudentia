ALTER TABLE documents
    ADD COLUMN last_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL;

CREATE FUNCTION documents_update_last_modified() RETURNS TRIGGER
    LANGUAGE plpgsql
    AS $$
        BEGIN
            NEW.last_modified = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
    $$;

CREATE TRIGGER documents_update_last_modified
    BEFORE UPDATE
    ON documents
    FOR EACH ROW
    EXECUTE FUNCTION documents_update_last_modified();
