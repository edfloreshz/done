create table tasks
(
    id_task                 text    not null
        constraint tasks_pk
            primary key,
    id_list text not null
        constraint tasks_lists_fk
            references lists(id_list),
    title                   text    not null,
    body                    text,
    completed_on            text,
    due_date                text,
    importance              text    default 'normal',
    favorite                BOOLEAN default false not null,
    is_reminder_on          BOOLEAN default false not null,
    reminder_date           text,
    status                  text    default 'notStarted',
    created_date_time       text    default CURRENT_TIMESTAMP,
    last_modified_date_time text    default CURRENT_TIMESTAMP
);

create unique index tasks_id_task_uindex
    on tasks (id_task);

CREATE TRIGGER save_task_count_new
    AFTER INSERT ON tasks
BEGIN
    UPDATE lists
    SET count = (SELECT COUNT(*) FROM tasks WHERE id_list = new.id_list)
    WHERE id_list = new.id_list;
end;

CREATE TRIGGER update_task_count
    BEFORE DELETE ON tasks
BEGIN
    UPDATE lists
    SET count = (SELECT COUNT(*) FROM tasks WHERE id_list = old.id_list)
    WHERE id_list = old.id_list;
end;

CREATE TRIGGER remove_tasks_on_list_delete
    BEFORE DELETE ON lists
BEGIN
    DELETE FROM tasks WHERE tasks.id_list = old.id_list;
end;
