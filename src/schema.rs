// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        text -> Text,
        status -> SmallInt,
    }
}
