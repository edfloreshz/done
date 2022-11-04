CREATE TABLE lists
(
    id_list      TEXT       NOT NULL   PRIMARY KEY,
    name         TEXT       NOT NULL,
    is_owner     BOOLEAN    NOT NULL,
    icon_name    TEXT       DEFAULT 'view-list-symbolic',
    provider     TEXT       NOT NULL
);