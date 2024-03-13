// @generated automatically by Diesel CLI.

diesel::table! {
		lists (id_list) {
				id_list -> Text,
				name -> Text,
				description -> Text,
				icon_name -> Nullable<Text>,
		}
}

diesel::table! {
		tasks (id_task) {
				id_task -> Text,
				parent -> Text,
				title -> Text,
				favorite -> Bool,
				today -> Bool,
				status -> Integer,
				priority -> Integer,
				sub_tasks -> Text,
				tags -> Text,
				notes -> Text,
				completion_date -> Nullable<Timestamp>,
				deletion_date -> Nullable<Timestamp>,
				due_date -> Nullable<Timestamp>,
				reminder_date -> Nullable<Timestamp>,
				recurrence -> Text,
				created_date_time -> Timestamp,
				last_modified_date_time -> Timestamp,
		}
}

diesel::allow_tables_to_appear_in_same_query!(lists, tasks,);
