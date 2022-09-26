// @generated automatically by Diesel CLI.

diesel::table! {
    marks (id) {
        id -> Int4,
        user_id -> Int4,
        session_id -> Int4,
        mark_time -> Timestamp,
        is_manual -> Bool,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        title -> Text,
        owner_id -> Int4,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        name -> Nullable<Text>,
    }
}

diesel::joinable!(marks -> sessions (session_id));
diesel::joinable!(marks -> users (user_id));
diesel::joinable!(sessions -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    marks,
    sessions,
    users,
);
