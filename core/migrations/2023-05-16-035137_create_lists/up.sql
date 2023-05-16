CREATE TABLE lists
(
    id_list      TEXT       NOT NULL   PRIMARY KEY,
    name         TEXT       NOT NULL,
	description  TEXT,
    icon_name    TEXT       DEFAULT 'view-list-symbolic'
)