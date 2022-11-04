CREATE TABLE tasks
(
    id_task                 TEXT        NOT NULL
            CONSTRAINT tasks_pk
            PRIMARY KEY,
    parent_list             TEXT        NOT NULL,
    title                   TEXT        NOT NULL,
    body                    TEXT,
    importance              INTEGER     DEFAULT 1 NOT NULL,
    favorite                BOOLEAN     DEFAULT false NOT NULL,
    is_reminder_on          BOOLEAN     DEFAULT false NOT NULL,
    status                  INTEGER     DEFAULT 1 NOT NULL,
    completed_on            TIMESTAMP,
    due_date                TIMESTAMP,
    reminder_date           TIMESTAMP,
    created_date_time       TIMESTAMP    DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_modified_date_time TIMESTAMP    DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX tasks_id_uindex
    ON tasks (id_task);

CREATE TRIGGER remove_tasks_on_list_delete
    BEFORE DELETE ON lists
BEGIN
    DELETE FROM tasks WHERE tasks.parent_list = old.id_list;
END;