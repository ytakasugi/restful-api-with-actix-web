table! {
    tasks (task_id) {
        task_id -> Int4,
        content -> Varchar,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
        dead_line -> Timestamp,
        finished_flag -> Bool,
    }
}
