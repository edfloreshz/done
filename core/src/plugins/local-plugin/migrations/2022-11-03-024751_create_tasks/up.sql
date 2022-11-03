create table tasks
(
    id_task                 TEXT    not null
        constraint tasks_pk
            primary key,
    parent_list text not null
        constraint tasks_lists_fk
            references lists(parent_list),
    title                   TEXT    not null,
    body                    TEXT,
    importance              INTEGER    default 1 not null,
    favorite                BOOLEAN default false not null,
    is_reminder_on          BOOLEAN default false not null,
    status                  INTEGER    default 1 not null,
    completed_on            TIMESTAMP,
    due_date                TIMESTAMP,
    reminder_date           TIMESTAMP,
    created_date_time       TIMESTAMP    default CURRENT_TIMESTAMP not null,
    last_modified_date_time TIMESTAMP    default CURRENT_TIMESTAMP not null
);

create unique index tasks_id_uindex
    on tasks (id_task);

CREATE TRIGGER remove_tasks_on_list_delete
    BEFORE DELETE ON lists
BEGIN
    DELETE FROM tasks WHERE tasks.parent_list = old.parent_list;
end;
