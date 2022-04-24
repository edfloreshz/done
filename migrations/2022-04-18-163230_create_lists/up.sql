CREATE TABLE lists
(
    id_list      TEXT PRIMARY KEY NOT NULL,
    display_name TEXT             NOT NULL,
    is_owner     BOOLEAN          NOT NULL,
    count        INTEGER          NOT NULL,
    icon_name    TEXT             NOT NULL
)