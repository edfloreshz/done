create table lists
(
    id_list      TEXT    not null   primary key,
    name         TEXT    not null,
    is_owner     BOOLEAN not null,
    icon_name    TEXT default 'view-list-symbolic',
    provider     TEXT    not null
);