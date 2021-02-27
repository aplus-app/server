table! {
    comment (id) {
        id -> Int4,
        post_id -> Varchar,
        body -> Varchar,
        hearts -> Int4,
    }
}

table! {
    post (id) {
        id -> Int4,
        category -> Varchar,
        user_name -> Varchar,
        user_id -> Varchar,
        title -> Varchar,
        body -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(comment, post,);
