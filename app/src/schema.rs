table! {
    tasks (task_id) {
        task_id -> Int4,
        user_id -> Int4,
        content -> Varchar,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
        dead_line -> Timestamp,
        finished_flag -> Bool,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        e_mail -> Varchar,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
        delete_flag -> Bool,
    }
}

joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
