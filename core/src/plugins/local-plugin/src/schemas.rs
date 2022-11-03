// @generated automatically by Diesel CLI.

diesel::table! {
    lists (id_list) {
        id_list -> Text,
        name -> Text,
        is_owner -> Bool,
        icon_name -> Nullable<Text>,
        provider -> Text,
    }
}

diesel::table! {
    tasks (id_task) {
        id_task -> Text,
        parent_list -> Text,
        title -> Text,
        body -> Nullable<Text>,
        importance -> Integer,
        favorite -> Bool,
        is_reminder_on -> Bool,
        status -> Integer,
        completed_on -> Nullable<Timestamp>,
        due_date -> Nullable<Timestamp>,
        reminder_date -> Nullable<Timestamp>,
        created_date_time -> Timestamp,
        last_modified_date_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    lists,
    tasks,
);
