DROP TABLE IF EXISTS todos;

CREATE TABLE IF NOT EXISTS todos (
    todo_id SERIAL PRIMARY KEY NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    project VARCHAR(50) NOT NULL,
    task VARCHAR(255) NOT NULL,
    task_priority INT NOT NULL,
    completed BOOLEAN DEFAULT false NOT NULL,
    completed_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT false NOT NULL
);

CREATE OR REPLACE FUNCTION set_default_created_at()
RETURNS TRIGGER as $$
BEGIN
    NEW.created_at := NOW() AT TIME ZONE 'America/Los_Angeles';
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_created_at_default
BEFORE INSERT ON todos
FOR EACH ROW
EXECUTE FUNCTION set_default_created_at();