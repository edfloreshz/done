create table lists
(
    id_list      TEXT    not null   primary key,
    display_name TEXT    not null,
    is_owner     BOOLEAN not null,
    count        INTEGER not null,
    icon_name    TEXT default 'view-list-symbolic'
);