table! {
    person (id) {
        id -> Uuid,
        name -> Varchar,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    post (id) {
        id -> Uuid,
        person_id -> Uuid,
        text -> Varchar,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    person,
    post,
);
