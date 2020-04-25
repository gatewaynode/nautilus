table! {
    links (id) {
        id -> Int4,
        published -> Bool,
        text -> Varchar,
        title -> Varchar,
        url -> Varchar,
        tags -> Varchar,
        time -> Timestamptz,
    }
}

table! {
    posts (id) {
        id -> Int4,
        published -> Bool,
        title -> Varchar,
        body -> Text,
        time -> Timestamptz,
        tags -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    links,
    posts,
);
