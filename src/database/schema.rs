// @generated automatically by Diesel CLI.

diesel::table! {
    last_list_message (id) {
        id -> Integer,
        message_id -> Integer,
        tg_user_id -> Integer,
    }
}

diesel::table! {
    todos (id) {
        id -> Integer,
        text -> Text,
        status -> SmallInt,
        tg_user_id -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    last_list_message,
    todos,
);
