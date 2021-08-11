table! {
    comments (id) {
        id -> Int4,
        author_id -> Int4,
        timestamp -> Timestamp,
        contents -> Varchar,
        thread_position -> Int4,
        thread_id -> Int4,
    }
}

table! {
    namespaces (name) {
        name -> Varchar,
        owner_id -> Int4,
    }
}

table! {
    namespaces_users (user_id, namespace_name) {
        user_id -> Int4,
        namespace_name -> Varchar,
    }
}

table! {
    threads (id) {
        id -> Int4,
        title -> Varchar,
        last_posted -> Timestamp,
        namespace_name -> Varchar,
    }
}

table! {
    threads_users (author_id, thread_id) {
        author_id -> Int4,
        thread_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        created_on -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}

joinable!(comments -> threads (thread_id));
joinable!(comments -> users (author_id));
joinable!(namespaces -> users (owner_id));
joinable!(namespaces_users -> namespaces (namespace_name));
joinable!(namespaces_users -> users (user_id));
joinable!(threads -> namespaces (namespace_name));
joinable!(threads_users -> threads (thread_id));
joinable!(threads_users -> users (author_id));

allow_tables_to_appear_in_same_query!(
    comments,
    namespaces,
    namespaces_users,
    threads,
    threads_users,
    users,
);
