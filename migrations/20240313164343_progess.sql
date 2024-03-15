DROP TABLE IF EXISTS todo;

CREATE TABLE IF NOT EXISTS todo (
    todo_id SERIAL PRIMARY KEY NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    project VARCHAR(50) NOT NULL,
    task VARCHAR(255) NOT NULL,
    task_priority INT NOT NULL,
    completed BOOLEAN DEFAULT false NOT NULL,
    completed_at TIMESTAMPTZ DEFAULT TIMESTAMP WITH TIME ZONE 'epoch',
    is_active BOOLEAN DEFAULT false NOT NULL
);

DROP TABLE IF EXISTS progress;

CREATE TABLE IF NOT EXISTS progress (
    progress_id SERIAL PRIMARY KEY NOT NULL,
    todo_id INT,
    note VARCHAR(255) DEFAULT 'Progress made' NOT NULL,
    made_at TIMESTAMPTZ NOT NULL,
    time_spent INT DEFAULT 0 NOT NULL,
    FOREIGN KEY (todo_id) REFERENCES todo(todo_id)
);

CREATE OR REPLACE FUNCTION set_default_made_at()
RETURNS TRIGGER as $$
BEGIN
    NEW.made_at := NOW() AT TIME ZONE 'America/Los_Angeles';
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_made_at_default
BEFORE INSERT ON progress
FOR EACH ROW
EXECUTE FUNCTION set_default_made_at();
