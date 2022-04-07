create table tasks
(
    id_task                 text    not null
        constraint tasks_pk
            primary key,
    id_list text not null
        constraint tasks_lists_fk
            references lists(id_list),
    title                   text    not null,
    body                    text    not null,
    completed_on            text,
    due_date                text,
    importance              text    not null,
    is_reminder_on          integer not null,
    reminder_date           text,
    status                  text    not null,
    created_date_time       text    not null,
    last_modified_date_time text    not null
);

create unique index tasks_id_task_uindex
    on tasks (id_task);

