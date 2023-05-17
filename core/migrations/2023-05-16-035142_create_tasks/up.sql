CREATE TABLE tasks (
    id_task TEXT NOT NULL CONSTRAINT tasks_pk PRIMARY KEY,
    parent TEXT NOT NULL,
    title TEXT NOT NULL,
    notes TEXT,
    priority INTEGER DEFAULT 1 NOT NULL,
    favorite BOOLEAN DEFAULT false NOT NULL,
    status INTEGER DEFAULT 1 NOT NULL,
    completion_date TIMESTAMP,
    due_date TIMESTAMP,
    reminder_date TIMESTAMP,
    created_date_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_modified_date_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    sub_tasks TEXT DEFAULT "[]" NOT NULL,
    tags TEXT DEFAULT "[]" NOT NULL,
    today BOOLEAN DEFAULT false NOT NULL,
    deletion_date TIMESTAMP,
    recurrence TEXT
);
CREATE UNIQUE INDEX tasks_id_uindex ON tasks (id_task);
CREATE TRIGGER remove_tasks_on_list_delete BEFORE DELETE ON lists BEGIN
DELETE FROM tasks
WHERE tasks.parent = old.id_list;
END;