table! {
    lists (id_list) {
        id_list -> Text,
        display_name -> Text,
        is_owner -> Bool,
        count -> Integer,
        icon_name -> Text,
    }
}

table! {
    tasks (id_task) {
        id_task -> Text,
        id_list -> Text,
        title -> Text,
        body -> Text,
        completed_on -> Nullable<Text>,
        due_date -> Nullable<Text>,
        importance -> Text,
        is_reminder_on -> Bool,
        reminder_date -> Nullable<Text>,
        status -> Text,
        created_date_time -> Text,
        last_modified_date_time -> Text,
    }
}

joinable!(tasks -> lists (id_list));

allow_tables_to_appear_in_same_query!(
    lists,
    tasks,
);
